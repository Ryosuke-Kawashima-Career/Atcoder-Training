use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let h: usize = match iter.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };
    let w: usize = iter.next().unwrap().parse().unwrap();

    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(h);
    for _ in 0..h {
        let row = iter.next().unwrap().as_bytes().to_vec();
        grid.push(row);
    }

    let mut visited = vec![vec![false; w]; h];
    let mut ans = 0;

    let dr: [isize; 4] = [-1, 1, 0, 0];
    let dc: [isize; 4] = [0, 0, -1, 1];

    let mut queue = VecDeque::new();

    for r in 0..h {
        for c in 0..w {
            if grid[r][c] == b'.' && !visited[r][c] {
                // Discovered a new white connected component
                visited[r][c] = true;
                queue.push_back((r, c));

                let mut touches_border = false;

                while let Some((cr, cc)) = queue.pop_front() {
                    // Check if current cell touches any outermost boundary
                    if cr == 0 || cr == h - 1 || cc == 0 || cc == w - 1 {
                        touches_border = true;
                    }

                    // Traverse 4-directionally adjacent neighbors
                    for d in 0..4 {
                        let nr = cr as isize + dr[d];
                        let nc = cc as isize + dc[d];

                        if nr >= 0 && nr < h as isize && nc >= 0 && nc < w as isize {
                            let (nr, nc) = (nr as usize, nc as usize);
                            if grid[nr][nc] == b'.' && !visited[nr][nc] {
                                visited[nr][nc] = true;
                                queue.push_back((nr, nc));
                            }
                        }
                    }
                }

                // If no cell in this component is on the border, it is completely enclosed
                if !touches_border {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
