use proconio::input;
use std::collections::HashSet;
const LIMIT: usize = 1_000_000_000;
// abc451d
// Q. Count the number of integers which are created by combining powers of two.
// A. Categorize by digits and Brute force attack
// The first strategy to consider is Brute force attack.
fn main() {
    input! {n: usize}
    let mut power_of_2s: Vec<Vec<usize>> = vec![vec![]; 10];
    let mut power_of_2: usize = 1;
    while power_of_2 <= LIMIT {
        let length: usize = power_of_2.to_string().len();
        power_of_2s[length].push(power_of_2);
        power_of_2 *= 2;
    }

    let good_numbers_list: Vec<usize> = enum_good_numbers(&power_of_2s);
    let ans: usize = good_numbers_list[n - 1];
    println!("{}", ans);
}

fn enum_good_numbers(powers: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let mut good_numbers_with_length: Vec<HashSet<usize>> = vec![HashSet::new(); 10];
    good_numbers_with_length[0].insert(0);
    for keta in 1..10 {
        for prev_keta in 0..=keta {
            for &prev_num in good_numbers_with_length[prev_keta].clone().iter() {
                for &power2 in powers[keta - prev_keta].iter() {
                    let next_num: usize =
                        10usize.pow((keta - prev_keta) as u32) * prev_num + power2;
                    if next_num > LIMIT {
                        break;
                    }
                    good_numbers_with_length[keta].insert(next_num);
                }
            }
        }
        result.extend(good_numbers_with_length[keta].iter());
    }
    result.sort();
    result
}
