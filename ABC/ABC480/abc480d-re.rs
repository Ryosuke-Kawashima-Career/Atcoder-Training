use proconio::input;
fn main() {
    input! {n: usize, q: usize, a: [usize; n], queries: [(usize, usize); q]}
    let mut prefix: Vec<usize> = vec![0; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + a[i];
    }
    let mut prefix_meta: Vec<usize> = vec![0; n + 2];
    for i in 1..=n {
        prefix_meta[i + 1] = prefix_meta[i] + prefix[i];
    }
    let mut length_prefix: Vec<usize> = vec![0; n + 1];
    for i in 1..=n {
        length_prefix[i + 1] = length_prefix[i] + (i * (i + 1) / 2);
    }

    let get_value = |idx: usize| -> usize {
        let idx_length: usize = length_prefix.partition_point(|&x| x < idx);
        let length_remain: usize = idx - length_prefix[idx_length];
        return prefix_meta[idx_length] + prefix[length_remain];
    };
    for idx_query in 0..q {
        let (l, r) = queries[idx_query];
        let ans: usize = get_value(r) - get_value(l - 1);
        println!("{}", ans);
    }
}
