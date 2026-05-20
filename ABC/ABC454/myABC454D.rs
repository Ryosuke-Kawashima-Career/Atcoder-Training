use proconio::{input, marker::Chars};
// Q. Replace (xx) => xx and xx => (xx) | Is string A adjustable to string B?
// A. Check Canonical & Standard Form(標準形)
/* Key Points of Canonical Form in Competitive Programming (つまりParaphrase)
   - Definition: A "standard" representation of an object such that all equivalent objects map to it.
   - Purpose: Simplifies equivalence testing (hashing), reduces state space (DP), or breaks symmetry.

   Examples:
   1. Fractions: Represent a/b as (a/g, b/g) where g = gcd(a, b) and b > 0.
   2. Sets: Store as a sorted list to identify {3, 1, 2} == {1, 2, 3}.
   3. Cyclic Strings: Use the lexicographically smallest rotation (e.g., "bca" -> "abc").
   4. Graphs: Use a standardized DFS/BFS order or tree hashing (e.g., AHU algorithm).
   5. Matrix: Use Row Echelon Form (Linear Basis) to represent the span of vectors.
*/
fn solve() {
    input! {
        a_str: String,
        b_str: String,
    }

    if canonicalize(a_str) == canonicalize(b_str) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn canonicalize(s: String) -> String {
    /* Converts the input into its canonical form.
       After all, the idea itself depends on Greedy Algorithm and Conservative Property.
    */
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        stack.push(c);
        let n: usize = stack.len();
        if n >= 4 {
            // Converts (xx) => xx
            if stack[n - 4] == '('
                && stack[n - 3] == 'x'
                && stack[n - 2] == 'x'
                && stack[n - 1] == ')'
            {
                stack.pop();
                stack.pop();
                stack.pop();
                stack.pop();
                stack.push('x');
                stack.push('x');
            }
        }
    }
    return stack.iter().collect::<String>();
}

fn main() {
    input! { t: usize }
    for _ in 0..t {
        solve();
    }
}
