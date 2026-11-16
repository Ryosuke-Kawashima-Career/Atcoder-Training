use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        xy: [(i64, i64); n],
    }

    let mut dist2 = vec![vec![0_i64; n]; n];
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = xy[i].0 - xy[j].0;
            let dy = xy[i].1 - xy[j].1;
            let d = dx * dx + dy * dy;
            dist2[i][j] = d;
            dist2[j][i] = d;
        }
    }

    let full = (1usize << n) - 1;
    let mut cost = vec![0_i64; 1 << n];
    for mask in 1..=full {
        let lsb = mask.trailing_zeros() as usize;
        let prev = mask ^ (1 << lsb);
        let mut max_d = cost[prev];
        for j in (lsb + 1)..n {
            if (prev >> j) & 1 == 1 {
                max_d = max_d.max(dist2[lsb][j]);
            }
        }
        cost[mask] = max_d;
    }

    const INF: i64 = i64::MAX;
    let mut dp_prev = cost.clone();
    for c in 2..=k {
        let mut dp_curr = vec![INF; 1 << n];
        for mask in 1..=full {
            if (mask.count_ones() as usize) < c {
                continue;
            }
            let lsb = 1usize << mask.trailing_zeros();
            let rem = mask ^ lsb;
            let mut best = INF;
            let mut sub_rem = (rem - 1) & rem;
            loop {
                let sub = sub_rem | lsb;
                let rest = mask ^ sub;
                best = best.min(dp_prev[rest].max(cost[sub]));
                if sub_rem == 0 {
                    break;
                }
                sub_rem = (sub_rem - 1) & rem;
            }
            dp_curr[mask] = best;
        }
        dp_prev = dp_curr;
    }

    println!("{}", dp_prev[full]);
}
