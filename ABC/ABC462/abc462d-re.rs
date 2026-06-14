use proconio::input;
const LIMIT: usize = 1_000_000;
// ABC462d
// Q. Two culprits commited a murder for d miniues. They entered the scene from time=S to time=T.
// What is the number of possible combinations of two suspects?
// A. Imos and combination
fn main() {
    input! {n: usize, d: usize, st: [(usize,usize); n]}
    let mut imos: Vec<i64> = vec![0; LIMIT + 1];
    for &(entry, exit) in st.iter() {
        if exit >= entry + d {
            imos[entry] += 1;
            imos[exit - d + 1] -= 1;
        }
    }
    for time in 1..=LIMIT {
        imos[time] += imos[time - 1];
    }

    let mut ans = 0;
    for time in 1..=LIMIT {
        if imos[time] >= 2 {
            let combination: usize = (imos[time] as usize) * (imos[time] as usize - 1) / 2;
            ans += combination;
        }
    }
    println!("{}", ans);
}
