use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let ns = s.len();
    let nt = t.len();

    // next_pos[i][c] stores the earliest index >= i where character `c` appears
    let mut next_pos = vec![[ns; 26]; ns + 1];

    // Fill the next_pos array pre-computed from right to left in strict O(|S|)
    for i in (0..ns).rev() {
        next_pos[i] = next_pos[i + 1];
        next_pos[i][(s[i] as u8 - b'a') as usize] = i;
    }

    let mut count: usize = 0;

    // Query each possible starting limit `L` of our dynamically evaluated substring
    for l in 0..ns {
        let mut curr = l;
        let mut found_all = true;

        // Greedily find exactly the earliest end point `R` that COMPLETES the entire Subsequence `T`
        for j in 0..nt {
            if curr == ns {
                found_all = false;
                break;
            }
            let c_idx = (t[j] as u8 - b'a') as usize;
            curr = next_pos[curr][c_idx];

            if curr == ns {
                found_all = false;
                break;
            }

            // Move string pointer 1 physical index safely forward evaluating the next nested character hunt
            curr += 1;
        }

        // FIX: Re-evaluating the physical boundary counting!
        if found_all {
            // If T was entirely mapped, `curr` realistically finished at R_min + 1.
            // So `curr - 1` strictly equals the absolute shortest span R_min perfectly containing T completely!
            // Any R boundaries strictly less than `R_min` CANNOT contain T.
            // Number of acceptable subsets without T natively equates to (R_min - L).
            count += (curr - 1) - l;
        } else {
            // T mathematically couldn't resolve its parameters from this L parameter!
            // Meaning absolutely ANY combination spanned extending to the right inherently avoids containing T!
            count += ns - l;
        }
    }

    println!("{}", count);
}
