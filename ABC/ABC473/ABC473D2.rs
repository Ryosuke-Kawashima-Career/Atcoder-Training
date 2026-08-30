use proconio::input;

// ABC473D - Coefficient Stair
// Q. Enumerate sequences such that the sum of i x a_i == k in lexicographical order
// A. Recursive DFS with O(1) determination of the last element
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut memo: Vec<usize> = Vec::new();
    lexico(1, n, k, &mut memo);
}

fn lexico(start: usize, n: usize, k: usize, memo: &mut Vec<usize>) -> bool {
    if n == 1 {
        if k % start == 0 {
            let last_element = k / start;
            memo.push(last_element);
            println!(
                "{}",
                memo.iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
            memo.pop();
            return true;
        } else {
            return false;
        }
    }

    // Avoid returning early on the first found solution
    let mut found = false;
    for num in 0..=k {
        if num * start > k {
            break;
        }
        memo.push(num);
        let next_k: usize = k - (num * start);
        if lexico(start + 1, n - 1, next_k, memo) {
            found = true;
        }
        memo.pop();
    }

    return found;
}
