use proconio::input;
const INF: i64 = 1 << 60;

fn main() {
    input! {n: usize}
    let mut cost: Vec<Vec<i64>> = vec![vec![INF; n]; n];
    
    // Read the upper triangle (without diagonal) of the cost matrix
    for i in 0..n - 1 {
        input! {cost_i: [i64; n - 1 - i]}
        for j in i + 1..n {
            // FIX: Normalize the index. 
            // `j` spans from `i+1` to `n-1`, but `cost_i`'s valid indices are `0` to `n-1-i -1`.
            cost[i][j] = cost_i[j - i - 1];
        }
    }
    
    let mut is_ok: bool = false;
    for a in 0..n {
        for b in a + 1..n {
            for c in b + 1..n {
                if cost[a][b] + cost[b][c] < cost[a][c] {
                    is_ok = true;
                }
            }
        }
    }
    
    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
