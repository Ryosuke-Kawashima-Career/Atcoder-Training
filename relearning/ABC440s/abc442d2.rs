use proconio::input;

fn main() {
    input! {n: usize, q: usize, mut a: [i64; n]}
    let mut prefix: Vec<i64> = vec![0; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + a[i];
    }
    for _query in 0..q {
        input! {query_type: usize}
        if query_type == 1 {
            input! {x: usize}
            let prev_val: i64 = a[x - 1];
            let next_val: i64 = a[x];
            a[x - 1] = next_val;
            a[x] = prev_val;
            prefix[x] += next_val - prev_val;
        } else {
            input! {l: usize, r: usize}
            let ans: i64 = prefix[r] - prefix[l - 1];
            println!("{}", ans);
        }
    }
}
