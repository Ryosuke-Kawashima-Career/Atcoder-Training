use proconio::input;
// Typical DP Problem C
// Q. There are 2^k people, and calculate the probability that each player can win the tournament.
// A. DP with a Balanced Binary Tree
fn main() {
    // 1 << k is just a slightly safer/cleaner alternative to 2usize.pow(..)
    input! {k: usize, ratings: [usize; 1 << k]}

    // FIX: Explictly cast to f64 FIRST before calculating the difference
    // to prevent usize underflow panics!
    let winning_prob = |p: usize, q: usize| -> f64 {
        return 1.0 / (1.0 + 10.0_f64.powf((ratings[q] as f64 - ratings[p] as f64) / 400.0));
    };

    let n: usize = 1 << k;
    let mut dp: Vec<Vec<(usize, usize, f64)>> = vec![vec![]; 2 * n];

    for person in 0..n {
        dp[person + n] = vec![(person, ratings[person], 1.0)];
    }

    // FIX: We only need to compute internal nodes down to node 1!
    // Node 0 isn't part of a 1-indexed binary tree segment.
    for now in (1..n).rev() {
        let left_winners = dp[now << 1].clone();
        let right_winners = dp[(now << 1) + 1].clone();

        for &(person, rating, prob) in left_winners.iter() {
            let mut cur_prob: f64 = 0.0;
            for &(other_person, _other_rating, other_prob) in right_winners.iter() {
                cur_prob += prob * other_prob * winning_prob(person, other_person);
            }
            dp[now].push((person, rating, cur_prob));
        }

        for &(person, rating, prob) in right_winners.iter() {
            let mut cur_prob: f64 = 0.0;
            for &(other_person, _other_rating, other_prob) in left_winners.iter() {
                cur_prob += prob * other_prob * winning_prob(person, other_person);
            }
            dp[now].push((person, rating, cur_prob));
        }
    }

    // Because left children are always pushed before right children in the loop,
    // dp[1] flawlessly preserves the initial 0..n numerical order of contestants.
    for person in 0..n {
        println!("{}", dp[1][person].2);
    }
}
