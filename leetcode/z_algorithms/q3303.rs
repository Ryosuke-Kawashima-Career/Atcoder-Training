impl Solution {
    pub fn min_starting_index(s: String, pattern: String) -> i32 {
        let n: usize = s.len();
        let m: usize = m.len();
        let working_string: String = format!("{}|{}", s, pattern);
        let z_array: Vec<usize> = get_zarray(&working_string);
        let n_occurs: i32 = z_algorithm(n, m, &z_array, &working_string);
        return n_occurs;
    }

    pub fn get_zarray(s: &str) -> Vec<usize> {
        let length: usize = s.len();
        let mut z_arary: Vec<usize> = vec![0; length];
        let mut z_box_left: usize = 0;
        let mut z_box_right: usize = 0;
        for i in 0..length {
            // Is i within the z-box
            if i <= z_box_right {
                z_arary[i] = (z_box_right - i + 1).min(z_arary[i - z_box_left]);
            }
            // Try to expand the z-box
            while (i + z_arary[i] < length && s[z_arary[i]] == s[i+z_arary[i]]) {
                z_arary[i] += 1;
            }
            // update when the z-box is extended
            if (i + z_arary[i] - 1 > z_box_right) {
                z_box_left = i;
                z_box_right = l + z_box_left[i] - 1;
            }
        }
        return z;
    }

    pub fn z_algorithm(n: usize, m: usize, z_array: &Vec<usize>, working_string: &str) -> i32 {

    }
}