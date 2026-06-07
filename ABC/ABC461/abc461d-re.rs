use proconio::{input, marker::Chars};
// ABC461D
// Q. You are given the table of 0 and 1. What is the number of rectangle sub areas which have k-ko `1`s?
// A. Prefix 2D with Shakutori
fn main() {
    input!{h: usize, w: usize, k: usize, s: [Chars; h]}
    let prefix: Vec<Vec<usize>> = get_prefix2d(&s);
    let ans: usize = scan_col_by_col(&prefix, k);
    println!("{}", ans);
}

fn scan_col_by_col(prefix: &Vec<Vec<usize>>, k: usize) -> usize {
    let h: usize = prefix.len() - 1;
    let w: usize = prefix[0].len() - 1;
    let mut result: usize = 0;
    // Speed Optimization!
    let mut count_number_of_1: Vec<usize> = vec![0; h * w + 1];
    for h1 in 1..=h {
        for h2 in h1..=h {
            let mut touched_indexes: Vec<usize> = Vec::with_capacity(w+1);
            count_number_of_1[0] += 1;
            touched_indexes.push(0);
            // Cumulative sum of columns
            for col in 1..=w {
                let number_of_1: usize = prefix[h2][col] - prefix[h1-1][col];
                // This is the part of shakutori: S(h2) - S(h1) == k
                if number_of_1 >= k {
                    result += count_number_of_1[number_of_1 - k];
                }
                count_number_of_1[number_of_1] += 1;
                touched_indexes.push(number_of_1);
            }
            for &index in touched_indexes.iter() {
                count_number_of_1[index] = 0;
            }
        }
    }
    return result;
}

fn get_prefix2d(s: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let h: usize = s.len();
    let w: usize = s[0].len();
    let mut prefix: Vec<Vec<usize>> = vec![vec![0; w + 1]; h + 1];
    for i in 1..=h {
        for j in 1..=w {
            if s[i-1][j-1] == '1' {
                prefix[i][j] = 1;
            }
        }
    }
    for i in 1..=h {
        for j in 1..=w {
            prefix[i][j] = prefix[i - 1][j] + prefix[i][j];
        }
    }
    for i in 1..=h {
        for j in 1..=w {
            prefix[i][j] += prefix[i][j-1];
        }
    }
    return prefix;
}