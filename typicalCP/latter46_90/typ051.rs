use proconio::input;
// Q. Enumerate the number of items to choose
// A. Semi-Brute Force
fn main() {
    input! {n: usize, k: usize, p: i64, a: [i64; n]}
    let n1: usize = n / 2;
    let n2: usize = n - n1;
    let mut enum1: Vec<Vec<i64>> = vec![vec![]; n1 + 1];
    let mut enum2: Vec<Vec<i64>> = vec![vec![]; n2 + 1];
    enum_items(k, &a[..n1], &mut enum1);
    enum_items(k, &a[n1..], &mut enum2);
    for sums in &mut enum2 {
        sums.sort_unstable();
    }

    let mut count = 0_u64;
    for k1 in 0..=k.min(n1) {
        let k2 = k - k1;
        if k2 > n2 {
            continue;
        }
        for &price1 in &enum1[k1] {
            let valid = enum2[k2].partition_point(|&price2| price1 + price2 <= p);
            count += valid as u64;
        }
    }
    println!("{}", count);
}

fn enum_items(k: usize, a: &[i64], enum_arr: &mut [Vec<i64>]) {
    enum_arr[0].push(0);
    for (i, &price) in a.iter().enumerate() {
        // Descending order prevents choosing the current item more than once.
        for chosen in (1..=k.min(i + 1)).rev() {
            let (previous, current) = enum_arr.split_at_mut(chosen);
            for &prev_price in &previous[chosen - 1] {
                current[0].push(prev_price + price);
            }
        }
    }
}
