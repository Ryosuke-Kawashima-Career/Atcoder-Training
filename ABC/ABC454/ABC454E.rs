use proconio::{input, marker::Usize1};
const N1: usize = 1usize.wrapping_neg();
// U, D, L, R
const D4: [(usize, usize); 4] = [(N1, 0), (1, 0), (0, N1), (0, 1)];
fn num_to_dir(num: usize) -> char {
    match num {
        0 => 'U',
        1 => 'D',
        2 => 'L',
        3 => 'R',
        _ => {
            unreachable!();
        }
    }
}
fn main() {
    input! {t: usize}
    let mut answer: Vec<(bool, Vec<char>)> = Vec::new();
    for _case in 0..t {
        input! {n: usize, a: Usize1, b: Usize1}
        let solution: Vec<(bool, Vec<char>)> = solve(a, b);
        answer.push(solution);
    }

    for (flag, s) in answer {
        if flag {
            println!("Yes");
            for c in s {
                print!("{}", c);
            }
            println!();
        } else {
            println!("No");
        }
    }
}

fn solve(a: usize, b: usize) -> (bool, Vec<char>) {
    /* Judge whether there is a path which allows to start from (0, 0) to the end (a, b) without revisiting any nodes.
    The number of operations is exactly N^2-2
    Args:
        (a, b): the end node
    Returns:
        (flag, path): flag is true if there is a path, false otherwise. path is the path if flag is true, empty vector otherwise.
     */
    let distance = a + b;
    if n * n - 2 - distance % 2 == 1 {
        return (false, vec![]);
    }
    let mut path: Vec<char> = Vec::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; n];
    let mut curr = (0, 0);
    visited[curr.0][curr.1] = true;
    if dfs(a, b, curr, &mut visited, &mut path) {
        return (true, path);
    } else {
        return (false, vec![]);
    }
}

fn dfs(
    a: usize,
    b: usize,
    curr: (usize, usize),
    visited: &mut Vec<Vec<bool>>,
    path: &mut Vec<char>,
) -> bool {
    if curr == (a, b) {
        return true;
    }
    for i in 0..4 {
        let next = (curr.0 + D4[i].0, curr.1 + D4[i].1);
        if next.0 < n && next.1 < n && !visited[next.0][next.1] {
            visited[next.0][next.1] = true;
            path.push(num_to_dir(i));
            let (flag, path) = dfs(a, b, next, visited, path);
            if flag {
                return (true, path);
            }
            visited[next.0][next.1] = false;
            path.pop();
        }
    }
    (false, vec![])
}
