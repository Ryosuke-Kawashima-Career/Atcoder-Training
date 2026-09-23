use proconio::input;

fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {n: usize, rows: [usize; n]}
        let min_row: usize = *rows.iter().min().unwrap();
        let mut min_indexes: Vec<usize> = vec![];
        for i in 0..n {
            if rows[i] == min_row {
                min_indexes.push(i)
            }
        }
        let dist: Vec<usize> = bfs_1d(&min_indexes, &rows);
        let mut ans: usize = 0;
        for i in 0..n {
            let destination: usize = min_row + dist[i];
            let diff: usize = rows[i].saturating_sub(destination);
            ans += diff;
        }
        println!("{}", ans);
    }
}

fn bfs_1d(starts: &[usize], rows: &[usize]) -> Vec<usize> {
    let n: usize = rows.len();
    let mut dist: Vec<usize> = vec![usize::MAX; n];
    let mut que = std::collections::VecDeque::new();
    for &start in starts.iter() {
        dist[start] = 0;
        que.push_back(start);
    }

    while let Some(i) = que.pop_front() {
        let cur_row = rows[i];
        let cur_dist = dist[i];
        if i > 0 {
            let next_left: usize = i - 1;
            if dist[next_left] > dist[i] + 1 {
                dist[next_left] = cur_dist + 1;
                que.push_back(next_left);
            }
        }
        if i < n - 1 {
            let next_right: usize = i + 1;
            if dist[next_right] > dist[i] + 1 {
                dist[next_right] = cur_dist + 1;
                que.push_back(next_right);
            }
        }
    }
    return dist;
}
