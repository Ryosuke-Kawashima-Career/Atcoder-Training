use proconio::{input, marker::Chars};

fn main() {
    input! {t: usize}
    let mut answers: Vec<String> = Vec::new();
    for _case in 0..t {
        input! {n: usize, m: usize, uv: [(Usize1, Usize1); m], w: usize, s: [Chars; n]}
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        for &(u, v) in uv.iter() {
            graph[u].push(v);
            graph[v].push(u);
        }
        let is_ok: bool = solve(&graph, &s);
        if is_ok {
            answers.push("Yes".to_string());
        } else {
            answers.push("No".to_string());
        }
    }
    for ans in answers {
        println!("{}", ans);
    }
}

fn solve(graph: &Vec<Vec<usize>>, s: &Vec<Chars>) -> bool {
    let n: usize = graph.len();
    let w: usize = s[0].len();
}
