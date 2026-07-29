fn main() {
    println!("=== Running Z-Algorithm Test Cases ===");

    // Test 1: Direct Z-function computation
    let str1: Vec<char> = "abacaba".chars().collect();
    println!("Test 1 (Z-array for \"abacaba\"):");
    println!("  String: \"abacaba\"");
    println!("  Z-array: {:?}", z_function(&str1));

    // Test 2: Basic Pattern Search
    let pattern1: Vec<char> = "world".chars().collect();
    let text1: Vec<char> = "hello world".chars().collect();
    println!("\nTest 2 (Pattern Search - Single Match):");
    println!("  Pattern: \"world\", Text: \"hello world\"");
    println!("  Start Indices: {:?}", search_pattern(&pattern1, '$', &text1));

    // Test 3: Overlapping Pattern Search
    let pattern2: Vec<char> = "ana".chars().collect();
    let text2: Vec<char> = "bananana".chars().collect();
    println!("\nTest 3 (Pattern Search - Overlapping Matches):");
    println!("  Pattern: \"ana\", Text: \"bananana\"");
    println!("  Start Indices: {:?}", search_pattern(&pattern2, '$', &text2));

    // Test 4: Pattern Longer Than Text
    let pattern3: Vec<char> = "atcoder".chars().collect();
    let text3: Vec<char> = "atc".chars().collect();
    println!("\nTest 4 (Pattern Search - Pattern Longer Than Text):");
    println!("  Pattern: \"atcoder\", Text: \"atc\"");
    println!("  Start Indices: {:?}", search_pattern(&pattern3, '$', &text3));

    // Test 5: No Match Found
    let pattern4: Vec<char> = "xyz".chars().collect();
    let text4: Vec<char> = "abcdef".chars().collect();
    println!("\nTest 5 (Pattern Search - No Match):");
    println!("  Pattern: \"xyz\", Text: \"abcdef\"");
    println!("  Start Indices: {:?}", search_pattern(&pattern4, '$', &text4));

    // Test 6: Exact Match
    let pattern5: Vec<char> = "rust".chars().collect();
    let text5: Vec<char> = "rust".chars().collect();
    println!("\nTest 6 (Pattern Search - Exact Match):");
    println!("  Pattern: \"rust\", Text: \"rust\"");
    println!("  Start Indices: {:?}", search_pattern(&pattern5, '$', &text5));
}

fn z_function(combined: &Vec<char>) -> Vec<usize> {
    /* Calculates z array (the lengths of the longest common prefixes (LCP) between the suffix starting at index i and the original string)
    Args:
        combined: `pattern` + `separator` + `text`
    Returns:
        z array: the longest common prefixes
    */
    let n: usize = combined.len();
    let mut left: usize = 0;
    let mut right: usize = 0;
    let mut z_array: Vec<usize> = vec![0; n];

    for i in 1..n {
        // use precalculated values
        if i <= right {
            let k: usize = i - left;
            z_array[i] = z_array[k].min(right + 1 - i);
        }
        // expand the z_window
        while i + z_array[i] < n && combined[z_array[i]] == combined[z_array[i] + i] {
            z_array[i] += 1;
        }
        // update the left and right of the z_window if the current z_window extends beyond the current right boundary
        if i + z_array[i] > right + 1 {
            right = i + z_array[i] - 1;
            left = i;
        }
    }
    z_array
}

fn search_pattern(pattern: &Vec<char>, separator: char, text: &Vec<char>) -> Vec<usize> {
    let mut combined = pattern.clone();
    combined.push(separator);
    combined.extend(text);
    let z_array: Vec<usize> = z_function(&combined);
    let pattern_len: usize = pattern.len();
    let mut start_indices: Vec<usize> = Vec::new();
    for i in pattern_len + 1..combined.len() {
        if z_array[i] == pattern_len {
            start_indices.push(i - pattern_len - 1);
        }
    }
    start_indices
}
