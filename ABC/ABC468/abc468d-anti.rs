use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut ans: u64 = 0;

    // Odd-length substrings (center at index i)
    for i in 0..n {
        let mut mismatches = 0;
        let mut radius = 0;
        while i >= radius && i + radius < n {
            if bytes[i - radius] != bytes[i + radius] {
                mismatches += 1;
            }
            if mismatches > 1 {
                break;
            }
            ans += 1;
            radius += 1;
        }
    }

    // Even-length substrings (center between index i and i + 1)
    for i in 0..n.saturating_sub(1) {
        let mut mismatches = 0;
        let mut radius = 0;
        while i >= radius && i + 1 + radius < n {
            if bytes[i - radius] != bytes[i + 1 + radius] {
                mismatches += 1;
            }
            if mismatches > 1 {
                break;
            }
            ans += 1;
            radius += 1;
        }
    }

    println!("{}", ans);
}
