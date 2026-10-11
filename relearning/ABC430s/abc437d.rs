use proconio::input;

fn main() {
    input! {n: usize, m: usize, mut a: [i64; n], mut b: [i64; m]}
    a.sort();
    b.sort();
    let mut positive_contributions: i64 = 0;
    let mut negative_contributions: i64 = 0;
    for i in 0..n {
        let index: usize = b.partition_point(|&x| x < a[i]);
        positive_contributions += index as i64 * a[i];
        negative_contributions += (m - index) as i64 * a[i];
    }
    for j in 0..m {
        let index: usize = a.partition_point(|&x| x <= b[j]);
        positive_contributions += index as i64 * b[j];
        negative_contributions += (n - index) as i64 * b[j];
    }
    println!("{}", positive_contributions - negative_contributions);
}
