use proconio::input;

fn main() {
    input!{n: usize, m: usize, ab: [(usize, usize); m]}
    let mut candidate_first_votes: Vec<usize> = vec![0; n+1];
    let mut to_seconds: Vec<Vec<usize>> = vec![vec![]; n+1];
    for &(a, b) in ab.iter() {
        candidate_first_votes[a] += 1;
        to_seconds[a].push(b);
    }
    let mut top_candidate: usize = 1;
    let mut second_candidate: usize = 0;
    for i in 2..=n {
        if candidate_first_votes[top_candidate] < candidate_first_votes[i] {
            second_candidate = top_candidate;
            top_candidate = i;
        } else {
            if second_candidate == 0 {
                second_candidate = i;
            } else if candidate_first_votes[second_candidate] < candidate_first_votes[i] {
                second_candidate = i;
            }
        }
    }
    for i in 1..=n {
        to_seconds[i].sort();
    }
    for k in 1..=n {
        if k == top_candidate {
            let mut ans: usize = second_candidate;
            for &to_person in to_seconds[k].iter() {
                candidate_first_votes[to_person] += 1;
                if candidate_first_votes[ans] < candidate_first_votes[to_person] {
                    ans = to_person;
                } else if candidate_first_votes[ans] == candidate_first_votes[to_person] {
                    ans = ans.min(to_person);
                }
            }
            for &to_person in to_seconds[k].iter() {
                candidate_first_votes[to_person] -= 1;
            }
            println!("{}", ans);
        } else {
            let mut ans: usize = top_candidate;
            for &to_person in to_seconds[k].iter() {
                candidate_first_votes[to_person] += 1;
                if candidate_first_votes[ans] < candidate_first_votes[to_person] {
                    ans = to_person;
                } else if candidate_first_votes[ans] == candidate_first_votes[to_person] {
                    ans = ans.min(to_person);
                }
            }
            for &to_person in to_seconds[k].iter() {
                candidate_first_votes[to_person] -= 1;
            }
            println!("{}", ans);
        }
    }
}
