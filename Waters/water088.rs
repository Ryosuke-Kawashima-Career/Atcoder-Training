use proconio::input;
// JOI 2008 本選 1 - 碁石ならべ
// Q. Reversi with Go's stones
// A. Sandwiching flips the stones between the two stones. (Color1, Color2) <- Color1 => (Color1 only)
// A. Analyze three parts: Second last, last stone, added stone
fn main() {
    // 0: white, 1: black
    input! {
        n: usize,
        colors: [usize; n],
    }

    // Use a stack of (color, count) for Run-Length Encoding
    let mut stack: Vec<(usize, usize)> = Vec::new();

    for (idx, &c) in colors.iter().enumerate() {
        let i = idx + 1; // 1-indexed for the rules

        if stack.is_empty() {
            stack.push((c, 1));
            continue;
        }

        let last_idx = stack.len() - 1;
        let (top_color, top_count) = stack[last_idx];

        if i % 2 != 0 {
            // Rule for ODD: Just place it
            if top_color == c {
                stack[last_idx].1 += 1;
            } else {
                stack.push((c, 1));
            }
        } else {
            // Rule for EVEN:
            if top_color == c {
                // Same color: Just place it (merge)
                stack[last_idx].1 += 1;
            } else {
                // Different color: Change rightmost block and merge with previous
                let popped_count = stack.pop().unwrap().1;
                if stack.is_empty() {
                    // Only one block existed, just flip color and increment
                    stack.push((c, popped_count + 1));
                } else {
                    // Merge flipped block into the previous block of the same color
                    let new_last = stack.len() - 1;
                    stack[new_last].1 += popped_count + 1;
                }
            }
        }
    }

    // Count white stones (0)
    let ans: usize = stack
        .iter()
        .filter(|&&(color, _)| color == 0)
        .map(|&(_, count)| count)
        .sum();

    println!("{}", ans);
}
