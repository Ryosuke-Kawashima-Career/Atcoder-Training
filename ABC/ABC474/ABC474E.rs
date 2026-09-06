use proconio::input;

fn solve() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }

    let mut min_a = i64::MAX;
    let mut m_star = 0;
    let mut min_d_for_min_a = i64::MAX;

    let mut b_total: i64 = 0;
    let mut d: Vec<i64> = Vec::with_capacity(n);

    for (i, &(a, b)) in ab.iter().enumerate() {
        b_total += b;
        let diff = a - b;
        d.push(diff);

        if a < min_a || (a == min_a && diff < min_d_for_min_a) {
            min_a = a;
            min_d_for_min_a = diff;
            m_star = i;
        }
    }

    // Region 1: No extra purchases needed (k >= ceil(n / 2))
    // Since D_i > 0, the minimum is achieved at k = ceil(n / 2) with the smallest D_i.
    let k_reg1 = (n + 1) / 2;
    let mut sorted_d = d.clone();
    sorted_d.sort_unstable();

    let mut ans: i64 = b_total + sorted_d.iter().take(k_reg1).sum::<i64>();

    // Region 2: Extra purchases of the cheapest coupon item (m_star) are used (1 <= k < ceil(n / 2)).
    // In this case, m_star MUST be bought without coupon (m_star in S_A).
    // The remaining (k - 1) items in S_A are chosen from the remaining items with smallest D_i.
    let mut other_d: Vec<i64> = d
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != m_star)
        .map(|(_, &diff)| diff)
        .collect();
    other_d.sort_unstable();

    let mut pref_sum: i64 = 0;
    for k in 1..k_reg1 {
        let sum_d = d[m_star] + pref_sum;
        let extra_coupons = (n - 2 * k) as i64;
        let cost = b_total + sum_d + extra_coupons * min_a;
        ans = ans.min(cost);

        if k - 1 < other_d.len() {
            pref_sum += other_d[k - 1];
        }
    }

    println!("{}", ans);
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        solve();
    }
}

