use proconio::input;
// brain storm
/*
# Brute force attach
set the minimum index -> find the minimum gap between MAX - MIN -> O(N^2)

# Sliding window
attach minimum and maximum and if the gap is smaller than k -> OK
 */
fn main() {
    input! {n: usize, k: usize, a: [usize; n]}
    let max_a: usize = *a.iter().max().unwrap();
    let mut lower_bound: Vec<usize> = a.clone();
    let mut upper_bound: Vec<usize> = a.clone();
    for i in 0..n {
        while lower_bound[i] + k <= max_a {
            lower_bound[i] += k;
        }
        while upper_bound[i] <= max_a {
            upper_bound[i] += k;
        }
    }
    let candidate1: usize = case1(&lower_bound, &upper_bound, max_a);
    let candidate2: usize = case2(&lower_bound, max_a);
    let candidate3: usize = case3(&upper_bound, max_a);
    let ans: usize = candidate1.min(candidate2.min(candidate3));
    println!("{ans}")
}

fn case1(lower_bound: &Vec<usize>, upper_bound: &Vec<usize>, max_a: usize) -> usize {
    // max_a is the middle
    let n: usize = lower_bound.len();
    let mut res: usize = 0;
    for i in 0..n {
        let diff1: usize = upper_bound[i] - max_a;
        let diff2: usize = max_a - lower_bound[i];
        res = res.max(diff1.min(diff2));
    }
    res
}

fn case2(lower_bound: &Vec<usize>, max_a: usize) -> usize {
    // min_a is the max
    let n: usize = lower_bound.len();
    let mut res: usize = 0;
    for i in 0..n {
        let diff: usize = max_a - lower_bound[i];
        res = res.max(diff);
    }
    res
}

fn case3(upper_bound: &Vec<usize>, max_a: usize) -> usize {
    // max_a is the min
    let n: usize = upper_bound.len();
    let mut res: usize = 0;
    for i in 0..n {
        let diff: usize = upper_bound[i] - max_a;
        res = res.max(diff);
    }
    res
}
