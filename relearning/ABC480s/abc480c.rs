use proconio::input;

fn main() {
    input! {n: usize, k: isize, t: isize, mut a: [isize; n]}
    a.sort_unstable();
    let mut curr_time: isize = 0;
    let mut ans: isize = 0;
    for i in 0..n {
        let time_diff: isize = a[i] - curr_time;
        if time_diff > 0 {
            let addition: isize = (time_diff + k - 1) / k;
            ans += addition;
            curr_time += addition * k;
        }
        curr_time = curr_time.max(a[i] + k);
    }
    if curr_time < t {
        let addition: isize = (t - curr_time + k - 1) / k;
        ans += addition;
    }
    println!("{}", ans);
}
