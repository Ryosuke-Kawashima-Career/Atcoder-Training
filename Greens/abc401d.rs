use proconio::{input, marker::Chars};
use std::cmp::{max, min};
// ABC401D: n <= 2 * 10^5
// Q. the string is comprised of '.' and 'o' and '?'.
// Q. 'o' does not stand consecutively and its number is k.
// Q. '?' shows it is not sure whether it is '.' or 'o'.
// A. Interval DP: Prefix and Suffix = 前から計算する場合と後ろから計算する場合
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    // pre[i]: Valid ranges of 'o' counts for the prefix s[0..i]
    // .0: Ends with '.'
    // .1: Ends with 'o'
    let mut pre = vec![(None, None); n + 1];
    pre[0] = (Some((0, 0)), None);

    for i in 0..n {
        let (p_dot, p_o) = pre[i];

        // '.' で終わるから.0を更新する
        // Next char `s[i]` is '.'
        if s[i] == '.' || s[i] == '?' {
            // Can come from '.' or 'o'
            pre[i + 1].0 = merge(p_dot, p_o);
        }

        // 'o' で終わるから.1を更新する
        // Next char `s[i]` is 'o'
        if s[i] == 'o' || s[i] == '?' {
            // Must come from '.'
            pre[i + 1].1 = add(p_dot, 1);
        }
    }

    // suf[i]: Valid ranges of 'o' counts for the suffix s[i..n]
    // .0: Starts with '.'
    // .1: Starts with 'o'
    let mut suf = vec![(None, None); n + 1];
    // Base case: empty suffix at n
    suf[n] = (Some((0, 0)), None);

    for i in (0..n).rev() {
        let (s_dot, s_o) = suf[i + 1];

        // Current char `s[i]` is '.'
        if s[i] == '.' || s[i] == '?' {
            // Next can be '.' or 'o'
            suf[i].0 = merge(s_dot, s_o);
        }

        // Current char `s[i]` is 'o'
        if s[i] == 'o' || s[i] == '?' {
            // Next must be '.'
            suf[i].1 = add(s_dot, 1);
        }
    }

    let mut ans = String::with_capacity(n);
    for i in 0..n {
        let mut can_dot = false;
        // Check if `s[i]` can be '.'
        if s[i] == '.' || s[i] == '?' {
            // Combine all valid prefixes ending at i (compatible with . at i)
            let p_range = merge(pre[i].0, pre[i].1);
            // Combine all valid suffixes starting at i+1 (compatible with . at i)
            let s_range = merge(suf[i + 1].0, suf[i + 1].1);

            if check(p_range, s_range, k) {
                can_dot = true;
            }
        }

        let mut can_o = false;
        // Check if `s[i]` can be 'o'
        if s[i] == 'o' || s[i] == '?' {
            // Prefix must end in '.'
            let p_range = pre[i].0;
            // Suffix must start with '.'
            let s_range = suf[i + 1].0;

            // Current 'o' adds 1 to the count
            if check_add(p_range, s_range, 1, k) {
                can_o = true;
            }
        }

        if can_dot && can_o {
            ans.push('?');
        } else if can_dot {
            ans.push('.');
        } else if can_o {
            ans.push('o');
        } else {
            // This case should not be reachable given the problem constraints (X is non-empty)
            ans.push('!');
        }
    }

    println!("{}", ans);
}

// Range: (min possible number, max possible number)
type Range = Option<(usize, usize)>;

fn merge(a: Range, b: Range) -> Range {
    match (a, b) {
        (Some(ra), Some(rb)) => Some((min(ra.0, rb.0), max(ra.1, rb.1))),
        (Some(ra), None) => Some(ra),
        (None, Some(rb)) => Some(rb),
        (None, None) => None,
    }
}

fn add(a: Range, v: usize) -> Range {
    if let Some((min_v, max_v)) = a {
        Some((min_v + v, max_v + v))
    } else {
        None
    }
}

fn check(a: Range, b: Range, k: usize) -> bool {
    if let (Some((amin, amax)), Some((bmin, bmax))) = (a, b) {
        let min_total = amin + bmin;
        let max_total = amax + bmax;
        k >= min_total && k <= max_total
    } else {
        false
    }
}

fn check_add(a: Range, b: Range, v: usize, k: usize) -> bool {
    if let (Some((amin, amax)), Some((bmin, bmax))) = (a, b) {
        let min_total = amin + bmin + v;
        let max_total = amax + bmax + v;
        k >= min_total && k <= max_total
    } else {
        false
    }
}
