use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }

    // pref_a[i] = A_1 + A_2 + ... + A_i (sum of the i-th block)
    let mut pref_a = vec![0i64; n + 1];
    for i in 0..n {
        pref_a[i + 1] = pref_a[i] + a[i];
    }

    // pref_block[i] = sum of the first i complete blocks of B
    // len[i] = i * (i + 1) / 2 = total elements in the first i blocks of B
    let mut pref_block = vec![0i64; n + 1];
    let mut len = vec![0u64; n + 1];
    for i in 1..=n {
        pref_block[i] = pref_block[i - 1] + pref_a[i];
        len[i] = len[i - 1] + i as u64;
    }

    // Returns B_1 + B_2 + ... + B_k
    let prefix_sum_b = |k: u64| -> i64 {
        // Find the largest i such that len[i] <= k
        let i = len.partition_point(|&x| x <= k) - 1;
        let rem = (k - len[i]) as usize;
        pref_block[i] + pref_a[rem]
    };

    for _ in 0..q {
        input! {
            l: u64,
            r: u64,
        }
        let ans = prefix_sum_b(r) - prefix_sum_b(l - 1);
        println!("{}", ans);
    }
}
