use proconio::input;

fn main() {
    input! {n: usize, m: usize, k: i64, a: [i64; n]}
    let mut window_sum: i64 = 0;
    let mut did_eat: Vec<bool> = vec![false; n];
    for i in 0..n {
        let start_day: isize = i as isize - m as isize;
        if start_day >= 0 && did_eat[start_day as usize] {
            window_sum -= a[start_day as usize];
        }
        window_sum += a[i];
        if window_sum > k {
            window_sum -= a[i];
            did_eat[i] = false;
        } else {
            did_eat[i] = true;
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
