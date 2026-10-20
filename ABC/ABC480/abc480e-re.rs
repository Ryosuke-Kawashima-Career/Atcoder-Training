use proconio::input;
// ABC480E
// Q. Find the number of arithmetic sequences whose product of all the elements is <= n
// A. Binary Search
fn main() {
    input! {n: usize}
    let mut ans: usize = 0;
    let mut a0: usize = 1;
    while a0 * (a0 + 1) * (a0 + 2) <= n {
        let mut length: usize = 3;
        let mut remains: usize = n / a0 / (a0 + 1) / (a0 + 2);
        // Upper bound of common difference d for (a0, length=3):
        // a0 * (a0 + d) * (a0 + 2d) <= n  =>  2 * d^2 < n / a0
        let mut max_possible_diff: usize = (((n / a0 / 2) as f64).sqrt() as usize) + 2;
        while remains > 0 {
            let max_d = get_num(a0, length, n, max_possible_diff);
            ans += max_d;
            // For the next length, valid d cannot exceed the current max_d
            max_possible_diff = max_d + 1;
            remains /= a0 + length;
            length += 1;
        }
        // Do not forget to increment the initial value
        a0 += 1;
    }
    println!("{}", ans);
}

fn get_num(a0: usize, length: usize, product_limit: usize, ng_init: usize) -> usize {
    // a0: first term, length: length of arithmetic sequence, product_limit: product limit
    // diff: common difference
    let mut ok_diff: usize = 1;
    let mut ng_diff: usize = ng_init.max(2);
    while ng_diff - ok_diff > 1 {
        let mid_diff = (ng_diff + ok_diff) / 2;
        let mut current_limit: usize = product_limit;
        for k in 0..length {
            current_limit /= a0 + k * mid_diff;
            if current_limit == 0 {
                break;
            }
        }
        if current_limit > 0 {
            ok_diff = mid_diff;
        } else {
            ng_diff = mid_diff;
        }
    }
    ok_diff
}
