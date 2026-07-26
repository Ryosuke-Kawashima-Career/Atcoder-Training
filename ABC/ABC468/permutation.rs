// Finds the next lexicographical permutation in-place
fn next_permutation<T: Ord>(slice: &mut [T]) -> bool {
    if slice.len() < 2 {
        return false;
    }

    // 1. Find the largest index i such that slice[i] < slice[i + 1]
    let mut i = slice.len() - 2;
    while i > 0 && slice[i] >= slice[i + 1] {
        i -= 1;
    }

    if i == 0 && slice[0] >= slice[1] {
        return false; // Last permutation reached
    }

    if slice[i] >= slice[i + 1] {
        // Handle edge case for index 0 when i didn't decrement
        slice.reverse();
        return false;
    }

    // 2. Find the largest index j greater than i such that slice[i] < slice[j]
    let mut j = slice.len() - 1;
    while slice[j] <= slice[i] {
        j -= 1;
    }

    // 3. Swap them
    slice.swap(i, j);

    // 4. Reverse the suffix from i + 1 to the end
    slice[i + 1..].reverse();
    true
}

fn main() {
    let mut data = vec![1, 2, 3];
    println!("Lexicographical Order:");
    println!("{:?}", data);

    while next_permutation(&mut data) {
        println!("{:?}", data);
    }
}
