// AHC069 - v3: v2's bid-price admission control + placement quality.
//
// v2 dropped each group at the first anchor where its most compact shape fit. The
// congested seeds showed that their bottleneck is geometric fragmentation, not
// pricing, so v3 keeps the admission rule and the shape set unchanged and instead
// chooses *where* the region goes:
//
//   1. lexicographic on compactness - only the most compact shape class that fits
//      anywhere is considered, so the fee is never traded away for packing;
//   2. best-fit contact - prefer anchors whose boundary hugs walls, ponds and other
//      groups. The boundary is merged into O(1) maximal segments per shape, so one
//      anchor costs a handful of row/column prefix-sum queries;
//   3. departure-time clustering - among the best candidates, prefer neighbours that
//      leave at a similar time, so space is released in contiguous blocks;
//   4. anti-fragmentation - penalize placements that strand free cells in components
//      smaller than the smallest possible group (P >= 4);
//   5. fallback - when no quasi-square fits at all, grow a connected region greedily
//      and take it only if its (lower) compactness still clears the bid price.
//
// A wall-clock guard degrades to v2's first-fit if a run ever gets close to the limit.

use std::collections::{BTreeMap, BinaryHeap};
use std::io::{self, BufWriter, Write};
use std::time::Instant;

const N: usize = 50;

// ---- admission knobs (tuned in v2) ------------------------------------------
const ETA: f64 = 0.20; // effective usable fraction of lawn capacity for pricing
const U_TARGET: f64 = 0.60; // utilization at which the feedback factor is 1
const FEEDBACK_K: f64 = 2.0; // exponent of the occupancy feedback
const WARMUP: usize = 30; // accept-if-fits for the first arrivals
const F_MIN: f64 = 0.05; // never price above the (1-F_MIN)-quantile
const G_CLAMP: (f64, f64) = (0.25, 4.0); // clamp of the feedback factor
const U_FREE: f64 = 0.30; // "park is nearly empty" guard threshold
const FREE_DUR: i64 = 2500; // guard: short duration relative to the horizon

// ---- placement knobs (new in v3) --------------------------------------------
// Tuned on seeds 0..19. The ranking pond > wall > group is the whole story of v3:
// hugging a pond fills the awkward nooks that nothing else can use; the park border
// is a long straight edge that stays usable, so it is worth half as much; and hugging
// another *group* is actively harmful (+4.5 % when its weight is dropped to 0) because
// interleaving with regions that expire at different times is what fragments the park.
const W_WALL: f64 = 0.5; // boundary edge against the park border
const W_POND: f64 = 1.0; // boundary edge against a pond
const W_OCC: f64 = 0.0; // boundary edge against another group
const W_TIME: f64 = 1.0; // departure-time affinity with touching groups
const TAU: f64 = 3000.0; // time scale of that affinity
const W_DEAD: f64 = 2.0; // penalty per free cell stranded in a < 4 cell pocket
const TOPK: usize = 16; // candidates that get the expensive refinement
const FALLBACK: f64 = 1.0; // > 0 enables the grown-region fallback
const FALLBACK_SEEDS: usize = 8; // growth attempts per fallback
const TIME_LIMIT_MS: u128 = 1500; // beyond this, degrade to first-fit

fn knob(name: &str, default: f64) -> f64 {
    std::env::var(name).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

// ---------------------------------------------------------------- input helper

struct Scanner {
    stdin: io::Stdin,
    buf: Vec<String>,
    pos: usize,
}

impl Scanner {
    fn new() -> Self {
        Scanner { stdin: io::stdin(), buf: Vec::new(), pos: 0 }
    }
    /// Reads whitespace separated tokens, refilling one line at a time so that the
    /// interactive protocol is never blocked waiting for input we have not asked for.
    fn token(&mut self) -> String {
        while self.pos >= self.buf.len() {
            let mut line = String::new();
            if self.stdin.read_line(&mut line).unwrap_or(0) == 0 {
                std::process::exit(0);
            }
            self.buf = line.split_whitespace().map(|s| s.to_string()).collect();
            self.pos = 0;
        }
        self.pos += 1;
        self.buf[self.pos - 1].clone()
    }
    fn read<T: std::str::FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        self.token().parse::<T>().unwrap()
    }
}

// ---------------------------------------------------------------- shapes

/// A shape is the union of at most two axis-aligned rectangles inside a bh x bw box.
/// `hsegs` / `vsegs` are its boundary merged into maximal segments: `(d, a, b)` means
/// the neighbouring cells are row `r + d`, columns `[c + a, c + b)` (hseg) or column
/// `c + d`, rows `[r + a, r + b)` (vseg). Their total length equals `perim`.
#[derive(Clone)]
struct Shape {
    bh: usize,
    bw: usize,
    rects: Vec<(usize, usize, usize, usize)>,
    cells: Vec<(usize, usize)>,
    hsegs: Vec<(isize, usize, usize)>,
    vsegs: Vec<(isize, usize, usize)>,
    perim: usize,
}

impl Shape {
    fn place(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        self.cells.iter().map(|&(dr, dc)| (r + dr, c + dc)).collect()
    }
}

fn cells_of(rects: &[(usize, usize, usize, usize)]) -> Vec<(usize, usize)> {
    let mut v = Vec::new();
    for &(r0, c0, r1, c1) in rects {
        for rr in r0..r1 {
            for cc in c0..c1 {
                v.push((rr, cc));
            }
        }
    }
    v
}

/// Perimeter of a cell set: 4 per cell minus 2 per orthogonally adjacent pair.
fn perimeter(cells: &[(usize, usize)]) -> usize {
    let mut set = std::collections::HashSet::new();
    for &c in cells {
        set.insert(c);
    }
    let mut adj = 0;
    for &(r, c) in cells {
        if set.contains(&(r + 1, c)) {
            adj += 1;
        }
        if set.contains(&(r, c + 1)) {
            adj += 1;
        }
    }
    4 * cells.len() - 2 * adj
}

/// Merges unit boundary edges into maximal runs: `map[d]` holds the positions of the
/// neighbouring cells along that line.
fn merge_runs(map: BTreeMap<isize, Vec<usize>>) -> Vec<(isize, usize, usize)> {
    let mut out = Vec::new();
    for (d, mut xs) in map {
        xs.sort_unstable();
        let mut i = 0;
        while i < xs.len() {
            let start = xs[i];
            let mut end = start;
            while i + 1 < xs.len() && xs[i + 1] == end + 1 {
                i += 1;
                end = xs[i];
            }
            out.push((d, start, end + 1));
            i += 1;
        }
    }
    out
}

fn boundary_segments(cells: &[(usize, usize)]) -> (Vec<(isize, usize, usize)>, Vec<(isize, usize, usize)>) {
    let set: std::collections::HashSet<(usize, usize)> = cells.iter().cloned().collect();
    let mut h: BTreeMap<isize, Vec<usize>> = BTreeMap::new();
    let mut v: BTreeMap<isize, Vec<usize>> = BTreeMap::new();
    for &(r, c) in cells {
        // up / down neighbours live in a different row, same column
        if r == 0 || !set.contains(&(r - 1, c)) {
            h.entry(r as isize - 1).or_default().push(c);
        }
        if !set.contains(&(r + 1, c)) {
            h.entry(r as isize + 1).or_default().push(c);
        }
        // left / right neighbours live in a different column, same row
        if c == 0 || !set.contains(&(r, c - 1)) {
            v.entry(c as isize - 1).or_default().push(r);
        }
        if !set.contains(&(r, c + 1)) {
            v.entry(c as isize + 1).or_default().push(r);
        }
    }
    (merge_runs(h), merge_runs(v))
}

/// All quasi-square shapes for `p` cells, sorted by increasing perimeter
/// (i.e. decreasing compactness). Includes both notch alignments and transposes.
fn gen_shapes(p: usize) -> Vec<Shape> {
    let mut out: Vec<Shape> = Vec::new();
    if p == 0 {
        return out;
    }
    let root = (p as f64).sqrt() as usize;
    let lo = if root >= 2 { root - 1 } else { 1 };
    for w in lo..=root + 2 {
        if w == 0 || w > N {
            continue;
        }
        let h = (p + w - 1) / w; // ceil
        if h > N {
            continue;
        }
        let rem = w * h - p; // cells to remove from one border row
        if rem >= w {
            continue; // would empty the row; a smaller h already covers this case
        }
        // Four variants: partial row at the bottom or the top, flushed left or right.
        for &bottom in &[true, false] {
            for &left in &[true, false] {
                let mut rects: Vec<(usize, usize, usize, usize)> = Vec::new();
                if rem == 0 {
                    rects.push((0, 0, h, w));
                } else {
                    let part_c0 = if left { 0 } else { rem };
                    let part_c1 = part_c0 + (w - rem);
                    if bottom {
                        rects.push((0, 0, h - 1, w));
                        rects.push((h - 1, part_c0, h, part_c1));
                    } else {
                        rects.push((1, 0, h, w));
                        rects.push((0, part_c0, 1, part_c1));
                    }
                }
                // Row-major variant and its transpose.
                for t in 0..2 {
                    let (bh, bw, rs): (usize, usize, Vec<(usize, usize, usize, usize)>) = if t == 0 {
                        (h, w, rects.clone())
                    } else {
                        (w, h, rects.iter().map(|&(a, b, c, d)| (b, a, d, c)).collect())
                    };
                    if bh > N || bw > N {
                        continue;
                    }
                    let cells = cells_of(&rs);
                    let (hsegs, vsegs) = boundary_segments(&cells);
                    let per = perimeter(&cells);
                    out.push(Shape { bh, bw, rects: rs, cells, hsegs, vsegs, perim: per });
                }
                if rem == 0 {
                    break; // no left/right distinction for a full rectangle
                }
            }
            if rem == 0 {
                break; // no top/bottom distinction either
            }
        }
    }
    out.sort_by_key(|s| s.perim);
    out.dedup_by(|a, b| a.bh == b.bh && a.bw == b.bw && a.rects == b.rects);
    out
}

// ---------------------------------------------------------------- board

struct Board {
    blocked: [[u8; N]; N], // 1 = pond or occupied
    pond: [[u8; N]; N],    // static
    occ: [[u8; N]; N],     // dynamic
    owner: [[i32; N]; N],  // group index occupying the cell, -1 if none
    pre: [[u32; N + 1]; N + 1], // 2D prefix sum of `blocked`
    pond_row: [[u16; N + 1]; N],
    pond_col: [[u16; N + 1]; N],
    occ_row: [[u16; N + 1]; N],
    occ_col: [[u16; N + 1]; N],
    dirty: bool,
}

impl Board {
    fn new(grid: &[Vec<u8>]) -> Self {
        let mut b = Board {
            blocked: [[0; N]; N],
            pond: [[0; N]; N],
            occ: [[0; N]; N],
            owner: [[-1; N]; N],
            pre: [[0; N + 1]; N + 1],
            pond_row: [[0; N + 1]; N],
            pond_col: [[0; N + 1]; N],
            occ_row: [[0; N + 1]; N],
            occ_col: [[0; N + 1]; N],
            dirty: true,
        };
        for r in 0..N {
            for c in 0..N {
                b.pond[r][c] = grid[r][c];
                b.blocked[r][c] = grid[r][c];
            }
        }
        for r in 0..N {
            for c in 0..N {
                b.pond_row[r][c + 1] = b.pond_row[r][c] + b.pond[r][c] as u16;
            }
        }
        for c in 0..N {
            for r in 0..N {
                b.pond_col[c][r + 1] = b.pond_col[c][r] + b.pond[r][c] as u16;
            }
        }
        b.rebuild();
        b
    }

    fn rebuild(&mut self) {
        for r in 0..N {
            for c in 0..N {
                self.pre[r + 1][c + 1] =
                    self.pre[r][c + 1] + self.pre[r + 1][c] - self.pre[r][c] + self.blocked[r][c] as u32;
                self.occ_row[r][c + 1] = self.occ_row[r][c] + self.occ[r][c] as u16;
            }
        }
        for c in 0..N {
            for r in 0..N {
                self.occ_col[c][r + 1] = self.occ_col[c][r] + self.occ[r][c] as u16;
            }
        }
        self.dirty = false;
    }

    #[inline]
    fn rect_blocked(&self, r0: usize, c0: usize, r1: usize, c1: usize) -> u32 {
        self.pre[r1][c1] + self.pre[r0][c0] - self.pre[r0][c1] - self.pre[r1][c0]
    }

    fn add(&mut self, cells: &[(usize, usize)], gid: i32) {
        for &(r, c) in cells {
            self.blocked[r][c] = 1;
            self.occ[r][c] = 1;
            self.owner[r][c] = gid;
        }
        self.dirty = true;
    }

    fn remove(&mut self, cells: &[(usize, usize)]) {
        for &(r, c) in cells {
            self.blocked[r][c] = self.pond[r][c];
            self.occ[r][c] = 0;
            self.owner[r][c] = -1;
        }
        self.dirty = true;
    }

    #[inline]
    fn fits(&self, s: &Shape, r: usize, c: usize) -> bool {
        for &(r0, c0, r1, c1) in &s.rects {
            if self.rect_blocked(r + r0, c + c0, r + r1, c + c1) != 0 {
                return false;
            }
        }
        true
    }

    /// First anchor at which `s` fits, scanning row-major (the v2 behaviour, kept as
    /// the degraded path for the wall-clock guard).
    fn find_fit(&self, s: &Shape) -> Option<(usize, usize)> {
        if s.bh > N || s.bw > N {
            return None;
        }
        for r in 0..=N - s.bh {
            for c in 0..=N - s.bw {
                if self.fits(s, r, c) {
                    return Some((r, c));
                }
            }
        }
        None
    }

    /// Boundary edges of `s` at `(r, c)` split by what they touch: (wall, pond, group).
    fn contact(&self, s: &Shape, r: usize, c: usize) -> (usize, usize, usize) {
        let (mut wall, mut pond, mut occ) = (0usize, 0usize, 0usize);
        for &(d, a, b) in &s.hsegs {
            let nr = r as isize + d;
            if nr < 0 || nr >= N as isize {
                wall += b - a;
            } else {
                let nr = nr as usize;
                pond += (self.pond_row[nr][c + b] - self.pond_row[nr][c + a]) as usize;
                occ += (self.occ_row[nr][c + b] - self.occ_row[nr][c + a]) as usize;
            }
        }
        for &(d, a, b) in &s.vsegs {
            let nc = c as isize + d;
            if nc < 0 || nc >= N as isize {
                wall += b - a;
            } else {
                let nc = nc as usize;
                pond += (self.pond_col[nc][r + b] - self.pond_col[nc][r + a]) as usize;
                occ += (self.occ_col[nc][r + b] - self.occ_col[nc][r + a]) as usize;
            }
        }
        (wall, pond, occ)
    }
}

// ---------------------------------------------------------------- refinement

struct Scratch {
    mark: [[u32; N]; N], // cells of the candidate region
    vis: [[u32; N]; N],  // flood-fill visited stamp
    comp: [[u16; N]; N], // which pocket of the current candidate a cell belongs to
    gen: u32,
    stack: Vec<(usize, usize)>,
}

impl Scratch {
    fn new() -> Self {
        Scratch { mark: [[0; N]; N], vis: [[0; N]; N], comp: [[0; N]; N], gen: 0, stack: Vec::new() }
    }
}

#[inline]
fn neighbours(r: usize, c: usize) -> impl Iterator<Item = (usize, usize)> {
    let mut v = [(0usize, 0usize); 4];
    let mut n = 0;
    if r > 0 {
        v[n] = (r - 1, c);
        n += 1;
    }
    if r + 1 < N {
        v[n] = (r + 1, c);
        n += 1;
    }
    if c > 0 {
        v[n] = (r, c - 1);
        n += 1;
    }
    if c + 1 < N {
        v[n] = (r, c + 1);
        n += 1;
    }
    v.into_iter().take(n)
}

/// Departure-time affinity: how well the candidate's neighbours match its own
/// departure time. Space frees up in usable blocks when neighbours leave together.
fn time_affinity(board: &Board, cells: &[(usize, usize)], t: i64, dep: &[i64], tau: f64) -> f64 {
    let mut sum = 0.0;
    for &(r, c) in cells {
        for (nr, nc) in neighbours(r, c) {
            let g = board.owner[nr][nc];
            if g >= 0 {
                let dt = (t - dep[g as usize]).abs() as f64;
                sum += (-dt / tau).exp();
            }
        }
    }
    sum
}

/// Free cells stranded in connected components smaller than the smallest group
/// (P >= 4), once `cells` is occupied. Those cells can never earn anything again
/// until a neighbour departs.
///
/// Only pockets touching the candidate region are examined: a component with no cell
/// adjacent to the region is identical for every candidate, so it shifts all scores by
/// the same constant and cannot change the ranking. Each pocket is abandoned as soon as
/// it is provably >= 4 cells, and a walk that runs into a pocket explored earlier in
/// this candidate inherits its verdict, so truncation never invents a dead pocket.
fn dead_cells(board: &Board, cells: &[(usize, usize)], sc: &mut Scratch) -> usize {
    sc.gen += 1;
    let g = sc.gen;
    for &(r, c) in cells {
        sc.mark[r][c] = g;
    }
    let mut dead = 0;
    let mut pocket: u16 = 0;
    for &(r, c) in cells {
        for (sr, scol) in neighbours(r, c) {
            if board.blocked[sr][scol] != 0 || sc.mark[sr][scol] == g || sc.vis[sr][scol] == g {
                continue;
            }
            pocket += 1;
            sc.vis[sr][scol] = g;
            sc.comp[sr][scol] = pocket;
            sc.stack.clear();
            sc.stack.push((sr, scol));
            let mut size = 0;
            let mut alive = false;
            while let Some((ar, ac)) = sc.stack.pop() {
                size += 1;
                if size >= 4 {
                    alive = true;
                    break;
                }
                for (nr, nc) in neighbours(ar, ac) {
                    if board.blocked[nr][nc] != 0 || sc.mark[nr][nc] == g {
                        continue;
                    }
                    if sc.vis[nr][nc] == g {
                        if sc.comp[nr][nc] != pocket {
                            alive = true; // joins a pocket already judged big enough
                        }
                        continue;
                    }
                    sc.vis[nr][nc] = g;
                    sc.comp[nr][nc] = pocket;
                    sc.stack.push((nr, nc));
                }
            }
            if !alive && size < 4 {
                dead += size;
            }
        }
    }
    dead
}

/// Greedy connected region of `p` free cells grown from `seed`, always taking the
/// frontier cell with the most neighbours already inside (that is the choice which
/// adds the least perimeter). Used only when no quasi-square fits anywhere.
fn grow_region(board: &Board, seed: (usize, usize), p: usize, sc: &mut Scratch) -> Option<Vec<(usize, usize)>> {
    sc.gen += 1;
    let g = sc.gen;
    let mut region = Vec::with_capacity(p);
    let mut heap: BinaryHeap<(u8, usize, usize)> = BinaryHeap::new();
    let inside = |sc: &Scratch, r: usize, c: usize| sc.mark[r][c] == g;

    sc.mark[seed.0][seed.1] = g;
    region.push(seed);
    for (nr, nc) in neighbours(seed.0, seed.1) {
        if board.blocked[nr][nc] == 0 {
            heap.push((1, nr, nc));
        }
    }
    while region.len() < p {
        let (stored, r, c) = heap.pop()?;
        if inside(sc, r, c) {
            continue;
        }
        let cnt = neighbours(r, c).filter(|&(nr, nc)| inside(sc, nr, nc)).count() as u8;
        if cnt != stored {
            heap.push((cnt, r, c)); // stale key, reinsert with the current count
            continue;
        }
        sc.mark[r][c] = g;
        region.push((r, c));
        for (nr, nc) in neighbours(r, c) {
            if board.blocked[nr][nc] == 0 && !inside(sc, nr, nc) {
                let k = neighbours(nr, nc).filter(|&(ar, ac)| inside(sc, ar, ac)).count() as u8;
                heap.push((k, nr, nc));
            }
        }
    }
    if region.len() == p {
        Some(region)
    } else {
        None
    }
}

// ---------------------------------------------------------------- main

struct Active {
    t: i64,
    cells: Vec<(usize, usize)>,
}

fn main() {
    let t_start = Instant::now();
    let mut sc = Scanner::new();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let _n: usize = sc.read();
    let m: usize = sc.read();
    let _r: f64 = sc.read();

    let mut grid = vec![vec![0u8; N]; N];
    for r in 0..N {
        let row = sc.token();
        for (c, ch) in row.bytes().enumerate() {
            grid[r][c] = if ch == b'#' { 1 } else { 0 };
        }
    }

    let mut board = Board::new(&grid);
    let mut scratch = Scratch::new();
    let mut active: Vec<Active> = Vec::new();
    let mut money: i64 = 0; // local score model, printed to stderr for cross-checking

    let a_lawn = grid.iter().flatten().filter(|&&b| b == 0).count() as f64;
    let eta = knob("AHC_ETA", ETA);
    let u_target = knob("AHC_UTARGET", U_TARGET);
    let fb_k = knob("AHC_K", FEEDBACK_K);
    let warmup = knob("AHC_WARMUP", WARMUP as f64) as usize;
    let w_wall = knob("AHC_WWALL", W_WALL);
    let w_pond = knob("AHC_WPOND", W_POND);
    let w_occ = knob("AHC_WOCC", W_OCC);
    let w_time = knob("AHC_WTIME", W_TIME);
    let tau = knob("AHC_TAU", TAU);
    let w_dead = knob("AHC_WDEAD", W_DEAD);
    let topk = knob("AHC_TOPK", TOPK as f64) as usize;
    let use_fallback = knob("AHC_FALLBACK", FALLBACK) > 0.0;

    // Shapes depend only on P, so build them once (P <= 150).
    let shape_table: Vec<Vec<Shape>> = (0..=150).map(gen_shapes).collect();

    let mut densities: Vec<(f64, f64)> = Vec::with_capacity(m); // (d_j, P_j*dur_j)
    let mut sum_demand: f64 = 0.0;
    let mut occupied: usize = 0;
    let mut accepted = 0usize;
    let mut fallbacks = 0usize;
    let mut dep: Vec<i64> = vec![0; m];
    let mut cands: Vec<(f64, usize, usize, usize)> = Vec::new();

    for idx in 0..m {
        let _i: usize = sc.read();
        let s: i64 = sc.read();
        let t: i64 = sc.read();
        let p: usize = sc.read();
        let v: f64 = sc.read();
        dep[idx] = t;

        // 1. release the regions of groups that have already departed. The fee is
        // booked at placement time: v3 never moves a group, so its compactness can
        // never change after it is placed.
        let mut kept = Vec::with_capacity(active.len());
        for g in active.drain(..) {
            if g.t < s {
                occupied -= g.cells.len();
                board.remove(&g.cells);
            } else {
                kept.push(g);
            }
        }
        active = kept;
        if board.dirty {
            board.rebuild();
        }

        // 2. no moves in v3
        writeln!(out, "0").unwrap();

        // 3a. record the arrival's value density (using the best achievable C for its
        // size, so densities are comparable across group sizes)
        let dur = (t - s) as f64;
        let shapes = &shape_table[p];
        let c_best = 4.0 * (p as f64).sqrt() / shapes[0].perim as f64;
        let area = p as f64 * dur;
        let density = v * c_best / area;
        // kept sorted by decreasing density so the quantile below is a plain walk
        let at = densities.partition_point(|&(d, _)| d > density);
        densities.insert(at, (density, area));
        sum_demand += area;

        // 3b. bid price lambda = density-quantile(1 - f) * occupancy feedback
        let u = occupied as f64 / a_lawn;
        let lambda = if idx < warmup {
            0.0 // thin sample: fill the park and gather data
        } else {
            let demand_rate = sum_demand / s.max(1) as f64;
            let f = (a_lawn * eta / demand_rate).clamp(F_MIN, 1.0);
            // demand-weighted quantile (the LP marginal price): walk the densities in
            // decreasing order and stop once fraction f of the total cell*time demand
            // is covered - exactly how the offline LP bound prices capacity.
            let mut acc = 0.0;
            let mut q = 0.0;
            for &(d, a) in densities.iter() {
                acc += a;
                q = d;
                if acc >= f * sum_demand {
                    break;
                }
            }
            let g = (u / u_target).powf(fb_k).clamp(G_CLAMP.0, G_CLAMP.1);
            q * g
        };

        // 3c. choose the placement. Compactness comes first: only the most compact
        // shape class that fits anywhere is considered, and within it the packing
        // score decides.
        let in_budget = t_start.elapsed().as_millis() < TIME_LIMIT_MS;
        let mut chosen: Option<(Vec<(usize, usize)>, usize)> = None; // (cells, perimeter)

        if in_budget {
            let mut i0 = 0;
            while i0 < shapes.len() {
                let per = shapes[i0].perim;
                let mut i1 = i0;
                while i1 < shapes.len() && shapes[i1].perim == per {
                    i1 += 1;
                }
                cands.clear();
                for si in i0..i1 {
                    let sh = &shapes[si];
                    if sh.bh > N || sh.bw > N {
                        continue;
                    }
                    for r in 0..=N - sh.bh {
                        for c in 0..=N - sh.bw {
                            if !board.fits(sh, r, c) {
                                continue;
                            }
                            let (wall, pond, occ) = board.contact(sh, r, c);
                            let cheap =
                                w_wall * wall as f64 + w_pond * pond as f64 + w_occ * occ as f64;
                            cands.push((cheap, si, r, c));
                        }
                    }
                }
                if !cands.is_empty() {
                    // keep the most promising anchors, then re-rank them with the
                    // expensive terms (time clustering, stranded free cells)
                    if cands.len() > topk {
                        // linear selection; the anchor tie-break keeps the chosen set
                        // deterministic even though the partition order is not
                        cands.select_nth_unstable_by(topk, |a, b| {
                            b.0.partial_cmp(&a.0).unwrap().then((a.1, a.2, a.3).cmp(&(b.1, b.2, b.3)))
                        });
                        cands.truncate(topk);
                    }
                    let mut best: Option<(f64, usize, usize, usize)> = None;
                    for &(cheap, si, r, c) in cands.iter() {
                        let cells = shapes[si].place(r, c);
                        let aff = time_affinity(&board, &cells, t, &dep, tau);
                        let dead = dead_cells(&board, &cells, &mut scratch) as f64;
                        let score = cheap + w_time * aff - w_dead * dead;
                        // explicit anchor tie-break: equal scores are common once the
                        // group-contact weight is 0, and the winner must not depend on
                        // the order the candidates happen to arrive in
                        let better = match best {
                            None => true,
                            Some((bs, bsi, br, bc)) => {
                                score > bs || (score == bs && (si, r, c) < (bsi, br, bc))
                            }
                        };
                        if better {
                            best = Some((score, si, r, c));
                        }
                    }
                    let (_, si, r, c) = best.unwrap();
                    chosen = Some((shapes[si].place(r, c), shapes[si].perim));
                    break;
                }
                i0 = i1;
            }
        } else {
            // wall-clock guard: v2's first-fit
            for sh in shapes.iter() {
                if let Some((r, c)) = board.find_fit(sh) {
                    chosen = Some((sh.place(r, c), sh.perim));
                    break;
                }
            }
        }

        // 3d. no quasi-square fits: grow a connected region instead. Its compactness
        // is lower, so the admission check below decides whether it is worth it.
        if chosen.is_none() && use_fallback && in_budget {
            let mut seeds: Vec<(usize, usize, usize)> = Vec::new(); // (contact, r, c)
            for r in 0..N {
                for c in 0..N {
                    if board.blocked[r][c] != 0 {
                        continue;
                    }
                    let mut k = 0;
                    if r == 0 || board.blocked[r - 1][c] != 0 {
                        k += 1;
                    }
                    if r + 1 == N || board.blocked[r + 1][c] != 0 {
                        k += 1;
                    }
                    if c == 0 || board.blocked[r][c - 1] != 0 {
                        k += 1;
                    }
                    if c + 1 == N || board.blocked[r][c + 1] != 0 {
                        k += 1;
                    }
                    seeds.push((k, r, c));
                }
            }
            seeds.sort_unstable_by(|a, b| b.cmp(a));
            let mut best: Option<(usize, Vec<(usize, usize)>)> = None;
            let mut tried = 0;
            let mut used: Vec<(usize, usize)> = Vec::new();
            for &(_, r, c) in &seeds {
                if tried >= FALLBACK_SEEDS {
                    break;
                }
                // spread the attempts out
                if used.iter().any(|&(ur, uc)| ur.abs_diff(r) < 4 && uc.abs_diff(c) < 4) {
                    continue;
                }
                used.push((r, c));
                tried += 1;
                if let Some(region) = grow_region(&board, (r, c), p, &mut scratch) {
                    let per = perimeter(&region);
                    if best.is_none() || per < best.as_ref().unwrap().0 {
                        best = Some((per, region));
                    }
                }
            }
            if let Some((per, region)) = best {
                fallbacks += 1;
                chosen = Some((region, per));
            }
        }

        // 3e. admission decision: the region must pay for the space*time it takes
        if let Some((_, per)) = chosen.as_ref() {
            let c_fit = 4.0 * (p as f64).sqrt() / *per as f64;
            let worth = v * c_fit >= lambda * area;
            // guard: an almost-empty park should not refuse short stays
            let free_pass = u < U_FREE && (t - s) <= FREE_DUR;
            if !(worth || free_pass) {
                chosen = None;
            }
        }

        match chosen {
            Some((cells, per)) => {
                debug_assert_eq!(cells.len(), p);
                accepted += 1;
                occupied += p;
                let comp = 4.0 * (p as f64).sqrt() / per as f64;
                money += (v * comp).round() as i64;
                board.add(&cells, idx as i32);
                board.rebuild();
                writeln!(out, "Yes").unwrap();
                for (rr, cc) in cells.iter() {
                    writeln!(out, "{} {}", rr, cc).unwrap();
                }
                active.push(Active { t, cells });
            }
            None => {
                writeln!(out, "No").unwrap();
            }
        }
        out.flush().unwrap();
    }

    eprintln!(
        "local_score = {} accepted = {}/{} fallbacks = {} elapsed = {} ms",
        money,
        accepted,
        m,
        fallbacks,
        t_start.elapsed().as_millis()
    );
}
