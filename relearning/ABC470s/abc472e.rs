use proconio::{input, marker::Usize1};
use std::collections::VecDeque;
fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {n: usize, m: usize, ab: [(Usize1, Usize1); m]}
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        for (a, b) in ab {
            graph[a].push(b);
            graph[b].push(a);
        }
        let mut colors: Vec<isize> = vec![-1; n];
        let mut parent: Vec<usize> = vec![usize::MAX; n];
        let (is_bipartite, pair_1, pair_2) = judge_bipartite(&graph, &mut colors, &mut parent);
        if !is_bipartite && pair_1.is_some() && pair_2.is_some() {
            let path: Vec<usize> = edmons_blossom(&graph, &parent, pair_1, pair_2);
            let k: usize = path.len();
            println!("{}", k);
            for v in 0..k {
                print!("{} ", path[v] + 1);
            }
            println!("");
        } else {
            println!("-1");
        }
    }
}

fn judge_bipartite(
    graph: &Vec<Vec<usize>>,
    colors: &mut Vec<isize>,
    parent: &mut Vec<usize>,
) -> (bool, Option<usize>, Option<usize>) {
    let n: usize = colors.len();
    let mut que = VecDeque::new();
    let mut pair_1: Option<usize> = None;
    let mut pair_2: Option<usize> = None;
    for v in 0..n {
        if colors[v] == -1 {
            colors[v] = 0;
            que.push_back((v, 0));
            while let Some((curr_node, curr_color)) = que.pop_front() {
                if pair_1.is_some() && pair_2.is_some() {
                    return (false, pair_1, pair_2);
                }
                for &next in graph[curr_node].iter() {
                    if colors[next] == -1 {
                        colors[next] = 1 - curr_color;
                        que.push_back((next, 1 - curr_color));
                        parent[next] = curr_node;
                    } else if colors[next] == curr_color {
                        return (false, Some(curr_node), Some(next));
                    }
                }
            }
        }
    }
    (true, pair_1, pair_2)
}

fn edmons_blossom(
    graph: &Vec<Vec<usize>>,
    parent: &Vec<usize>,
    pair_1: Option<usize>,
    pair_2: Option<usize>,
) -> Vec<usize> {
    /* Reconstructs the path */
    let mut v1: usize = pair_1.unwrap();
    let mut v2: usize = pair_2.unwrap();
    let mut path_1: Vec<usize> = vec![v1];
    let mut path_2: Vec<usize> = vec![v2];
    while v1 != v2 {
        v1 = parent[v1];
        v2 = parent[v2];
        path_1.push(v1);
        path_2.push(v2);
        if v1 == v2 {
            break;
        }
    }
    path_2.pop();
    path_1.extend(path_2.into_iter().rev());
    path_1
}
