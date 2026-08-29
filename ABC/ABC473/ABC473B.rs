use proconio::input;

fn main() {
    input! {n: usize, a: [i64; n]}
    let mut count: Vec<i64> = vec![0; 101];
    for i in 0..n {
        count[a[i] as usize] += 1;
    }
    let mut ans: i64 = 0;
    for num in 1..=100 {
        ans += (count[num as usize] % 2) as i64 * num as i64;
    }
    println!("{}", ans);
}
