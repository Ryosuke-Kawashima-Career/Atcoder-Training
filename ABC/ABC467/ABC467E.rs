use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        m: i64,
        a: [i64; n],
        b: [i64; n - 1],
    }

    let mut s = vec![0; n - 1];
    let mut d = vec![0; n - 1];
    for i in 0..n - 1 {
        s[i] = (a[i] + a[i + 1]) % m;
        d[i] = (b[i] + m - s[i]) % m;
    }

    let mut c = vec![0; n];
    c[0] = 0;
    for i in 1..n {
        c[i] = (d[i - 1] + m - c[i - 1]) % m;
    }

    let mut n_odd = 0;
    let mut n_even = 0;
    for i in 0..n {
        if i % 2 == 0 {
            n_odd += 1;
        } else {
            n_even += 1;
        }
    }

    let slope = n_odd - n_even;

    // Group events by v+1
    let mut delta_map = BTreeMap::new();
    for i in 0..n {
        if i % 2 == 0 {
            // odd index (0-based: 0, 2, ...)
            let v_plus_1 = m - c[i];
            *delta_map.entry(v_plus_1).or_insert(0) -= m;
        } else {
            // even index (0-based: 1, 3, ...)
            let v_plus_1 = c[i] + 1;
            *delta_map.entry(v_plus_1).or_insert(0) += m;
        }
    }

    let mut init_val = 0;
    for i in 0..n {
        init_val += c[i];
    }

    let mut min_val = init_val;
    let mut curr_val = init_val;
    let mut curr_v = 0;

    for (&next_v, &delta) in delta_map.iter() {
        if next_v >= m {
            continue;
        }
        let steps = next_v - 1 - curr_v;
        let val_before = curr_val + slope * steps;
        let val_at = val_before + slope + delta;

        min_val = std::cmp::min(min_val, val_at);
        curr_val = val_at;
        curr_v = next_v;
    }

    println!("{}", min_val);
}
