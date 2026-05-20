use proconio::input;
use std::collections::BTreeMap;
fn main() {
    input! {n: usize, k: usize, a: [usize; n]}
    let mut map: BTreeMap<usize, usize> = BTreeMap::new();
    for x in a {
        *map.entry(x).or_insert(0) += 1;
    }

    loop {
        if !can_reduce_diff(&mut map, k) {
            break;
        } else {
            update_to_reduce_diff(&mut map, k);
        }
    }

    let ans = get_min_max_diff(&map);
    println!("{}", ans);
}

fn get_min_max_diff(map: &BTreeMap<usize, usize>) -> usize {
    let min_val = map.keys().next().unwrap();
    let max_val = map.keys().last().unwrap();
    max_val - min_val
}

fn can_reduce_diff(map: &BTreeMap<usize, usize>, k: usize) -> bool {
    let min_val = map.keys().next().unwrap();
    let max_val = map.keys().last().unwrap();
    let cur_diff: usize = max_val - min_val;
    let min_count = map.get(min_val).unwrap();

    let updated_min: usize = min_val + k;
    if let Some(second_min) = map.keys().nth(1) {
        let next_max: usize = *max_val.max(&updated_min);
        let next_min: usize = *second_min.min(&updated_min);
        let next_diff: usize = next_max - next_min;
        return next_diff < cur_diff;
    } else {
        return false;
    }
}

fn update_to_reduce_diff(map: &mut BTreeMap<usize, usize>, k: usize) {
    let min_val: usize = *map.keys().next().unwrap();
    let min_count: usize = *map.get(&min_val).unwrap();

    map.remove(&min_val);
    let updated_min: usize = min_val + k;
    *map.entry(updated_min).or_insert(0) += min_count;
}
