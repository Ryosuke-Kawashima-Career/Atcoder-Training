use proconio::{input, marker::Chars};

fn main() {
    input! {h: usize, w: usize, s: [Chars; h]}
    let mut ans: usize = 0;
    for h1 in 0..h {
        for h2 in h1..h {
            for w1 in 0..w {
                for w2 in w1..w {
                    for i in h1..=h2 {
                        for j in w1..=w2 {
                            if s[i][j] == s[h1 + h2 - i][w1 + w2 - j] {
                                ans += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
