use proconio::input;

fn main() {
    input! {
        n: usize,
        hl: [(usize, usize); n],
        q: usize,
        times: [usize; q],
    }

    let mut suff_max = vec![0; n];
    suff_max[n - 1] = hl[n - 1].0;
    for i in (0..n - 1).rev() {
        suff_max[i] = std::cmp::max(hl[i].0, suff_max[i + 1]);
    }

    for &t in &times {
        let idx = hl.partition_point(|&(_, l)| l <= t);
        println!("{}", suff_max[idx]);
    }
}
