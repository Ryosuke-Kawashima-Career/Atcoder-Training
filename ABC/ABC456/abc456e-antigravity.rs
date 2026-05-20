use proconio::{input, marker::Chars, marker::Usize1};
use std::collections::VecDeque;

fn solve() {
    input! {
        n: usize,
        m: usize,
    }
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        input! { u: Usize1, v: Usize1 }
        adj[u].push(v);
        adj[v].push(u);
    }
    input! {
        w: usize,
        s: [Chars; n],
    }

    let mut holiday = vec![vec![false; w]; n];
    for i in 0..n {
        for j in 0..w {
            if s[i][j] == 'o' {
                holiday[i][j] = true;
            }
        }
    }

    let mut out_degree = vec![vec![0usize; w]; n];
    let mut removed = vec![vec![false; w]; n];
    let mut queue = VecDeque::new();

    for i in 0..n {
        for j in 0..w {
            if !holiday[i][j] {
                removed[i][j] = true;
                continue;
            }
            let next_day = (j + 1) % w;
            let mut count = 0;
            if holiday[i][next_day] {
                count += 1;
            }
            for &v in &adj[i] {
                if holiday[v][next_day] {
                    count += 1;
                }
            }
            out_degree[i][j] = count;
            if count == 0 {
                queue.push_back((i, j));
            }
        }
    }

    while let Some((curr_city, curr_day)) = queue.pop_front() {
        if removed[curr_city][curr_day] {
            continue;
        }
        removed[curr_city][curr_day] = true;

        let prev_day = (curr_day + w - 1) % w;

        if holiday[curr_city][prev_day] && !removed[curr_city][prev_day] {
            out_degree[curr_city][prev_day] -= 1;
            if out_degree[curr_city][prev_day] == 0 {
                queue.push_back((curr_city, prev_day));
            }
        }

        for &p in &adj[curr_city] {
            if holiday[p][prev_day] && !removed[p][prev_day] {
                out_degree[p][prev_day] -= 1;
                if out_degree[p][prev_day] == 0 {
                    queue.push_back((p, prev_day));
                }
            }
        }
    }

    for i in 0..n {
        if holiday[i][0] && !removed[i][0] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn main() {
    input! { t: usize }
    for _ in 0..t {
        solve();
    }
}
