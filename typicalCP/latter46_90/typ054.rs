use proconio::{input, marker::Usize1};
use std::collections::VecDeque;
// Check only relevant edges and ignore unnecessary ones.
fn main() {
    // n: researchers, m: papers
    input! {n: usize, m: usize}
    let mut coauthors: Vec<Vec<usize>> = vec![];
    let mut belonging_groups: Vec<Vec<usize>> = vec![vec![]; n];
    for group_id in 0..m {
        input! {k: usize, r: [Usize1; k]}
        for &person in &r {
            belonging_groups[person].push(group_id);
        }
        coauthors.push(r);
    }
    let mut points: Vec<isize> = vec![-1; n];
    points[0] = 0;
    bfs(0, &coauthors, &belonging_groups, &mut points);
    for &point in points.iter() {
        println!("{}", point);
    }
}

fn bfs(
    start: usize,
    coauthors: &[Vec<usize>],
    belonging_groups: &[Vec<usize>],
    points: &mut [isize],
) {
    let mut queue = VecDeque::new();
    let mut visited_groups = vec![false; coauthors.len()];
    queue.push_back(start);

    while let Some(person) = queue.pop_front() {
        for &group_id in &belonging_groups[person] {
            if visited_groups[group_id] {
                continue;
            }
            // BFS reaches this paper through an author with minimum distance.
            visited_groups[group_id] = true;
            for &coauthor in &coauthors[group_id] {
                if points[coauthor] == -1 {
                    points[coauthor] = points[person] + 1;
                    queue.push_back(coauthor);
                }
            }
        }
    }
}
