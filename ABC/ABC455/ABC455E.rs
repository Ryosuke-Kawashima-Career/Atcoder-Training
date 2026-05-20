use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut diff_ab = Vec::with_capacity(n + 1);
    let mut diff_bc = Vec::with_capacity(n + 1);
    let mut diff_ca = Vec::with_capacity(n + 1);
    let mut diff_abc = Vec::with_capacity(n + 1);

    let mut count_a: i64 = 0;
    let mut count_b: i64 = 0;
    let mut count_c: i64 = 0;

    diff_ab.push(count_a - count_b);
    diff_bc.push(count_b - count_c);
    diff_ca.push(count_c - count_a);
    diff_abc.push((count_a - count_b, count_b - count_c));

    for character in s {
        if character == 'A' {
            count_a += 1;
        } else if character == 'B' {
            count_b += 1;
        } else if character == 'C' {
            count_c += 1;
        }

        diff_ab.push(count_a - count_b);
        diff_bc.push(count_b - count_c);
        diff_ca.push(count_c - count_a);
        diff_abc.push((count_a - count_b, count_b - count_c));
    }

    let equal_ab = count_pairs(diff_ab);
    let equal_bc = count_pairs(diff_bc);
    let equal_ca = count_pairs(diff_ca);
    let equal_abc = count_pairs(diff_abc);

    let total: usize = (n as usize) * ((n as usize) + 1) / 2;
    let ans: usize = total - equal_ab - equal_bc - equal_ca + 2 * equal_abc;

    println!("{}", ans);
}

fn count_pairs<T: Eq + Ord + Copy>(mut array: Vec<T>) -> usize {
    if array.is_empty() {
        return 0;
    }
    array.sort();
    let mut res: usize = 0;
    let mut count: usize = 1;
    for i in 1..array.len() {
        if array[i] == array[i - 1] {
            count += 1;
        } else {
            res += count * (count - 1) / 2;
            count = 1;
        }
    }
    res += count * (count - 1) / 2;
    return res;
}
