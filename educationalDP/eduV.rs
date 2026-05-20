use proconio::{input, marker::Usize1};
const NIL: usize = 1 << 60;
// Educational DP V
// Q. Count the number of trees which have connected black nodes as one cluster
// A.Re-rooting DP = 全方位木DP
fn main() {
    input! {n: usize, m: usize, xy: [(Usize1, Usize1); n-1]}
    let mut tree: Vec<Vec<usize>> = vec![vec![]; n];
    for &(u, v) in xy.iter() {
        tree[u].push(v);
        tree[v].push(u);
    }
    // dp[node][color: 0: black, 1: white]
    let mut dp: Vec<usize> = vec![0; n];
    run_dp(0, NIL, m, &tree, &mut dp);
    run_prefix_dp(0, NIL, m, &tree, &mut dp, 0);
    for v in 0..n {
        let ans = dp[v];
        println!("{}", ans % m);
    }
}

fn run_dp(v: usize, parent: usize, md: usize, tree: &Vec<Vec<usize>>, dp: &mut Vec<usize>) {
    dp[v] = 1;
    for &next in tree[v].iter() {
        if next == parent {
            continue;
        }
        // Postorder traversal
        run_dp(next, v, md, tree, dp);
        dp[v] *= (dp[next] + 1) % md;
        dp[v] %= md;
    }
}

fn run_prefix_dp(
    v: usize,
    parent: usize,
    md: usize,
    tree: &Vec<Vec<usize>>,
    dp: &mut Vec<usize>,
    parent_value: usize,
) {
    let nv: usize = tree[v].len();
    let mut mul_left: Vec<usize> = vec![0; nv];
    let mut mul_right: Vec<usize> = vec![0; nv];
    for (next_index, &next) in tree[v].iter().enumerate() {
        if next == parent {
            mul_left[next_index] = (parent_value + 1) % md;
            mul_right[next_index] = (parent_value + 1) % md;
            dp[v] *= (parent_value + 1) % md;
            dp[v] %= md;
        } else {
            mul_left[next_index] = (dp[next] + 1) % md;
            mul_right[next_index] = (dp[next] + 1) % md;
        }
    }
    for next_index in 0..nv - 1 {
        mul_left[next_index + 1] *= mul_left[next_index];
        mul_left[next_index + 1] %= md;
    }
    for next_index in (1..nv).rev() {
        mul_right[next_index - 1] *= mul_right[next_index];
        mul_right[next_index - 1] %= md;
    }
    for (next_index, &next) in tree[v].iter().enumerate() {
        if next == parent {
            continue;
        }
        let value_left = if next_index == 0 {
            1
        } else {
            mul_left[next_index - 1]
        };
        let value_right = if next_index == nv - 1 {
            1
        } else {
            mul_right[next_index + 1]
        };
        let value = value_left * value_right % md;
        run_prefix_dp(next, v, md, tree, dp, value);
    }
}
