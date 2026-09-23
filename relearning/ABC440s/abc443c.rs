use proconio::input;

fn main() {
    input! {n: usize, t: usize, a: [usize; n]}
    let mut cur_time: usize = 0;
    let mut ans: usize = 0;
    for i in 0..n {
        if a[i] >= cur_time {
            ans += a[i] - cur_time;
            cur_time = a[i] + 100;
        } else {
            continue;
        }
    }
    ans += t.saturating_sub(cur_time);
    println!("{ans}")
}
