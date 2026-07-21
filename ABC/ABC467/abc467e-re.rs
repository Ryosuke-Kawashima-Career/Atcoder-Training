use proconio::input;
use std::collections::BTreeMap;
fn main() {
    input! {n: usize, m: i64, a: [i64; n], b: [i64; n-1]}
    // Step1: Calculate Difference
    let mut sum_a: Vec<i64> = vec![0; n - 1];
    let mut diff: Vec<i64> = vec![0; n - 1];
    for i in 0..n - 1 {
        sum_a[i] = (a[i] + a[i + 1]) % m;
        diff[i] = (b[i] + m - sum_a[i]) % m;
    }

    // Step2: Estimate Base Increment
    let mut base: Vec<i64> = vec![0; n];
    for i in 1..n {
        base[i] = (diff[i - 1] + m - base[i - 1]) % m;
    }
    // Step3: Slope of parities
    let mut n_odd: usize = 0;
    let mut n_even: usize = 0;
    for i in 0..n {
        if i % 2 == 0 {
            n_even += 1;
        } else {
            n_odd += 1;
        }
    }
    let slope: usize = n_odd - n_even;

    // Step4: Delta Mapping
    let mut delta_map: BTreeMap<i64, i64> = BTreeMap::new();
    for i in 0..n {
        if i % 2 == 0 {
            let v_plus_1: i64 = m - base[i];
            *delta_map.entry(v_plus_1).or_insert(0) -= m;
        } else {
            let v_plus_1: i64 = base[i] + 1;
            *delta_map.entry(v_plus_1).or_insert(0) += m;
        }
    }

    // Step5: Sweep line algorithm
    let mut init_val: i64 = 0;
    for i in 0..n {
        init_val += base[i];
    }

    let mut min_val: i64 = init_val;
    let mut curr_val: i64 = init_val;
    let mut curr_v: i64 = 0;
    for (&next_v, &delta) in delta_map.iter() {
        if next_v >= m {
            continue;
        }
        let steps: i64 = next_v - 1 - curr_v;
        let val_before: i64 = curr_val + slope as i64 * steps;
        let val_at: i64 = val_before + slope as i64 + delta;
        min_val = min_val.min(val_at);
        curr_val = val_at;
        curr_v = next_v;
    }
    println!("{}", min_val);
}
