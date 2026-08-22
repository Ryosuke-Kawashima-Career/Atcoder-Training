use proconio::input;

fn main() {
    input!{n: usize, l: [i64; n]};
    let sum: i64 = l.iter().sum();
    let sum_half: i64 = sum / 2;
    let mut length: i64 = 0;
    let mut ans: i64 = sum;
    for i in 0..n {
        length += l[i];
        let diff: i64 = ((sum - length) - length).abs();
        ans = ans.min(diff);
    }
    println!("{}", ans);
}
