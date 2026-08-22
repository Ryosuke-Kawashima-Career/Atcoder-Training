use proconio::input;

fn main() {
    input!{n: usize, m: usize, k: i64, a: [i64; n]}
    let mut prefix: Vec<i64> = vec![0; n+1];
    for i in 1..=n {
        prefix[i] = prefix[i-1] + a[i-1];
    }
    for day in 1..=n {
        let min_day = if day >= m {
            day - m
        } else {
            0
        };
        let calories: i64 = prefix[day] - prefix[min_day];
        if calories <= k {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
