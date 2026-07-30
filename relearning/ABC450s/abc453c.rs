use proconio::input;

fn main() {
    input! {n: usize, l: [i64; n]}
    let bits: usize = 1 << n;
    let mut max_traverse: usize = 0;
    for bit in 0..bits {
        let mut current_traverse: usize = 0;
        let mut current_pos: i64 = 0;
        for i in 0..n {
            let next_pos: i64 = if bit >> i & 1 == 0 {
                current_pos + l[i]
            } else {
                current_pos - l[i]
            };
            if (current_pos >= 0 && next_pos < 0) || (current_pos < 0 && next_pos >= 0) {
                current_traverse += 1;
            }
            current_pos = next_pos;
        }
        if max_traverse < current_traverse {
            max_traverse = current_traverse;
        }
    }
    println!("{}", max_traverse);
}
