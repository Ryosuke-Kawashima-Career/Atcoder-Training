use proconio::input;

fn main() {
    input! {h: usize, w: usize, n: usize, q: usize, rc: [(usize, usize); n], queries: [(usize, usize, usize); q]}
    let mut graph: Vec<Vec<usize>> = vec![vec![0; w + 1]; h + 1];
    for &(row, col) in rc.iter() {
        graph[row][col] += 1;
    }
    let mut prefix_row: Vec<Vec<usize>> = graph.clone();
    let mut prefix_col: Vec<Vec<usize>> = graph.clone();
    for i in 0..h {
        for j in 0..w {
            prefix_row[i][j + 1] += prefix_row[i][j];
        }
    }
    for j in 0..w {
        for i in 0..h {
            prefix_col[i + 1][j] += prefix_col[i][j];
        }
    }
    let mut meta_prefix_row: Vec<usize> = vec![0; h + 1];
    let mut meta_prefix_col: Vec<usize> = vec![0; w + 1];
    for i in 0..h {
        meta_prefix_row[i + 1] += prefix_row[i][w];
    }
    for j in 0..w {
        meta_prefix_col[j + 1] += prefix_col[h][j];
    }
    for query in queries.iter() {
        if query.0 == 1 {
            let (u, d) = (query.1, query.2);
            let ans: usize = meta_prefix_row[d] - meta_prefix_row[u - 1];
            println!("{}", ans);
        } else {
            let (l, r) = (query.1, query.2);
            let ans: usize = meta_prefix_col[r] - meta_prefix_col[l - 1];
            println!("{}", ans);
        }
    }
}
