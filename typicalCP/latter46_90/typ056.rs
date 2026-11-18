use proconio::input;
fn main() {
    input! {n: usize, s: usize, ab: [(usize, usize); n]}
    // dp[i][j]: whether buying one bag on each of the first i days can total j.
    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for item in 1..=n {
        let (price_a, price_b) = ab[item - 1];
        for price in 0..=s {
            if price >= price_a && dp[item - 1][price - price_a] {
                dp[item][price] = true;
            }
            if price >= price_b && dp[item - 1][price - price_b] {
                dp[item][price] = true;
            }
        }
    }
    if !dp[n][s] {
        println!("Impossible");
        return;
    }
    let purchase = recon_dp(&dp, &ab);
    println!("{}", purchase);
}

fn recon_dp(dp: &[Vec<bool>], prices: &[(usize, usize)]) -> String {
    let mut curr_price = dp[0].len() - 1;
    let mut purchase = Vec::with_capacity(prices.len());
    for curr_item in (1..dp.len()).rev() {
        let (price_a, price_b) = prices[curr_item - 1];
        if curr_price >= price_a && dp[curr_item - 1][curr_price - price_a] {
            purchase.push('A');
            curr_price -= price_a;
        } else {
            // The current state is reachable, so B must be a valid predecessor.
            debug_assert!(curr_price >= price_b && dp[curr_item - 1][curr_price - price_b]);
            purchase.push('B');
            curr_price -= price_b;
        }
    }
    debug_assert_eq!(curr_price, 0);
    purchase.into_iter().rev().collect()
}
