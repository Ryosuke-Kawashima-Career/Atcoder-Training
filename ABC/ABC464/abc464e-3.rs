use proconio::input;
// ABC464E
// Q. We are given a grid. Rectangular ranges are updated into a new color. Answer the color of each cell in the final grid.
// A.  Imos to propagate the latest color to each cell.
fn main() {
    input! {h: usize, w: usize, q: usize, rcx: [(usize, usize, char); q]}
    let mut latest_operation: Vec<Vec<usize>> = vec![vec![0; w]; h];
    let mut colors: Vec<char> = Vec::new();
    for index in (0..q).rev() {
        let (row_edge, col_edge, color) = rcx[index];
        colors.push(color);
        for row in (0..row_edge).rev() {
            // if already updated
            if latest_operation[row][col_edge - 1] != 0 {
                break;
            }
            for col in (0..col_edge).rev() {
                if latest_operation[row][col] != 0 {
                    break;
                }
                latest_operation[row][col] = index + 1;
            }
        }
    }
    colors.push('A');
    let colors: Vec<char> = colors.into_iter().rev().collect();

    for row in 0..h {
        let mut row_chars = String::with_capacity(w);
        for col in 0..w {
            row_chars.push(colors[latest_operation[row][col]]);
        }
        println!("{}", row_chars);
    }
}
