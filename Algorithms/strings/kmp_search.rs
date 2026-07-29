use proconio::input;

fn main() {
    println!("=== Running KMP Search Test Cases ===");

    // Test Case 1: Basic single match
    let text1: Vec<char> = "hello world".chars().collect();
    let pattern1: Vec<char> = "world".chars().collect();
    println!("Test 1 (Single Match):");
    println!("  Text: \"hello world\", Pattern: \"world\"");
    println!("  Indices: {:?}", kmp_search(&text1, &pattern1));

    // Test Case 2: Multiple overlapping matches
    let text2: Vec<char> = "aaaaa".chars().collect();
    let pattern2: Vec<char> = "aaa".chars().collect();
    println!("Test 2 (Overlapping Matches):");
    println!("  Text: \"aaaaa\", Pattern: \"aaa\"");
    println!("  Indices: {:?}", kmp_search(&text2, &pattern2));

    // Test Case 3: Pattern longer than text (Boundary check test)
    let text3: Vec<char> = "ab".chars().collect();
    let pattern3: Vec<char> = "abc".chars().collect();
    println!("Test 3 (Pattern Longer Than Text):");
    println!("  Text: \"ab\", Pattern: \"abc\"");
    println!("  Indices: {:?}", kmp_search(&text3, &pattern3));

    // Test Case 4: No match found
    let text4: Vec<char> = "abcdef".chars().collect();
    let pattern4: Vec<char> = "xyz".chars().collect();
    println!("Test 4 (No Match):");
    println!("  Text: \"abcdef\", Pattern: \"xyz\"");
    println!("  Indices: {:?}", kmp_search(&text4, &pattern4));

    // Test Case 5: Exact match
    let text5: Vec<char> = "atcoder".chars().collect();
    let pattern5: Vec<char> = "atcoder".chars().collect();
    println!("Test 5 (Exact Match):");
    println!("  Text: \"atcoder\", Pattern: \"atcoder\"");
    println!("  Indices: {:?}", kmp_search(&text5, &pattern5));

    // Test Case 6: Multiple non-overlapping matches
    let text6: Vec<char> = "abcabcabc".chars().collect();
    let pattern6: Vec<char> = "abc".chars().collect();
    println!("Test 6 (Multiple Non-overlapping Matches):");
    println!("  Text: \"abcabcabc\", Pattern: \"abc\"");
    println!("  Indices: {:?}", kmp_search(&text6, &pattern6));
}

fn get_longest_prefix_suffix(pattern: &Vec<char>) -> Vec<usize> {
    /* Calculate the longest prefix of pattern[..i], which is also a suffix of pattern[..i] */
    let m: usize = pattern.len();
    if m == 0 {
        return Vec::new();
    }
    let mut longest_prefix_suffix: Vec<usize> = vec![0; m];
    let mut length: usize = 0;
    for i in 1..m {
        while length > 0 && pattern[i] != pattern[length] {
            length = longest_prefix_suffix[length - 1];
        }
        if pattern[i] == pattern[length] {
            length += 1;
        }
        longest_prefix_suffix[i] = length;
    }
    return longest_prefix_suffix;
}

fn kmp_search(text: &Vec<char>, pattern: &Vec<char>) -> Vec<usize> {
    /* Calculates the occurrences of `pattern` in `text`
    Returns: Vector of start indices of `pattern` in `text`.
    Time: O(|text| + |pattern|)
    */
    let n: usize = text.len();
    let m: usize = pattern.len();
    let mut indices: Vec<usize> = Vec::new();

    if m == 0 || n < m {
        return indices;
    }

    let lps: Vec<usize> = get_longest_prefix_suffix(pattern);
    let mut index_text: usize = 0;
    let mut index_pattern: usize = 0;
    while index_text < n {
        while index_text < n && index_pattern < m && text[index_text] == pattern[index_pattern] {
            index_text += 1;
            index_pattern += 1;
        }
        if index_pattern == m {
            indices.push(index_text - m);
            index_pattern = lps[index_pattern - 1];
        } else if index_pattern > 0 {
            index_pattern = lps[index_pattern - 1];
        } else {
            index_text += 1;
        }
    }
    return indices;
}
