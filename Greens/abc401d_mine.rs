use proconio::{input, marker::Chars};
const INF: isize = 1 << 30;
// ABC401D: n <= 2 * 10^5
// Q. the string is comprised of '.' and 'o' and '?'.
// Q. 'o' does not stand consecutively and its number is k.
// Q. '?' shows it is not sure whether it is '.' or 'o'.
/* Answer: index=iを中心として右と左をそれぞれ見る. = Bidirectional DP
S に含まれる o の数が K に等しいとき、答えはS の ? をすべて . に置き換えたものです。
以下では、それ以外の場合を考えます。
S の i 番目の文字が ? であると仮定します。
まず、i の左側の文字列について最大で M1 文字、i の右側の文字列について最大で M2 文字を o にできるとします。
このとき、M1 + M2 ≥ Kが成り立つならば、S の i 文字目を . にすることができます。
次に、i の左側の文字列の右端を . に固定した上で最大で M1′ 文字、i の右側の文字列の左端を . に固定した上で最大で M2′ 文字をo にできるとします。
このとき、M1′ + M2′ + 1 ≥ Kが成り立つならば、S の i 文字目を o にすることができます。
ここで、次の動的計画法を考えます。
dp[i][j] を「S の i 文字目までを考えたとき、
先頭の文字が j = 0 の場合は .、j = 1 の場合は o であるときのo の個数の最大値」と定義します。
この DP を S の左側および右側の両方向から計算することで、上記の M1、M2、M1′、M2′ をすべて求めることができ、その結果として本問題を解くことができます。
 */
fn main() {
    input! {n: usize, k: usize, s: Chars}
    let num_o: usize = count_o(&s);
    if num_o == k {
        let mut answer: Vec<char> = s.clone();
        for i in 0..n {
            if answer[i] == '?' {
                answer[i] = '.';
            }
        }
        println!("{}", answer.iter().collect::<String>());
        return;
    }

    // dp[i][j] := max 'o's in prefix/suffix of length i
    // j=0: ends with '.', j=1: ends with 'o'
    // Initialize with -INF to handle unreachable states (e.g., "oo")
    let mut dp_prefix: Vec<Vec<isize>> = vec![vec![-INF; 2]; n + 1];
    let mut dp_suffix: Vec<Vec<isize>> = vec![vec![-INF; 2]; n + 1];

    calc_dp(&s, &mut dp_prefix);

    // For suffix, we reverse the string, calculate DP, then results correspond to suffixes
    let s_rev: Vec<char> = s.iter().rev().copied().collect();
    calc_dp(&s_rev, &mut dp_suffix);

    let mut answer: Vec<char> = s.clone();
    for i in 0..n {
        if answer[i] == '?' {
            // max 'o's if s[i] is '.'
            // Prefix length i (ends in anything) + Suffix length n-(i+1) (starts with anything)
            // dp_suffix[len] corresponds to suffix of length len.
            // Suffer starting at i+1 has length n - 1 - i.
            // Note: dp_suffix was computed on reversed string, so dp_suffix[len] is correct for suffix of that length.
            let suffix_len = n - 1 - i;

            let max_if_dot = dp_prefix[i][0].max(dp_prefix[i][1])
                + dp_suffix[suffix_len][0].max(dp_suffix[suffix_len][1]);

            // max 'o's if s[i] is 'o'
            // Prefix ends in '.' (dp_prefix[i][0]) + 1 + Suffix starts with '.' (dp_suffix[suffix_len][0])
            // Note: suffix starts with '.' means in reversed string it ended with '.'
            let max_if_o = dp_prefix[i][0] + dp_suffix[suffix_len][0] + 1;

            if max_if_dot < k as isize {
                answer[i] = 'o';
            } else if max_if_o < k as isize {
                answer[i] = '.';
            }
        }
    }
    println!("{}", answer.iter().collect::<String>());
}

fn calc_dp(s: &Vec<char>, dp: &mut Vec<Vec<isize>>) {
    let n: usize = s.len();
    dp[0][0] = 0;
    dp[0][1] = 0; // Following C++ reference: set both to 0 at start
    for i in 1..=n {
        // if s[i-1] can be '.', update dp[i][0]
        if s[i - 1] != 'o' {
            dp[i][0] = dp[i - 1][0].max(dp[i - 1][1]);
        }
        // if s[i-1] can be 'o', update dp[i][1] (must come from '.')
        if s[i - 1] != '.' {
            dp[i][1] = dp[i - 1][0] + 1;
        }
    }
}

fn count_o(s: &Vec<char>) -> usize {
    s.iter().filter(|&c| *c == 'o').count()
}
