use proconio::input;

fn main() {
    input!{n: usize, m: usize, k: i64, a: [i64; n]}
    let mut did_eat: Vec<bool> = vec![false; n+1];
    let mut calories: i64 = 0;
    for day in 1..=n {
        calories += a[day-1];
        let min_day = if day >= m {
            day - m
        } else {
            0
        };
        if calories <= k {
            did_eat[day] = true;
        } else {
            did_eat[day] = false;
        }
        if min_day >= 1 && did_eat[min_day] {
            calories -= a[min_day-1];
        }
    }
    for day in 1..=n {
        if did_eat[day] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
