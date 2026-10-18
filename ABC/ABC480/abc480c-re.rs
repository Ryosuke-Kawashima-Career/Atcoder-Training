use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        t: i64,
        mut a: [i64; n],
    }
    a.sort_unstable();
    let mut due_time: i64 = 0;
    let mut count: i64 = 0;
    if a[0] != 0 {
        count += 1;
        due_time += k;
    }
    for i in 0..n {
        if due_time >= t {
            break;
        }
        let aoki_time: i64 = a[i];
        if aoki_time > due_time {
            // ceiling
            let required_taps: i64 = (aoki_time - due_time + k - 1) / k;
            due_time = aoki_time + k;
            count += required_taps;
        } else {
            due_time = aoki_time + k;
        }
    }
    if due_time < t {
        let required_taps = (t - due_time + k - 1) / k;
        count += required_taps;
    }
    println!("{}", count);
}
