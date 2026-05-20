use proconio::input;
const INF: i64 = 1 << 60;
fn main() {
    input! {
        v: usize, e: usize,
        edges: [(usize, usize, i64); e]
    }
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; v];
    for &(v1, v2, cost) in edges.iter() {
        graph[v1].push((v2, cost));
    }
    let bits: usize = 1 << v;
    let mut dp: Vec<Vec<i64>> = vec![vec![INF; bits]; v];
    // dp[current vertex][visited vertices]
    dp[0][0] = 0;
    for bit in 0..bits {
        for curr in 0..v {
            if bit >> curr & 1 == 1 {
                for &(next, cost) in graph[curr].iter() {
                    if bit >> next & 1 == 0 {
                        let next_mask: usize = bit | (1 << next);
                        dp[next][next_mask] = dp[next][next_mask].min(dp[curr][bit] + cost);
                    }
                }
            }
        }
    }
    let ans = dp[0][bits - 1];
    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
