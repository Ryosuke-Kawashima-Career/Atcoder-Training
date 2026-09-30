use proconio::input;

fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {n: usize, w: usize, c: [usize; n]}
        let mut costs: Vec<usize> = c.clone();
        for i in 0..n {
            costs.push(c[i]);
        }
        let mut min_cost: usize = usize::MAX;
        let mut prefix: Vec<usize> = vec![0; 2 * n + 1];
        for i in 1..=2 * n {
            prefix[i] = prefix[i - 1] + costs[i - 1];
        }
        for i in 0..=n {
            let curr_cost: usize = prefix[i + w] - prefix[i];
            if curr_cost < min_cost {
                min_cost = curr_cost;
            }
        }
        println!("{}", min_cost);
    }
}
