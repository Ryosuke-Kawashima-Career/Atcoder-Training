use proconio::{
    input,
    marker::{Chars, Usize1},
};
fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {n: usize, m: usize, uv: [(Usize1, Usize1); m], w: usize, s: [Chars; n]}
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        for &(u, v) in uv.iter() {
            graph[u].push(v);
            graph[v].push(u);
        }
        let mut is_holiday: Vec<bool> = vec![false; n];
        for start in 0..n {
            if s[start][0] == 'o' {
                is_holiday[start] = true;
            }
        }
        for day in 1..w {
            let next_is_holiday: Vec<bool> = run_dp(day, &graph, &s, &is_holiday);
            is_holiday = next_is_holiday;
        }

        let mut is_ok: bool = false;
        'a: for vertex in 0..n {
            for &next in graph[vertex].iter() {
                if is_holiday[vertex] && s[next][0] == 'o' {
                    is_ok = true;
                    break 'a;
                }
            }
        }
        if is_ok {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn run_dp(
    day: usize,
    graph: &Vec<Vec<usize>>,
    s: &Vec<Vec<char>>,
    is_holiday: &Vec<bool>,
) -> Vec<bool> {
    let n: usize = graph.len();
    let mut next_is_holiday: Vec<bool> = vec![false; n];
    for vertex in 0..n {
        for &next in graph[vertex].iter() {
            if is_holiday[vertex] && s[next][day] == 'o' {
                next_is_holiday[next] = true;
            }
        }
    }
    return next_is_holiday;
}
