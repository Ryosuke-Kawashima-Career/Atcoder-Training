use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, ab: [(Usize1, Usize1); n-1]}
    let mut colors: Vec<Option<usize>> = vec![None; n];
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        graph[a].push(b);
        graph[b].push(a);
    }
    bipartite(0, 0, &graph, &mut colors);
    let mut color0s: usize = 0;
    let mut vertices0: Vec<usize> = Vec::new();
    let mut color1s: usize = 0;
    let mut vertices1: Vec<usize> = Vec::new();
    for v in 0..n {
        if colors[v] == Some(0) {
            color0s += 1;
            vertices0.push(v);
        } else {
            color1s += 1;
            vertices1.push(v);
        }
    }
    if color0s < color1s {
        display(&vertices0);
    } else {
        display(&vertices1);
    }
}

fn display(array: &Vec<usize>) {
    let n: usize = array.len();
    for v in 0..n {
        print!("{} ", array[v] + 1);
    }
    println!("");
}

fn bipartite(
    curr: usize,
    curr_color: usize,
    graph: &Vec<Vec<usize>>,
    colors: &mut Vec<Option<usize>>,
) -> bool {
    let n: usize = graph.len();
    match colors[curr] {
        Some(color) => {
            if curr_color != color {
                return false;
            } else {
                return true;
            }
        }
        None => {
            colors[curr] = Some(curr_color);
        }
    }
    let mut is_partite: bool = true;
    for &next in graph[curr].iter() {
        let next_color = 1 - curr_color;
        if !bipartite(next, next_color, graph, colors) {
            is_partite = false;
        }
    }
    return is_partite;
}
