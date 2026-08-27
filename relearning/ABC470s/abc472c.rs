use proconio::input;

fn main() {
    input! {n: usize, m: usize, k: i64, a: [i64; n]}
    let mut window_sum: i64 = 0;
    let mut did_eat: Vec<bool> = vec![false; n];
    for i in 0..n {
        let mut last_day: isize = i as isize - m as isize;
        if last_day >= 0 && did_eat[last_day as usize] {
            window_sum -= a[last_day as usize];
        }
        if window_sum + a[i] <= k {
            did_eat[i] = true;
            window_sum += a[i];
        }
    }
    for day in 0..n {
        if did_eat[day] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
