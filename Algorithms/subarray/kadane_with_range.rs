fn main() {
    let a = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let (max_sum, start, end) = max_subarray_with_range(&a);

    println!("Maximum Sum: {}", max_sum); // Output: 6
    println!("Range Indices: [{}, {})", start, end); // Output: [3, 7)
    println!("Subarray: {:?}", &a[start..end]); // Output: [4, -1, 2, 1]
}

fn max_subarray_with_range(array: &[i64]) -> (i64, usize, usize) {
    /* Kadane's algorithm is about inherting the max values or restring from scratch */
    let n: usize = array.len();
    let mut max_sums: Vec<i64> = vec![0; n];
    let mut global_sums: Vec<i64> = vec![0; n];
    // [max_start max_end)
    let mut max_start: usize = 0;
    let mut max_end: usize = 1;
    let mut cur_start: usize = 0;
    max_sums[0] = array[0];
    global_sums[0] = array[0];

    for i in 1..n {
        if max_sums[i - 1] + array[i] > array[i] {
            max_sums[i] = max_sums[i - 1] + array[i];
        } else {
            max_sums[i] = array[i];
            cur_start = i;
        }
        if max_sums[i] > global_sums[i - 1] {
            max_start = cur_start;
            global_sums[i] = max_sums[i];
            max_end = i + 1;
        } else {
            global_sums[i] = global_sums[i - 1];
        }
    }
    return (global_sums[n - 1], max_start, max_end);
}
