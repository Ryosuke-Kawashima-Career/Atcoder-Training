use proconio::input;
// ABC479C
// Q. Get the maximum value within the total weight less than and equal to w.
// A. meet in the middle
fn main() {
    input! {n: usize, w: usize, ab: [(i64, usize); n]}
    let n1: usize = n / 2;
    let n2: usize = n - n1;
    let deliciousness_left: Vec<(i64, usize)> = enum_subsets(&ab[0..n1]);
    let mut deliciousness_right: Vec<(i64, usize)> = enum_subsets(&ab[n1..n]);
    deliciousness_right.sort_unstable_by_key(|&(_, weight)| weight);
    let mut best_value_right: Vec<i64> = vec![0; deliciousness_right.len() + 1];
    for i in 0..deliciousness_right.len() {
        best_value_right[i + 1] = best_value_right[i].max(deliciousness_right[i].0);
    }
    let mut ans: i64 = 0;
    for &(value_left, weight_left) in deliciousness_left.iter() {
        if w < weight_left {
            continue;
        }
        let max_value_with_weight_limit: i64 =
            binary_search(w - weight_left, &deliciousness_right, &best_value_right);
        ans = ans.max(value_left + max_value_with_weight_limit)
    }
    println!("{ans}");
}

fn binary_search(w: usize, values_weights: &Vec<(i64, usize)>, best_values: &Vec<i64>) -> i64 {
    let mut low: usize = 0;
    let mut high: usize = values_weights.len();
    while high - low > 1 {
        let mid: usize = (low + high) / 2;
        let weight_mid: usize = values_weights[mid].1;
        if weight_mid <= w {
            low = mid;
        } else {
            high = mid;
        }
    }
    best_values[low]
}

fn enum_subsets(item: &[(i64, usize)]) -> Vec<(i64, usize)> {
    let n: usize = item.len();
    let mut result: Vec<(i64, usize)> = Vec::new();
    for bit in 0..(1 << n) {
        let mut total_value: i64 = 0;
        let mut total_weight: usize = 0;
        for i in 0..n {
            if bit >> 1 & 1 == 1 {
                total_value += item[i].0;
                total_weight += item[i].1;
            }
        }
        result.push((total_value, total_weight));
    }
    result
}
