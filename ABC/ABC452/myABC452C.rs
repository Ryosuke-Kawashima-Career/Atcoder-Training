use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {n: usize, ab: [(usize, Usize1); n], m: usize, s: [Chars; m]}
    // is_valid[length][cur_index][letter]
    let mut is_valid: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 26]; 11]; 11];
    for rib in 0..m {
        let length = s[rib].len();
        for i in 0..length {
            is_valid[length][i][s[rib][i] as usize - b'a' as usize] = true;
        }
    }

    let mut answer: Vec<String> = Vec::new();
    for query in 0..m {
        let mut is_ok = true;
        if s[query].len() != n {
            is_ok = false;
        } else {
            for rib in 0..n {
                let (length, index) = ab[rib];
                if !is_valid[length][index][s[query][index] as usize - b'a' as usize] {
                    is_ok = false;
                    break;
                }
            }
        }
        if is_ok {
            answer.push("Yes".to_string());
        } else {
            answer.push("No".to_string());
        }
    }
    println!("{}", answer.join("\n"));
}
