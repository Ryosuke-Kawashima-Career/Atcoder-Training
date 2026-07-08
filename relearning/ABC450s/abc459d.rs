use proconio::{input, marker::Chars};

fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {s: Chars}
        solve(&s);
    }
}

fn solve(s: &Vec<char>) {
    let n: usize = s.len();
    let mut ans: Vec<char> = vec!['?'; n];
    let mut odd_indexes: Vec<usize> = vec![];
    let mut even_indexes: Vec<usize> = vec![];
    for i in 0..n {
        if i % 2 == 1 {
            odd_indexes.push(i);
        } else {
            even_indexes.push(i);
        }
    }
    let mut count: Vec<usize> = vec![0; 26];
    for i in 0..n {
        count[s[i] as usize - 'a' as usize] += 1;
    }
    let mut alphas: Vec<usize> = (0..26).collect();
    alphas.sort_by(|x, y| count[*y].cmp(&count[*x]));
    if count[alphas[0]] > even_indexes.len() {
        println!("No");
        return;
    }
    for i in 0..26 {
        if count[alphas[i]] == 0 {
            continue;
        }
        while let Some(odd_index) = odd_indexes.pop() {
            if count[alphas[i]] == 0 {
                break;
            }
            ans[odd_index] = (alphas[i] as u8 + 'a' as u8) as char;
            count[alphas[i]] -= 1;
            if count[alphas[i]] == 0 {
                break;
            }
        }
        if count[alphas[i]] == 0 {
            continue;
        }
        while let Some(even_index) = even_indexes.pop() {
            if count[alphas[i]] == 0 {
                break;
            }
            ans[even_index] = (alphas[i] as u8 + 'a' as u8) as char;
            count[alphas[i]] -= 1;
            if count[alphas[i]] == 0 {
                break;
            }
        }
    }
    println!("Yes");
    for i in 0..n {
        print!("{}", ans[i]);
    }
    println!();
}
