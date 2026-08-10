use std::collections::{BTreeMap, BinaryHeap};
use std::io::{self, BufWriter, Write};
use std::time::Instant;

const N: usize = 50;

const HORIZON: f64 = 100000.0;
const ETA: f64 = 0.65;
const U_TARGET: f64 = 0.60;
const FEEDBACK_K: f64 = 0.75;
const WARMUP: usize = 30;
const F_MIN: f64 = 0.05;
const G_CLAMP: (f64, f64) = (0.25, 4.0);
const U_FREE: f64 = 0.30;
const FREE_DUR: i64 = 2500;

const W_WALL: f64 = 0.5;
const W_POND: f64 = 1.0;
const W_OCC: f64 = 0.0;
const W_TIME: f64 = 1.0;
const TAU: f64 = 3000.0;
const W_DEAD: f64 = 2.0;
const TOPK: usize = 16;
const FALLBACK: f64 = 1.0;
const FALLBACK_SEEDS: usize = 8;
const TIME_LIMIT_MS: u128 = 1500;

const MOVES: f64 = 1.0;
const MAX_DISPLACE: usize = 3;
const MOVE_TOPK: usize = 3;
const MOVE_MARGIN: f64 = 1.0;
const MOVE_TIME_MS: u128 = 1200;

const RESEAT_TOPK: usize = 1;

fn knob(name: &str, default: f64) -> f64 {
    std::env::var(name).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

struct Scanner {
    stdin: io::Stdin,
    buf: Vec<String>,
    pos: usize,
}

impl Scanner {
    fn new() -> Self {
        Scanner { stdin: io::stdin(), buf: Vec::new(), pos: 0 }
    }

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
        if r == 0 || !set.contains(&(r - 1, c)) {
            h.entry(r as isize - 1).or_default().push(c);
        }
        if !set.contains(&(r + 1, c)) {
            h.entry(r as isize + 1).or_default().push(c);
        }
        if c == 0 || !set.contains(&(r, c - 1)) {
            v.entry(c as isize - 1).or_default().push(r);
        }
        if !set.contains(&(r, c + 1)) {
            v.entry(c as isize + 1).or_default().push(r);
        }
    }
    (merge_runs(h), merge_runs(v))
}

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
        let h = (p + w - 1) / w;
        if h > N {
            continue;
        }
        let rem = w * h - p;
        if rem >= w {
            continue;
        }

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
                    break;
                }
            }
            if rem == 0 {
                break;
            }
        }
    }
    out.sort_by_key(|s| s.perim);
    out.dedup_by(|a, b| a.bh == b.bh && a.bw == b.bw && a.rects == b.rects);
    out
}

struct Board {
    blocked: [[u8; N]; N],
    pond: [[u8; N]; N],
    occ: [[u8; N]; N],
    owner: [[i32; N]; N],
    pre: [[u32; N + 1]; N + 1],
    pond_pre: [[u32; N + 1]; N + 1],
    cost: [[f64; N]; N],
    cost_pre: [[f64; N + 1]; N + 1],
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
            pond_pre: [[0; N + 1]; N + 1],
            cost: [[0.0; N]; N],
            cost_pre: [[0.0; N + 1]; N + 1],
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
        for r in 0..N {
            for c in 0..N {
                b.pond_pre[r + 1][c + 1] = b.pond_pre[r][c + 1] + b.pond_pre[r + 1][c] - b.pond_pre[r][c]
                    + b.pond[r][c] as u32;
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
                self.cost_pre[r + 1][c + 1] = self.cost_pre[r][c + 1] + self.cost_pre[r + 1][c]
                    - self.cost_pre[r][c]
                    + self.cost[r][c];
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

    fn add(&mut self, cells: &[(usize, usize)], gid: i32, share: f64) {
        for &(r, c) in cells {
            self.blocked[r][c] = 1;
            self.occ[r][c] = 1;
            self.owner[r][c] = gid;
            self.cost[r][c] = share;
        }
        self.dirty = true;
    }

    fn remove(&mut self, cells: &[(usize, usize)]) {
        for &(r, c) in cells {
            self.blocked[r][c] = self.pond[r][c];
            self.occ[r][c] = 0;
            self.owner[r][c] = -1;
            self.cost[r][c] = 0.0;
        }
        self.dirty = true;
    }

    #[inline]
    fn rect_pond(&self, r0: usize, c0: usize, r1: usize, c1: usize) -> u32 {
        self.pond_pre[r1][c1] + self.pond_pre[r0][c0] - self.pond_pre[r0][c1] - self.pond_pre[r1][c0]
    }

    #[inline]
    fn rect_cost(&self, r0: usize, c0: usize, r1: usize, c1: usize) -> f64 {
        self.cost_pre[r1][c1] + self.cost_pre[r0][c0] - self.cost_pre[r0][c1] - self.cost_pre[r1][c0]
    }

    fn shape_pond_cost(&self, s: &Shape, r: usize, c: usize) -> (u32, f64) {
        let (mut pond, mut cost) = (0u32, 0.0);
        for &(r0, c0, r1, c1) in &s.rects {
            pond += self.rect_pond(r + r0, c + c0, r + r1, c + c1);
            cost += self.rect_cost(r + r0, c + c0, r + r1, c + c1);
        }
        (pond, cost)
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

struct Scratch {
    mark: [[u32; N]; N],
    vis: [[u32; N]; N],
    comp: [[u16; N]; N],
    generator: u32,
    stack: Vec<(usize, usize)>,
}

impl Scratch {
    fn new() -> Self {
        Scratch { mark: [[0; N]; N], vis: [[0; N]; N], comp: [[0; N]; N], generator: 0, stack: Vec::new() }
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

fn dead_cells(board: &Board, cells: &[(usize, usize)], sc: &mut Scratch) -> usize {
    sc.generator += 1;
    let g = sc.generator;
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
                            alive = true;
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

fn grow_region(board: &Board, seed: (usize, usize), p: usize, sc: &mut Scratch) -> Option<Vec<(usize, usize)>> {
    sc.generator += 1;
    let g = sc.generator;
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
            heap.push((cnt, r, c));
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

struct Active {
    t: i64,
    p: usize,
    perim: usize,
    move_cost: i64,
    cells: Vec<(usize, usize)>,
}

struct Weights {
    wall: f64,
    pond: f64,
    occ: f64,
    time: f64,
    tau: f64,
    dead: f64,
}

fn scored_placement(
    board: &Board,
    shapes: &[Shape],
    max_perim: usize,
    t_dep: i64,
    dep: &[i64],
    sc: &mut Scratch,
    w: &Weights,
    topk: usize,
) -> Option<(usize, usize, usize)> {
    let mut cands: Vec<(f64, usize, usize, usize)> = Vec::new();
    let mut i0 = 0;
    while i0 < shapes.len() && shapes[i0].perim <= max_perim {
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
                    cands.push((
                        w.wall * wall as f64 + w.pond * pond as f64 + w.occ * occ as f64,
                        si,
                        r,
                        c,
                    ));
                }
            }
        }
        if !cands.is_empty() {
            if cands.len() > topk {
                cands.select_nth_unstable_by(topk, |a, b| {
                    b.0.partial_cmp(&a.0).unwrap().then((a.1, a.2, a.3).cmp(&(b.1, b.2, b.3)))
                });
                cands.truncate(topk);
            }
            let mut best: Option<(f64, usize, usize, usize)> = None;
            for &(cheap, si, r, c) in cands.iter() {
                let cells = shapes[si].place(r, c);
                let aff = time_affinity(board, &cells, t_dep, dep, w.tau);
                let dead = dead_cells(board, &cells, sc) as f64;
                let score = cheap + w.time * aff - w.dead * dead;
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
            return Some((si, r, c));
        }
        i0 = i1;
    }
    None
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

    let mut groups: Vec<Option<Active>> = (0..m).map(|_| None).collect();
    let mut live: Vec<usize> = Vec::new();
    let mut money: i64 = 0;

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
    let use_moves = knob("AHC_MOVES", MOVES) > 0.0;
    let max_displace = knob("AHC_MAXDISP", MAX_DISPLACE as f64) as usize;
    let move_topk = knob("AHC_MOVETOPK", MOVE_TOPK as f64) as usize;
    let move_margin = knob("AHC_MOVEMARGIN", MOVE_MARGIN);
    let reseat_topk = knob("AHC_RESEATTOPK", RESEAT_TOPK as f64) as usize;
    let weights = Weights { wall: w_wall, pond: w_pond, occ: w_occ, time: w_time, tau, dead: w_dead };

    let r_milli = (_r * 1000.0).round() as i64;
    let move_cost_of = |vj: i64| (((vj * r_milli + 500) / 1000) as i64).max(1);

    let shape_table: Vec<Vec<Shape>> = (0..=150).map(gen_shapes).collect();

    let mut densities: Vec<(f64, f64)> = Vec::with_capacity(m);
    let mut obs: Vec<(f64, f64)> = Vec::with_capacity(m);
    let mut sum_demand: f64 = 0.0;
    let mut occupied: usize = 0;
    let mut accepted = 0usize;
    let mut fallbacks = 0usize;
    let mut relocations = 0usize;
    let mut moved_groups = 0usize;
    let mut move_spend: i64 = 0;
    let mut dep: Vec<i64> = vec![0; m];
    let mut cands: Vec<(f64, usize, usize, usize)> = Vec::new();
    let mut targets: Vec<(f64, f64, usize, usize, usize)> = Vec::new();

    for idx in 0..m {
        let _i: usize = sc.read();
        let s: i64 = sc.read();
        let t: i64 = sc.read();
        let p: usize = sc.read();
        let v_int: i64 = sc.read();
        let v = v_int as f64;
        dep[idx] = t;

        let mut kept = Vec::with_capacity(live.len());
        for id in live.drain(..) {
            let g = groups[id].as_ref().unwrap();
            if g.t < s {
                occupied -= g.cells.len();
                let cells = std::mem::take(&mut groups[id].as_mut().unwrap().cells);
                board.remove(&cells);
                groups[id] = None;
            } else {
                kept.push(id);
            }
        }
        live = kept;
        if board.dirty {
            board.rebuild();
        }

        let dur = (t - s) as f64;
        let shapes = &shape_table[p];
        let c_best = 4.0 * (p as f64).sqrt() / shapes[0].perim as f64;
        let area = p as f64 * dur;
        let density = v * c_best / area;

        let at = densities.partition_point(|&(d, _)| d > density);
        densities.insert(at, (density, area));
        sum_demand += area;
        obs.push((p as f64, dur));

        let u = occupied as f64 / a_lawn;
        let lambda = if idx < warmup {
            0.0
        } else {
            let horizon_left = (HORIZON - s as f64).max(1.0);
            let mut committed = 0.0;
            for &id in live.iter() {
                let g = groups[id].as_ref().unwrap();
                committed += g.p as f64 * (g.t - s) as f64;
            }
            let mut exp_area = 0.0;
            for &(pj, dj) in obs.iter() {
                exp_area += pj * dj.min(horizon_left);
            }
            exp_area /= obs.len() as f64;
            let future_demand = (m - idx - 1) as f64 * exp_area;
            let available = (a_lawn * horizon_left - committed).max(0.0) * eta;
            let f = if future_demand > 0.0 {
                (available / future_demand).clamp(F_MIN, 1.0)
            } else {
                1.0
            };
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

        let in_budget = t_start.elapsed().as_millis() < TIME_LIMIT_MS;
        let mut chosen: Option<(Vec<(usize, usize)>, usize)> = None;

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
                    if cands.len() > topk {
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
            for sh in shapes.iter() {
                if let Some((r, c)) = board.find_fit(sh) {
                    chosen = Some((sh.place(r, c), sh.perim));
                    break;
                }
            }
        }

        if chosen.is_none() && use_fallback && in_budget {
            let mut seeds: Vec<(usize, usize, usize)> = Vec::new();
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

        let base_value = match chosen.as_ref() {
            Some((_, per)) => v * 4.0 * (p as f64).sqrt() / *per as f64,
            None => 0.0,
        };
        let headroom = v * c_best - base_value;
        let mut plan: Option<(Vec<(usize, usize)>, Vec<(usize, Vec<(usize, usize)>)>, i64)> = None;

        if use_moves
            && headroom > move_margin
            && !live.is_empty()
            && t_start.elapsed().as_millis() < MOVE_TIME_MS
        {
            targets.clear();
            let per0 = shapes[0].perim;
            for (si, sh) in shapes.iter().enumerate() {
                if sh.perim != per0 {
                    break;
                }
                if sh.bh > N || sh.bw > N {
                    continue;
                }
                for r in 0..=N - sh.bh {
                    for c in 0..=N - sh.bw {
                        let (pond, est) = board.shape_pond_cost(sh, r, c);
                        if pond != 0 || est <= 0.0 || est >= headroom {
                            continue;
                        }
                        let (wall, pd, _) = board.contact(sh, r, c);
                        targets.push((est, -(w_wall * wall as f64 + w_pond * pd as f64), si, r, c));
                    }
                }
            }
            if targets.len() > move_topk {
                targets.select_nth_unstable_by(move_topk, |a, b| {
                    (a.0, a.1, a.2, a.3, a.4).partial_cmp(&(b.0, b.1, b.2, b.3, b.4)).unwrap()
                });
                targets.truncate(move_topk);
            }
            targets.sort_unstable_by(|a, b| {
                (a.0, a.1, a.2, a.3, a.4).partial_cmp(&(b.0, b.1, b.2, b.3, b.4)).unwrap()
            });

            for &(_, _, si, r, c) in targets.iter() {
                let region = shapes[si].place(r, c);

                let mut ids: Vec<usize> = Vec::new();
                for &(rr, cc) in &region {
                    let o = board.owner[rr][cc];
                    if o >= 0 && !ids.contains(&(o as usize)) {
                        ids.push(o as usize);
                    }
                }
                if ids.is_empty() || ids.len() > max_displace {
                    continue;
                }
                let cost: i64 = ids.iter().map(|&id| groups[id].as_ref().unwrap().move_cost).sum();
                if v * c_best - cost as f64 <= base_value + move_margin {
                    continue;
                }

                let old: Vec<Vec<(usize, usize)>> =
                    ids.iter().map(|&id| groups[id].as_ref().unwrap().cells.clone()).collect();
                for cells in &old {
                    board.remove(cells);
                }
                board.add(&region, idx as i32, 0.0);
                board.rebuild();

                let mut order: Vec<usize> = ids.clone();
                order.sort_unstable_by_key(|&id| std::cmp::Reverse(groups[id].as_ref().unwrap().p));
                let mut placed: Vec<(usize, Vec<(usize, usize)>)> = Vec::new();
                let mut ok = true;
                for (k, &id) in order.iter().enumerate() {
                    let g = groups[id].as_ref().unwrap();
                    match scored_placement(
                        &board,
                        &shape_table[g.p],
                        g.perim,
                        g.t,
                        &dep,
                        &mut scratch,
                        &weights,
                        reseat_topk,
                    ) {
                        Some((gsi, gr, gc)) => {
                            let cells = shape_table[g.p][gsi].place(gr, gc);
                            board.add(&cells, id as i32, g.move_cost as f64 / g.p as f64);
                            if k + 1 < order.len() {
                                board.rebuild();
                            }
                            placed.push((id, cells));
                        }
                        None => {
                            ok = false;
                            break;
                        }
                    }
                }

                for (_, cells) in &placed {
                    board.remove(cells);
                }
                board.remove(&region);
                for (k, &id) in ids.iter().enumerate() {
                    let g = groups[id].as_ref().unwrap();
                    board.add(&old[k], id as i32, g.move_cost as f64 / g.p as f64);
                }
                board.rebuild();

                if ok {
                    plan = Some((region, placed, cost));
                    break;
                }
            }
        }

        let plan_net = plan.as_ref().map(|(_, _, cost)| v * c_best - *cost as f64);
        let use_plan = matches!(plan_net, Some(net) if net > base_value + move_margin);
        let net_value = if use_plan { plan_net.unwrap() } else { base_value };
        let free_pass = u < U_FREE && (t - s) <= FREE_DUR;
        let admit = net_value > 0.0 && (net_value >= lambda * area || free_pass);

        let (moves, final_cells, final_perim) = if admit && use_plan {
            let (region, placed, cost) = plan.unwrap();
            relocations += 1;
            moved_groups += placed.len();
            move_spend += cost;
            money -= cost;
            (placed, Some(region), shapes[0].perim)
        } else if admit {
            match chosen {
                Some((cells, per)) => (Vec::new(), Some(cells), per),
                None => (Vec::new(), None, 0),
            }
        } else {
            (Vec::new(), None, 0)
        };

        writeln!(out, "{}", moves.len()).unwrap();
        for (id, cells) in &moves {
            writeln!(out, "{}", id).unwrap();
            for &(rr, cc) in cells {
                writeln!(out, "{} {}", rr, cc).unwrap();
            }
        }

        for (id, _) in &moves {
            let cells = std::mem::take(&mut groups[*id].as_mut().unwrap().cells);
            board.remove(&cells);
        }
        for (id, cells) in moves {
            let g = groups[id].as_mut().unwrap();
            board.add(&cells, id as i32, g.move_cost as f64 / g.p as f64);
            g.cells = cells;
        }

        match final_cells {
            Some(cells) => {
                debug_assert_eq!(cells.len(), p);
                accepted += 1;
                occupied += p;
                let comp = 4.0 * (p as f64).sqrt() / final_perim as f64;
                money += (v * comp).round() as i64;
                let mc = move_cost_of(v_int);
                board.add(&cells, idx as i32, mc as f64 / p as f64);
                writeln!(out, "Yes").unwrap();
                for (rr, cc) in cells.iter() {
                    writeln!(out, "{} {}", rr, cc).unwrap();
                }
                groups[idx] = Some(Active { t, p, perim: final_perim, move_cost: mc, cells });
                live.push(idx);
            }
            None => {
                writeln!(out, "No").unwrap();
            }
        }
        board.rebuild();
        out.flush().unwrap();
    }

    eprintln!(
        "local_score = {} accepted = {}/{} fallbacks = {} relocations = {} moved = {} spend = {} elapsed = {} ms",
        money,
        accepted,
        m,
        fallbacks,
        relocations,
        moved_groups,
        move_spend,
        t_start.elapsed().as_millis()
    );
}
