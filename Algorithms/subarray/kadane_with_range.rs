fn main() {
    let a = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let (max_sum, start, end) = max_subarray_with_range(&a);

    println!("Maximum Sum: {}", max_sum); // Output: 6
    println!("Range Indices: [{}, {})", start, end); // Output: [3, 7)
    println!("Subarray: {:?}", &a[start..end]); // Output: [4, -1, 2, 1]
}

fn max_subarray_with_range(array: &[i64]) -> (i64, usize, usize) {
    let n: usize = array.len();
}
