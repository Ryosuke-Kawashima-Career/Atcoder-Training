use proconio::input;
// bidirectional DP
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let lis_left: Vec<usize> = get_lis(&a);
    let lis_right: Vec<usize> = get_lis(&a.iter().rev().cloned().collect::<Vec<usize>>());
    let mut ans: usize = 0;
    for i in 0..n {
        let length_mountain: usize = lis_left[i] + lis_right[n - 1 - i];
        ans = ans.max(length_mountain);
    }
    println!("{}", ans - 1);
}

fn get_lis(arr: &[usize]) -> Vec<usize> {
    let n: usize = arr.len();
    let mut result: Vec<usize> = vec![1; n];
    // max_top[length] := the maximum top value of LIS with length 'length'
    let mut max_top: Vec<usize> = vec![usize::MAX; n + 1];
    // Check the initialization method
    max_top[0] = 0;
    for i in 0..n {
        let length: usize = max_top.partition_point(|&tail| tail < arr[i]);
        if length <= n {
            max_top[length] = arr[i];
        }
        result[i] = result[i].max(length);
    }
    result
}
