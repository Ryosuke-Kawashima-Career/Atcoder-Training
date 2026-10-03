use proconio::input;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    // Counting sort is O(N) because every value is between 1 and N.
    let mut frequency = vec![0; n + 1];
    for &value in &a {
        frequency[value] += 1;
    }
    let mut sorted_a = Vec::with_capacity(n);
    for value in 1..=n {
        sorted_a.resize(sorted_a.len() + frequency[value], value);
    }

    let mut first = None;
    let mut last = 0;
    for i in 0..n {
        if a[i] != sorted_a[i] {
            first.get_or_insert(i);
            last = i;
        }
    }

    // A single length-K interval must contain every mismatching position.
    if first.map_or(true, |left| last - left + 1 <= k) {
        println!("Yes");
    } else {
        println!("No");
    }
}
