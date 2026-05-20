use proconio::input;
const INF: i64 = 1 << 60;
fn main() {
    input! {v: usize, e: usize, edges: [(usize, usize, i64); e]}
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; v];
    for &(from, to, cost) in edges.iter() {
        graph[from].push((to, cost));
    }
    let (has_negative, dist) = warshall_floyd(&graph);
    if has_negative {
        println!("NEGATIVE CYCLE");
    } else {
        print_dist(&dist);
    }
}

fn warshall_floyd(graph: &Vec<Vec<(usize, i64)>>) -> (bool, Vec<Vec<i64>>) {
    let n: usize = graph.len();
    let mut dist: Vec<Vec<i64>> = vec![vec![INF; n]; n];
    // initialization!!!
    for i in 0..n {
        dist[i][i] = 0;
        for &(to, cost) in graph[i].iter() {
            dist[i][to] = cost;
        }
    }
    // k -> i -> j
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }

    // check if there is a negative loop
    let mut has_negative: bool = false;
    for i in 0..n {
        if dist[i][i] < 0 {
            has_negative = true;
            break;
        }
    }
    (has_negative, dist)
}

fn print_dist(dist: &Vec<Vec<i64>>) {
    let n: usize = dist.len();
    for i in 0..n {
        for j in 0..n {
            if dist[i][j] == INF {
                print!("INF ");
            } else {
                print!("{} ", dist[i][j]);
            }
        }
        println!();
    }
}
