use proconio::input;
use std::collections::HashMap;
// abc403d
// Q. Any difference of a[i] and a[j] should not be d.
// Example. a = [1, 1, 3, 4, 5], d = 2 | Answer. 1
fn main() {
    input! {n: usize, d: i64, mut a: [i64; n]}
    a.sort();
    let mut map = HashMap::new();
    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }
    let connected_values: Vec<Vec<(i64, usize)>> = get_connections(&map, d);
    let ans = min_decomposition(&connected_values);
    println!("{}", ans);
}

fn min_decomposition(connected_values: &Vec<Vec<(i64, usize)>>) -> usize {
    let mut ans: usize = 0;
    for i in 0..connected_values.len() {
        let mut value1: usize = 0;
        let mut value2: usize = 0;
        for j in 0..connected_values[i].len() {
            if j % 2 == 0 {
                value1 += connected_values[i][j].1;
            } else {
                value2 += connected_values[i][j].1;
            }
        }
        ans += value1.min(value2);
    }
    ans
}

fn get_connections(map: &HashMap<i64, i64>, d: i64) -> Vec<Vec<(i64, usize)>> {
    let mut connected_values: Vec<Vec<(i64, usize)>> = Vec::new();
    let values: Vec<i64> = map.keys().cloned().collect();
    let n: usize = values.len();
    let mut group_ID: usize = 0;
    let mut group_IDs: Vec<usize> = vec![n; n];
    let mut value_to_group_ID: HashMap<i64, usize> = HashMap::new();
    for i in 0..n {
        if map.contains_key(&(values[i] - d)) {
            group_IDs[i] = group_ID;
            connected_values[value_to_group_ID[&(values[i] - d)]].push((values[i], i));
            value_to_group_ID.insert(values[i], group_ID);
        } else {
            group_IDs[i] = group_ID;
            value_to_group_ID.insert(values[i], group_ID);
            group_ID += 1;
            connected_values.push(vec![(values[i], i)]);
        }
    }
    connected_values
}
