use proconio::input;

fn main() {
    input! {h: usize, w: usize}
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; w]; h];
    for i in 0..h {
        for j in 0..w {
            if i == 0 || i == h - 1 || j == 0 || j == w - 1 {
                grid[i][j] = '#';
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            print!("{}", grid[i][j]);
        }
        println!();
    }
}
