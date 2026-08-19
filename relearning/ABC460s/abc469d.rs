use proconio::{input, marker::Usize1};
// sweepline algorithm
fn main() {
    input!{n: usize, m: usize, ab: [(Usize1, Usize1); m]}
    let mut count: usize = 0;
    let mut player_to_tournament: Vec<Vec<usize>> = vec![vec![]; n];
    for (i, &(a, b)) in ab.iter().enumerate() {
        player_to_tournament[a].push(i);
        player_to_tournament[b].push(i);
    }
    update(ab[0].0, &player_to_tournament, m, &mut count);
    update(ab[0].1, &player_to_tournament, m, &mut count);
    let mut tournaments: Vec<bool> = vec![false; m];
    for &tournament in player_to_tournament[ab[0].0].iter() {
        tournaments[tournament] = true;
    }
    for &tournament in player_to_tournament[ab[0].1].iter() {
        tournaments[tournament] = true;
    }
    if tournaments.iter().all(|&t| t) {
        count -= 1;
    }
    println!("{}", count);
}

fn update(player0: usize, player_to_tournament: &Vec<Vec<usize>>, m: usize, count: &mut usize) {
    let n: usize = player_to_tournament.len();
    // choose player 1
    let mut tournaments: Vec<bool> = vec![false; m];
    for tournament in player_to_tournament[player0].iter() {
        tournaments[*tournament] = true;
    }
    for player in 0..n {
        if player == player0 {
            continue;
        }
        for &tournament in player_to_tournament[player].iter() {
            tournaments[tournament] = true;
        }
        if tournaments.iter().all(|&t| t) {
            *count += 1;
        }
        for &tournament in player_to_tournament[player].iter() {
            tournaments[tournament] = false;
        }
        for tournament in player_to_tournament[player0].iter() {
            tournaments[*tournament] = true;
        }
    }
}
