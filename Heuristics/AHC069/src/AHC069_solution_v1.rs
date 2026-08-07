// AHC069 - v1 baseline: quasi-square placement, first fit, no moves, no rejections.
//
// Strategy (see docs/AHC069_implementation_plan.md):
//   - The fee is round(V * C) with C = 4*sqrt(P)/L, and L depends only on the shape,
//     never on the position. So we always try the most compact shapes first.
//   - The most compact P-cell polyomino is the "quasi-square": a w x h rectangle with
//     the surplus w*h-P cells removed from one end of one border row (or column).
//   - Each such shape is the union of at most two rectangles, so a 2D prefix sum over
//     blocked cells answers "does it fit here?" in O(1).
//
// v1 accepts every group that fits anywhere. Admission control comes in v2.

use std::io::{self, BufWriter, Write};

const N: usize = 50;

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

/// A shape is the union of at most two axis-aligned rectangles, given as
/// (r0, c0, r1, c1) half-open relative boxes inside a bh x bw bounding box.
#[derive(Clone)]
struct Shape {
    bh: usize,
    bw: usize,
    rects: Vec<(usize, usize, usize, usize)>,
    perim: usize,
}

impl Shape {
    fn cells(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for &(r0, c0, r1, c1) in &self.rects {
            for rr in r0..r1 {
                for cc in c0..c1 {
                    v.push((r + rr, c + cc));
                }
            }
        }
        v
    }
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

/// All quasi-square shapes for `p` cells, sorted by increasing perimeter
/// (i.e. decreasing compactness). Includes both notch alignments and transposes.
fn gen_shapes(p: usize) -> Vec<Shape> {
    let mut out: Vec<Shape> = Vec::new();
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
                    let (bh, bw, rs) = if t == 0 {
                        (h, w, rects.clone())
                    } else {
                        (w, h, rects.iter().map(|&(a, b, c, d)| (b, a, d, c)).collect())
                    };
                    let s = Shape { bh, bw, rects: rs, perim: 0 };
                    if s.bh > N || s.bw > N {
                        continue;
                    }
                    let per = perimeter(&s.cells(0, 0));
                    out.push(Shape { perim: per, ..s });
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
    blocked: [[u8; N]; N],       // 1 = pond or occupied
    pre: [[u32; N + 1]; N + 1],  // 2D prefix sum of `blocked`
    dirty: bool,
}

impl Board {
    fn new(grid: &[Vec<u8>]) -> Self {
        let mut b = Board { blocked: [[0; N]; N], pre: [[0; N + 1]; N + 1], dirty: true };
        for r in 0..N {
            for c in 0..N {
                b.blocked[r][c] = grid[r][c];
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
            }
        }
        self.dirty = false;
    }
    #[inline]
    fn rect_blocked(&self, r0: usize, c0: usize, r1: usize, c1: usize) -> u32 {
        self.pre[r1][c1] + self.pre[r0][c0] - self.pre[r0][c1] - self.pre[r1][c0]
    }
    fn set(&mut self, cells: &[(usize, usize)], v: u8) {
        for &(r, c) in cells {
            self.blocked[r][c] = v;
        }
        self.dirty = true;
    }
    /// First anchor at which `s` fits, scanning row-major.
    fn find_fit(&self, s: &Shape) -> Option<(usize, usize)> {
        if s.bh > N || s.bw > N {
            return None;
        }
        for r in 0..=N - s.bh {
            'anchor: for c in 0..=N - s.bw {
                for &(r0, c0, r1, c1) in &s.rects {
                    if self.rect_blocked(r + r0, c + c0, r + r1, c + c1) != 0 {
                        continue 'anchor;
                    }
                }
                return Some((r, c));
            }
        }
        None
    }
}

// ---------------------------------------------------------------- main

struct Active {
    t: i64,
    cells: Vec<(usize, usize)>,
}

fn main() {
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
    let mut active: Vec<Active> = Vec::new();
    let mut money: i64 = 0; // local score model, printed to stderr for cross-checking

    for _ in 0..m {
        let _i: usize = sc.read();
        let s: i64 = sc.read();
        let t: i64 = sc.read();
        let p: usize = sc.read();
        let v: f64 = sc.read();

        // 1. release the regions of groups that have already departed.
        // The fee itself is booked at placement time: v1 never moves a group, so its
        // compactness can never change after it is placed.
        let mut kept = Vec::with_capacity(active.len());
        for g in active.drain(..) {
            if g.t < s {
                board.set(&g.cells, 0);
            } else {
                kept.push(g);
            }
        }
        active = kept;
        if board.dirty {
            board.rebuild();
        }

        // 2. no moves in v1
        writeln!(out, "0").unwrap();

        // 3. place the arriving group in the most compact shape that fits
        let mut placed = None;
        for sh in gen_shapes(p) {
            if let Some((r, c)) = board.find_fit(&sh) {
                placed = Some((sh, r, c));
                break;
            }
        }

        match placed {
            Some((sh, r, c)) => {
                let cells = sh.cells(r, c);
                debug_assert_eq!(cells.len(), p);
                let comp = 4.0 * (p as f64).sqrt() / sh.perim as f64;
                money += (v * comp).round() as i64;
                board.set(&cells, 1);
                board.rebuild();
                writeln!(out, "Yes").unwrap();
                for (rr, cc) in cells.iter() {
                    writeln!(out, "{} {}", rr, cc).unwrap();
                }
                active.push(Active { t, cells: sh.cells(r, c) });
            }
            None => {
                writeln!(out, "No").unwrap();
            }
        }
        out.flush().unwrap();
    }

    eprintln!("local_score = {}", money);
}
