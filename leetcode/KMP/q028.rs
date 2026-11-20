impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let lps = get_lps(&needle);
        let result: i32 = run_kmp(&haystack, &needle, &lcp);
        return result;
    }

    fn get_lps(pattern: &str) -> Vec<usize> {
        let m: usize = pattern.len();
        let mut lcp: Vec<usize> = vec![0; m];
        let mut length: usize = 0;
        for i in 1..m {
            if pattern[i] == pattern[length] {
                length += 1;
                lcp[i] = length;
            } else {
                if length > 0 {
                    length = lcp[length-1];
                }
            }
        }
    }

    fn run_kmp(target: &str, pattern: &str, lps: &Vec<usize>) -> i32 {
        let n: usize = target.len();
        let m: usize = pattern.len();
        
    }
}