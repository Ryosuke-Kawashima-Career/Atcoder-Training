use proconio::input;
const LIMIT: usize = 1_000_000;
// Register to Imos when it is applicable.
fn main() {
    input! {n: usize, d: usize, st: [(usize, usize); n]}
    let mut imos: Vec<isize> = vec![0isize; LIMIT + 1];
    for &(s, t) in st.iter() {
        if t >= s + d {
            imos[s] += 1;
            imos[t + 1 - d] -= 1;
        }
    }
    for time in 1..=LIMIT {
        imos[time] += imos[time - 1];
    }
    let mut comb: isize = 0;
    for time in 0..=LIMIT {
        comb += imos[time] * (imos[time] - 1) / 2;
    }
    println!("{}", comb);
}
