use proconio::{input, marker::Usize1};
use std::collections::VecDeque;
use std::fmt::Write;

fn construct_sequence(n: usize, conditions: &[(usize, usize, usize)]) -> Option<Vec<usize>> {
    let mut graph = vec![Vec::new(); n];
    let mut reverse_graph = vec![Vec::new(); n];
    for &(_, u, v) in conditions {
        graph[u].push(v);
        reverse_graph[v].push(u);
    }

    // First pass of Kosaraju: obtain DFS finishing order without recursion.
    let mut visited = vec![false; n];
    let mut order = Vec::with_capacity(n);
    let mut dfs_stack = Vec::new();
    for start in 0..n {
        if visited[start] {
            continue;
        }
        visited[start] = true;
        dfs_stack.push((start, 0));
        while let Some((v, next_index)) = dfs_stack.last_mut() {
            if *next_index < graph[*v].len() {
                let next = graph[*v][*next_index];
                *next_index += 1;
                if !visited[next] {
                    visited[next] = true;
                    dfs_stack.push((next, 0));
                }
            } else {
                let (v, _) = dfs_stack.pop().unwrap();
                order.push(v);
            }
        }
    }

    // Second pass: each traversal of the reversed graph identifies one SCC.
    let mut component = vec![usize::MAX; n];
    let mut component_count = 0;
    let mut stack = Vec::new();
    for &start in order.iter().rev() {
        if component[start] != usize::MAX {
            continue;
        }
        component[start] = component_count;
        stack.push(start);
        while let Some(v) = stack.pop() {
            for &next in &reverse_graph[v] {
                if component[next] == usize::MAX {
                    component[next] = component_count;
                    stack.push(next);
                }
            }
        }
        component_count += 1;
    }

    let mut dag = vec![Vec::new(); component_count];
    let mut indegree = vec![0; component_count];
    for &(t, u, v) in conditions {
        let from = component[u];
        let to = component[v];
        if from == to {
            // All values in an SCC must be equal, so a strict edge is impossible.
            if t == 1 {
                return None;
            }
        } else {
            dag[from].push((to, t));
            indegree[to] += 1;
        }
    }

    // Longest-path DP on the DAG enforces value[to] >= value[from] + t.
    let mut value = vec![1; component_count];
    let mut queue = VecDeque::new();
    for c in 0..component_count {
        if indegree[c] == 0 {
            queue.push_back(c);
        }
    }
    while let Some(c) = queue.pop_front() {
        for &(next, weight) in &dag[c] {
            value[next] = value[next].max(value[c] + weight);
            indegree[next] -= 1;
            if indegree[next] == 0 {
                queue.push_back(next);
            }
        }
    }

    Some(component.iter().map(|&c| value[c]).collect())
}

fn main() {
    input! {
        n: usize,
        q: usize,
        conditions: [(usize, Usize1, Usize1); q],
    }

    match construct_sequence(n, &conditions) {
        None => println!("No"),
        Some(a) => {
            let mut output = String::from("Yes\n");
            for (i, value) in a.iter().enumerate() {
                if i > 0 {
                    output.push(' ');
                }
                write!(output, "{}", value).unwrap();
            }
            output.push('\n');
            print!("{}", output);
        }
    }
}
