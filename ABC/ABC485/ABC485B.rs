use proconio::{input, marker::Chars};

fn main() {
    // Faucet -> Drain
    input!{n: usize, m: usize, s: [Chars; n]}
    let mut faucet_index: Option<usize> = None;
    let mut drain_index: Option<usize> = None;
    for i in 0..n {
        let mut is_all_x: bool = true;
        for j in 0..m {
            if s[i][j] == 'o' {
                is_all_x = false;
                break;
            }
        }
        if is_all_x {
            faucet_index = Some(i);
        }
    }
    for j in 0..m {
        let mut is_all_x: bool = true;
        for i in 0..n {
            if s[i][j] == 'o' {
                is_all_x = false;
                break;
            }
        }
        if is_all_x {
            drain_index = Some(j);
        }
    }
    match (faucet_index, drain_index) {
        (Some(faucet_index0), None) => {
            println!("Faucet {}", faucet_index0 + 1);
        }
        (None, Some(drain_index0)) => {
            println!("Drain {}", drain_index0 + 1);
        }
        _ => unreachable!()
    }
}
