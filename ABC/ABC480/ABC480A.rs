use proconio::{input, marker::Chars};

fn main() {
    input! {s: [Chars; 2]}
    let mut ia: usize = 2;
    let mut ja: usize = 2;
    let mut id: usize = 2;
    let mut jd: usize = 2;
    for i in 0..2 {
        for j in 0..2 {
            if s[i][j] == 'A' {
                ia = i;
                ja = j;
            }
            if s[i][j] == 'D' {
                id = i;
                jd = j;
            }
        }
    }
    if ia == id || ja == jd {
        println!("Yes");
    } else {
        println!("No");
    }
}
