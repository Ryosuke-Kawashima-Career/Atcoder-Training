use proconio::input;
#[derive(Debug, Clone, Copy)]
struct Deer {
    weight: usize,
    power: usize,
    contribution: usize,
}
impl Deer {
    fn new(w: usize, p: usize) -> Self {
        Self {
            weight: w,
            power: p,
            contribution: w + p,
        }
    }
}
fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {n: usize, wp: [(usize, usize); n]}
        let mut deers: Vec<Deer> = Vec::with_capacity(n);
        let mut curr_weights: usize = 0;
        for (w, p) in wp {
            curr_weights += w;
            deers.push(Deer::new(w, p));
        }
        deers.sort_by(|a, b| b.contribution.cmp(&a.contribution));
        let mut total_power: usize = 0;
        let mut total_count: usize = 0;
        for i in 0..n {
            curr_weights -= deers[i].weight;
            total_power += deers[i].power;
            total_count += 1;
            if total_power > curr_weights {
                break;
            }
        }
        let mut n_on_sleds: usize = n - total_count;
        if curr_weights > total_power {
            n_on_sleds = 0;
        }
        println!("{}", n_on_sleds);
    }
}
