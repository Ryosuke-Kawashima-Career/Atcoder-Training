use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, item_to_balls: [(Usize1, Usize1); n]}
    let mut balls_to_item: Vec<Vec<usize>> = vec![vec![]; n];
    for (i, &(ball_a, ball_b)) in item_to_balls.iter().enumerate() {
        balls_to_item[ball_a].push(i);
        balls_to_item[ball_b].push(i);
    }
    // (number_of_relevant_items, ball_idx)
    let mut ball_priorities: Vec<(usize, usize)> = Vec::new();
    for ball_idx in 0..n {
        ball_priorities.push((balls_to_item[ball_idx].len(), ball_idx));
    }
    ball_priorities.sort_unstable();
    if judge(&ball_priorities, &item_to_balls) {
        for i in 0..n {
            println!("{}", ball_priorities[i].1 + 1);
        }
    } else {
        println!("-1");
    }
}

fn judge(ball_priorities: &Vec<(usize, usize)>, item_to_balls: &Vec<(usize, usize)>) -> bool {
    let n = item_to_balls.len();
    let mut is_ball_black: Vec<bool> = vec![false; n];
    for &(_, ball_idx) in ball_priorities.iter() {
        let (ball_a, ball_b) = item_to_balls[ball_idx];
        if is_ball_black[ball_a] && is_ball_black[ball_b] {
            return false;
        }
        is_ball_black[ball_idx] = true;
    }
    true
}
