use proconio::input;

fn main() {
    input! {n: usize, q: usize, a: [i64; n], queries: [(usize, usize); q]}
    let mut prefix: Vec<i64> = vec![0; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + a[i];
    }
    let mut prefix_meta: Vec<i64> = vec![0; n + 1];
    for i in 1..=n {
        prefix_meta[i] = prefix_meta[i - 1] + prefix[i - 1];
    }
    let mut length: Vec<i64> = vec![0; n + 1];
    for i in 1..=n {
        length[i] = length[i - 1] + (i as i64) * (i as i64 + 1) / 2;
    }
    let get_index = |idx: usize| -> usize {
        length
            .binary_search(&(idx as i64))
            .unwrap_or_else(|x| x - 1)
    };
    for (l, r) in queries {
        let li: usize = get_index(l);
        let ri: usize = get_index(r);
        let mut ans: i64 = 0;
        ans += prefix_meta[ri];
        ans -= prefix_meta[li];
        println!("{}", ans);
    }
}
