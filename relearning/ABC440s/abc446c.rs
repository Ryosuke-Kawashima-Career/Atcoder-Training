use proconio::input;
use std::collections::VecDeque;
fn main() {
    input! {t: usize}
    for _ in 0..t {
        input! {n: usize, d: usize, purchased: [i64; n], used: [i64; n]}
        let mut eggs: VecDeque<(usize, i64)> = VecDeque::new();
        for day in 0..n {
            // morning
            eggs.push_back((day, purchased[day]));
            // day
            let mut to_be_used: i64 = used[day];
            while to_be_used > 0 && !eggs.is_empty() {
                let egg: i64 = eggs.front().unwrap().1;
                if to_be_used <= egg {
                    eggs.front_mut().unwrap().1 -= to_be_used;
                    to_be_used = 0;
                } else {
                    to_be_used -= egg;
                    eggs.pop_front();
                }
            }
            // evening
            while !eggs.is_empty() && eggs.front().unwrap().0 + d <= day {
                eggs.pop_front();
            }
        }
        let curr_sum: i64 = eggs.iter().map(|x| x.1).sum();
        println!("{}", curr_sum);
    }
}
