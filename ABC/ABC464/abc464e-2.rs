use proconio::input;
// ABC464E
// Q. We are given a grid. Rectangular ranges are updated into a new color. Answer the color of each cell in the final grid.
// A.  Imos to propagate the latest color to each cell.
fn main() {
    input! {h: usize, w: usize, q: usize, rcx: [(usize, usize, char); q]}
    let mut colors: Vec<char> = vec!['A'];
    // imos[i]: the latest operation(color) index that affects the cell i
    let mut imos: Vec<Vec<usize>> = vec![vec![0; w]; h];
    for &(row, col, color) in rcx.iter() {
        colors.push(color);
        imos[row - 1][col - 1] = colors.len() - 1;
    }
    // propagate imos
    for row in (0..h).rev() {
        for col in (0..w).rev() {
            if row > 0 {
                imos[row - 1][col] = imos[row][col].max(imos[row - 1][col]);
            }
            if col > 0 {
                imos[row][col - 1] = imos[row][col].max(imos[row][col - 1]);
            }
        }
    }
    for row in 0..h {
        for col in 0..w {
            let color: char = colors[imos[row][col]];
            print!("{}", color);
        }
        println!("");
    }
}
