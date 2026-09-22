use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = match iter.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };
    let k: i64 = iter.next().unwrap().parse().unwrap();

    let mut rems = Vec::with_capacity(n);
    for _ in 0..n {
        let a: i64 = iter.next().unwrap().parse().unwrap();
        rems.push(a % k);
    }

    rems.sort_unstable();
    rems.dedup();

    let m = rems.len();
    if m <= 1 {
        // All elements can be made completely equal
        println!("0");
        return;
    }

    // Find the maximum gap on the circle of circumference K
    let mut max_gap = (rems[0] + k) - rems[m - 1]; // Gap wrapping around 0
    for i in 0..m - 1 {
        let gap = rems[i + 1] - rems[i];
        if gap > max_gap {
            max_gap = gap;
        }
    }

    // The minimum span is the circumference minus the largest gap
    let ans = k - max_gap;
    println!("{}", ans);
}
