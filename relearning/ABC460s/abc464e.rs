use proconio::input;
// Read the queries in the reversed manner.
fn main() {
    input! {h: usize, w: usize, q: usize, queries: [(usize, usize, char); q]}
    // let mut graph: Vec<Vec<char>> = vec![vec!['A'; w + 1]; h + 1];
    let mut last_indexes: Vec<Vec<isize>> = vec![vec![-1; w + 1]; h + 1];
    for query in 0..q {
        let (row, col, _) = queries[query];
        last_indexes[row][col] = query as isize;
    }
    for row in (1..=h).rev() {
        for col in (1..=w).rev() {
            last_indexes[row - 1][col] = last_indexes[row][col].max(last_indexes[row - 1][col]);
            last_indexes[row][col - 1] = last_indexes[row][col].max(last_indexes[row][col - 1]);
        }
    }
    for row in 1..=h {
        for col in 1..=w {
            let character: char = if last_indexes[row][col] == -1 {
                'A'
            } else {
                queries[last_indexes[row][col] as usize].2
            };
            print!("{}", character);
        }
        println!("");
    }
}
