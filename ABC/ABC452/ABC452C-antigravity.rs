use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        // FIX 1: Length (A_i) is not an index, so it strictly needs standard `usize`
        ab: [(usize, Usize1); n],
        m: usize,
        s: [Chars; m]
    }

    // OPTIMIZATION: HashMap/Set is too slow for 200,000 strings.
    // Instead use a highly cached 3D array: valid[length][char_index][char]
    let mut valid = [[[false; 26]; 11]; 11];

    for string in 0..m {
        let length = s[string].len();
        for (i, &character) in s[string].iter().enumerate() {
            let c_idx = (character as u8 - b'a') as usize;
            valid[length][i][c_idx] = true;
        }
    }

    for spine in 0..m {
        // FIX 2: Spines physically require exactly length N
        if s[spine].len() != n {
            println!("No");
            continue;
        }

        let mut is_ok = true;
        for bone in 0..n {
            let (length, character_idx) = ab[bone];

            // FIX 3: We need the `bone`-th (i-th) character from the spine, not `B_i`
            let character = s[spine][bone];
            let c_idx = (character as u8 - b'a') as usize;

            if !valid[length][character_idx][c_idx] {
                is_ok = false;
                break;
            }
        }

        if is_ok {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
