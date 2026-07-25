use proconio::{input, marker::Chars};

fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {n: usize, s: Chars, x: [i64; n], y: [i64; n-1]}
        let mut dp_sunny: Vec<i64> = vec![0; n];
        let mut dp_rainny: Vec<i64> = vec![0; n];
        if s[0] == 'S' {
            dp_rainny[0] = -x[0];
        } else {
            dp_sunny[0] = -x[0];
        }

        for day in 1..n {
            if s[day] == 'S' {
                dp_sunny[day] = dp_sunny[day - 1].max(dp_rainny[day - 1] + y[day - 1]);
                dp_rainny[day] = dp_rainny[day - 1].max(dp_sunny[day - 1]) - x[day];
            } else {
                dp_sunny[day] = (dp_rainny[day - 1] + y[day - 1]).max(dp_sunny[day - 1]) - x[day];
                dp_rainny[day] = (dp_sunny[day - 1]).max(dp_rainny[day - 1]);
            }
        }
        let ans = dp_sunny[n - 1].max(dp_rainny[n - 1]);
        println!("{}", ans);
    }
}
