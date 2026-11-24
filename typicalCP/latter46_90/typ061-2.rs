use proconio::{input, marker::Usize1};
use std::collections::VecDeque;
// Typical Problem 061
// Q. Items are given to turn balls into black when either of ball_a, ball_b is white.
// A. Reverse the query by iterating from the last item to the first item.
fn main() {
    input! {n: usize, item_to_balls: [(Usize1, Usize1); n]}
    let mut rev_graph: Vec<Vec<usize>> = vec![vec![]; n];
    for (i, &(ball_a, ball_b)) in item_to_balls.iter().enumerate() {
        rev_graph[ball_a].push(i);
        rev_graph[ball_b].push(i);
    }
    let (mut queue, mut visited) = initialize_bfs(&item_to_balls);

    // =========================================================================
    // Run BFS to build `rev_order` (from last item X_N to first item X_1).
    // - Pop `v` from `queue` and push to `rev_order`.
    // - Iterate over `u` in `rev_graph[v]`: if not `visited[u]`, mark visited and push to `queue`.
    // =========================================================================
    let rev_order: Vec<usize> = run_bfs(&rev_graph, &mut queue, &mut visited);

    // =========================================================================
    // Check if all N items were scheduled.
    // - If `rev_order.len() < n`, print -1.
    // - Otherwise, print `rev_order` in reverse!
    // =========================================================================
    if rev_order.len() < n {
        println!("-1");
    } else {
        for &item in rev_order.iter().rev() {
            println!("{}", item + 1);
        }
    }
}

fn run_bfs(
    rev_graph: &Vec<Vec<usize>>,
    queue: &mut VecDeque<usize>,
    visited: &mut Vec<bool>,
) -> Vec<usize> {
    let n: usize = visited.len();
    let mut rev_order: Vec<usize> = Vec::with_capacity(n);
    while let Some(v) = queue.pop_front() {
        rev_order.push(v);
        for &u in &rev_graph[v] {
            if !visited[u] {
                visited[u] = true;
                queue.push_back(u);
            }
        }
    }
    rev_order
}

fn initialize_bfs(item_to_balls: &Vec<(usize, usize)>) -> (VecDeque<usize>, Vec<bool>) {
    let n: usize = item_to_balls.len();
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut visited: Vec<bool> = vec![false; n];
    for target_ball_idx in 0..n {
        if item_to_balls[target_ball_idx].0 == target_ball_idx
            || item_to_balls[target_ball_idx].1 == target_ball_idx
        {
            queue.push_back(target_ball_idx);
            visited[target_ball_idx] = true;
        }
    }
    return (queue, visited);
}
