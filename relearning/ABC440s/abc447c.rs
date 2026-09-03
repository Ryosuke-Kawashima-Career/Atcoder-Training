use proconio::{input, marker::Chars};
const INF: usize = 1 << 60;
fn main() {
    input! {s: Chars, t: Chars}
    let ns: usize = s.len();
    let nt: usize = t.len();
    let mut cost: usize = 0;
    let mut index_s: usize = 0;
    let mut index_t: usize = 0;
    while index_s < ns && index_t < nt {
        if s[index_s] == t[index_t] {
            index_s += 1;
            index_t += 1;
        } else {
            if s[index_s] == 'A' {
                cost += 1;
                index_s += 1;
            } else if t[index_t] == 'A' {
                cost += 1;
                index_t += 1;
            } else {
                cost = INF;
                break;
            }
        }
    }
    for i in index_s..ns {
        if s[i] != 'A' {
            cost = INF;
            break;
        } else {
            cost += 1;
        }
    }
    for i in index_t..nt {
        if t[i] != 'A' {
            cost = INF;
            break;
        } else {
            cost += 1;
        }
    }
    if cost == INF {
        println!("-1");
    } else {
        println!("{}", cost);
    }
}
