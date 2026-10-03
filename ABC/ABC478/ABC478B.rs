use proconio::input;

fn main() {
    input! {n: usize, v: usize, w: [i64; n]}
    // value: index -> usize, weight -> i64
    let mut max_w: i64 = 0;
    for i1 in 0..n {
        for i2 in i1 + 1..n {
            for i3 in i2 + 1..n {
                let value: usize = i1 + i2 + i3 + 3;
                if value <= v {
                    let sum_w: i64 = w[i1] + w[i2] + w[i3];
                    max_w = max_w.max(sum_w);
                }
            }
        }
    }
    println!("{}", max_w);
}
