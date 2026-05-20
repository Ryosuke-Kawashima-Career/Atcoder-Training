use proconio::input;

fn main() {
    input! {t: usize, x: i64, a: [i64; t+1]}
    let mut time_and_values: Vec<(usize, i64)> = Vec::new();
    for time in 0..=t {
        if time == 0 {
            time_and_values.push((time, a[0]));
            continue;
        }
        let prev_value = time_and_values[time_and_values.len() - 1].1;
        let diff = (a[time] - prev_value).abs();
        if diff >= x {
            time_and_values.push((time, a[time]));
        } else {
            // do nothing
        }
    }
    let k: usize = time_and_values.len();
    for i in 0..k {
        println!("{} {}", time_and_values[i].0, time_and_values[i].1);
    }
}
