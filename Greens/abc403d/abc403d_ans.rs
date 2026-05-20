use proconio::input;
use std::collections::HashMap;
// abc403d
// Q. Any difference of a[i] and a[j] should not be d.
// Example. a = [1, 1, 3, 4, 5], d = 2 | Answer. 1
// A. Mod D に注目する．
fn main() {
    input! {n: usize, d: i64, mut a: [i64; n]}
    a.sort();
    let mut count_values: HashMap<i64, usize> = HashMap::new();
    for i in 0..n {
        *count_values.entry(a[i]).or_insert(0) += 1;
    }
    if d == 0 {
        let ans = n - count_values.len();
        println!("{}", ans);
        return;
    }
    let mut keys_mod: Vec<Vec<i64>> = vec![vec![]; d as usize];
    for key in count_values.keys() {
        keys_mod[(key % d) as usize].push(*key);
    }
    let mut ans: usize = 0;
    for mod_cur in 0..d as usize {
        if keys_mod[mod_cur].len() == 0 {
            continue;
        }
        let mut counts_mod_cur: Vec<usize> = vec![0];
        keys_mod[mod_cur].sort();
        // key同士の距離がdより大きい可能性もあるのか...
        for i in 0..keys_mod[mod_cur].len() {
            let key = keys_mod[mod_cur][i];
            if i > 0 && key - keys_mod[mod_cur][i - 1] > d {
                counts_mod_cur.push(0);
            }
            counts_mod_cur.push(count_values[&key]);
        }
        ans += calc_dp(&counts_mod_cur);
    }
    println!("{}", ans);
}

fn calc_dp(counts_mod_cur: &Vec<usize>) -> usize {
    if counts_mod_cur.len() == 1 {
        return 0;
    }
    // dp[j] を，「Ci,Ci+D,…,Ci+Djの間で条件を満たすために必要な操作回数」(Cxはxが数列に表れるFrequency)
    let mut dp: Vec<usize> = vec![0; counts_mod_cur.len() + 1];
    for i in 1..counts_mod_cur.len() {
        // 連結成分を切り崩すために交互にDPを更新する．
        dp[i + 1] = (dp[i] + counts_mod_cur[i]).min(dp[i - 1] + counts_mod_cur[i - 1]);
    }
    return dp[counts_mod_cur.len()];
}
