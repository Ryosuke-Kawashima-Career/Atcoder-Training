use proconio::input;
const MOD: u64 = 998244353;

fn count_trees(q: &[usize]) -> u64 {
    // Indices whose values are strictly decreasing from bottom to top.
    let mut stack = Vec::with_capacity(q.len());
    stack.push(0);
    let mut answer = 1;

    for i in 1..q.len() {
        while let Some(&j) = stack.last() {
            if q[j] > q[i] {
                break;
            }
            stack.pop();
        }

        // The parent must be at or after the nearest previous greater value.
        // If no such value exists, any of the i earlier vertices can be its parent.
        let choices = stack.last().map_or(i, |&j| i - j);
        answer = answer * choices as u64 % MOD;
        stack.push(i);
    }

    answer
}

fn main() {
    input! {n: usize, q: [usize; n]}
    println!("{}", count_trees(&q));
}
