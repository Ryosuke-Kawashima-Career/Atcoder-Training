use proconio::input;
use std::collections::HashMap;

// ABC371E
// Q. Σ Σ(i=j<n) f(i, j) such that f(i, j) = the number of kinds of integers in {a_i, a_{i+1}, ..., a_j}.
// A. Contribution Method
fn main() {
    input! {n: usize, a: [i64; n]}
    // We want to calculate Sum_{i, j} f(i, j).
    // Instead of iterating ranges, we iterate each unique value occurring at index `k`.
    // Value `x` (appearing at `k`) contributes +1 to f(i, j) if:
    // i <= k <= j AND `x` does NOT appear in A[i..k] (before k, within the range).
    // The second condition means `first occurrence in the range`.
    //
    // Let `pre[k]` be the index of the previous occurrence of A[k] (or -1 if none).
    // Then for `x` at `k` to be counted, the range start `i` must be in (pre[k], k].
    // The range end `j` can be any index in [k, n-1].
    // So for a fixed `k`, valid `i` choices are: pre[k]+1 ..= k (count: k - pre[k]).
    // Valid `j` choices are: k ..= n-1 (count: n - k).
    // Contribution = (k - pre[k]) * (n - k).

    let mut last_index: HashMap<i64, isize> = HashMap::new();
    let mut ans: usize = 0;

    for (i, &val) in a.iter().enumerate() {
        let current_idx = i as isize;
        let prev_idx = *last_index.get(&val).unwrap_or(&-1);

        // Number of valid 'start' positions for a range to include this specific occurrence
        // as its "first" instance of `val`.
        let left_choices = (current_idx - prev_idx) as usize;

        // Number of valid 'end' positions for the range to validly include this occurrence.
        let right_choices = n - i;

        ans += left_choices * right_choices;

        last_index.insert(val, current_idx);
    }

    println!("{}", ans);
}
