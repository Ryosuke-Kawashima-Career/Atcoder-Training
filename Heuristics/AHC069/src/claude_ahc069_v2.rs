use std::io::{self, BufWriter, Write};

const ETA: f64 = 0.20;
const U_TARGET: f64 = 0.60;
const FEEDBACK_K: f64 = 2.0;
const WARMUP: usize = 30;
const F_MIN: f64 = 0.05;
const G_CLAMP: (f64, f64) = (0.25, 4.0);
const U_FREE: f64 = 0.30;
const FREE_DUR: i64 = 2500;

fn knob(name: &str, default: f64) -> f64 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

const N: usize = 50;

struct Scanner {
    stdin: io::Stdin,
    buf: Vec<String>,
    pos: usize,
}

impl Scanner {
    fn new() -> Self {
        Scanner {
            stdin: io::stdin(),
            buf: Vec::new(),
            pos: 0,
        }
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

fn gen_shapes(p: usize) -> Vec<Shape> {
    let mut out: Vec<Shape> = Vec::new();
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
                    let (bh, bw, rs) = if t == 0 {
                        (h, w, rects.clone())
                    } else {
                        (
                            w,
                            h,
                            rects.iter().map(|&(a, b, c, d)| (b, a, d, c)).collect(),
                        )
                    };
                    let s = Shape {
                        bh,
                        bw,
                        rects: rs,
                        perim: 0,
                    };
                    if s.bh > N || s.bw > N {
                        continue;
                    }
                    let per = perimeter(&s.cells(0, 0));
                    out.push(Shape { perim: per, ..s });
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
    pre: [[u32; N + 1]; N + 1],
    dirty: bool,
}

impl Board {
    fn new(grid: &[Vec<u8>]) -> Self {
        let mut b = Board {
            blocked: [[0; N]; N],
            pre: [[0; N + 1]; N + 1],
            dirty: true,
        };
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
                self.pre[r + 1][c + 1] = self.pre[r][c + 1] + self.pre[r + 1][c] - self.pre[r][c]
                    + self.blocked[r][c] as u32;
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
    let mut money: i64 = 0;

    let a_lawn = grid.iter().flatten().filter(|&&b| b == 0).count() as f64;
    let eta = knob("AHC_ETA", ETA);
    let u_target = knob("AHC_UTARGET", U_TARGET);
    let fb_k = knob("AHC_K", FEEDBACK_K);
    let warmup = knob("AHC_WARMUP", WARMUP as f64) as usize;

    let mut densities: Vec<(f64, f64)> = Vec::with_capacity(m);
    let mut sum_demand: f64 = 0.0;
    let mut occupied: usize = 0;
    let mut accepted = 0usize;

    for idx in 0..m {
        let _i: usize = sc.read();
        let s: i64 = sc.read();
        let t: i64 = sc.read();
        let p: usize = sc.read();
        let v: f64 = sc.read();

        let mut kept = Vec::with_capacity(active.len());
        for g in active.drain(..) {
            if g.t < s {
                occupied -= g.cells.len();
                board.set(&g.cells, 0);
            } else {
                kept.push(g);
            }
        }
        active = kept;
        if board.dirty {
            board.rebuild();
        }

        writeln!(out, "0").unwrap();

        let dur = (t - s) as f64;
        let shapes = gen_shapes(p);
        let c_best = 4.0 * (p as f64).sqrt() / shapes[0].perim as f64;
        let area = p as f64 * dur;
        let density = v * c_best / area;
        densities.push((density, area));
        sum_demand += area;

        let u = occupied as f64 / a_lawn;
        let lambda = if idx < warmup {
            0.0
        } else {
            let demand_rate = sum_demand / s.max(1) as f64;
            let f = (a_lawn * eta / demand_rate).clamp(F_MIN, 1.0);

            let mut ds = densities.clone();
            ds.sort_unstable_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            let mut acc = 0.0;
            let mut q = 0.0;
            for &(d, a) in &ds {
                acc += a;
                q = d;
                if acc >= f * sum_demand {
                    break;
                }
            }
            let g = (u / u_target).powf(fb_k).clamp(G_CLAMP.0, G_CLAMP.1);
            q * g
        };

        let mut placed = None;
        for sh in shapes {
            if let Some((r, c)) = board.find_fit(&sh) {
                placed = Some((sh, r, c));
                break;
            }
        }

        if let Some((ref sh, _, _)) = placed {
            let c_fit = 4.0 * (p as f64).sqrt() / sh.perim as f64;
            let worth = v * c_fit >= lambda * p as f64 * dur;
            let free_pass = u < U_FREE && (t - s) <= FREE_DUR;
            if !(worth || free_pass) {
                placed = None;
            }
        }

        match placed {
            Some((sh, r, c)) => {
                accepted += 1;
                occupied += p;
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
                active.push(Active {
                    t,
                    cells: sh.cells(r, c),
                });
            }
            None => {
                writeln!(out, "No").unwrap();
            }
        }
        out.flush().unwrap();
    }

    eprintln!("local_score = {} accepted = {}/{}", money, accepted, m);
}
