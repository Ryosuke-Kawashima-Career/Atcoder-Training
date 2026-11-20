impl Solution {
    pub fn min_starting_index(s: String, pattern: String) -> i32 {
        let n = s.len();
        let m = pattern.len();
        if m > n {
            return -1;
        }

        // Forward Z-array: LCP of `pattern` and `s[i..]`
        let fwd_str = format!("{}|{}", pattern, s);
        let z_fwd = Self::get_zarray(&fwd_str);

        // Backward Z-array: LCS of `pattern` and `s[i..i+m]`
        let rev_pattern: String = pattern.chars().rev().collect();
        let rev_s: String = s.chars().rev().collect();
        let bwd_str = format!("{}|{}", rev_pattern, rev_s);
        let z_bwd = Self::get_zarray(&bwd_str);

        // Substring s[i..i+m] differs from pattern in at most 1 char iff LCP + LCS >= m - 1
        for i in 0..=(n - m) {
            let lcp = z_fwd[m + 1 + i];
            let lcs = z_bwd[m + 1 + (n - i - m)];
            if lcp + lcs >= m - 1 {
                return i as i32;
            }
        }
        -1
    }

    pub fn get_zarray(s: &str) -> Vec<usize> {
        let bytes = s.as_bytes();
        let length = bytes.len();
        let mut z_array = vec![0; length];
        // Half-open Z-box [z_box_left, z_box_right) where s[l..r] == s[0..r-l]
        let mut z_box_left: usize = 0;
        let mut z_box_right: usize = 0;

        for i in 1..length {
            // If i is inside the current Z-box, reuse the previously computed Z-value
            if i < z_box_right {
                z_array[i] = (z_box_right - i).min(z_array[i - z_box_left]);
            }
            // Try to expand the match from i + z_array[i]
            while i + z_array[i] < length && bytes[z_array[i]] == bytes[i + z_array[i]] {
                z_array[i] += 1;
            }
            // Update the Z-box if the new match reaches further right
            if i + z_array[i] > z_box_right {
                z_box_left = i;
                z_box_right = i + z_array[i];
            }
        }
        z_array
    }
}
