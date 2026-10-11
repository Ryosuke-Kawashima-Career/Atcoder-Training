use proconio::input;

fn main() {
    input! {n: usize, w: usize, ab: [(i64, usize); n]}
    let n1: usize = n / 2;
    let n2: usize = n - n1;
    let delicious1: Vec<(i64, usize)> = enum_all(&ab[0..n1]);
    let delicious2: Vec<(i64, usize)> = enum_all(&ab[n1..n]);
    let mut ans: i64 = 0;
    for &(val1, weight1) in delicious1.iter() {
        for &(val2, weight2) in delicious2.iter() {
            if weight1 + weight2 <= w {
                let val: i64 = val1 + val2;
                ans = ans.max(val);
            }
        }
    }
    println!("{}", ans);
}

fn enum_all(item: &[(i64, usize)]) -> Vec<(i64, usize)> {
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
