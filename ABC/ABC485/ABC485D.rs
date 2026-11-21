use proconio::{input, marker::Chars};

fn main() {
    input!{n: usize, s: Chars}
    let mut prefix_a: Vec<usize> = vec![0; n+1];
    let mut prefix_b: Vec<usize> = vec![0; n+1];
    let mut prefix_c: Vec<usize> = vec![0; n+1];
    for i in 0..n {
        prefix_a[i+1] = prefix_a[i]; prefix_b[i+1] = prefix_b[i]; prefix_c[i+1] = prefix_c[i];
        match s[i] {
            'A' => {
                prefix_a[i+1] += 1;
            }
            'B' => {
                prefix_b[i+1] += 1;
            }
            'C' => {
                prefix_c[i+1] += 1;
            }
            _ => unreachable!()
        }
    }
    let last_a: usize = prefix_a.partition_point(|&num_a| num_a < prefix_a[n]);
    let last_b: usize = prefix_b.partition_point(|&num_b| num_b < prefix_b[n]);
    let last_c: usize = prefix_c.partition_point(|&num_c| num_c < prefix_c[n]);
    let max_j_exclusive: usize = last_a.min(last_b).min(last_c);

    let mut count: usize = 0;
    for i in 1..=n {
        if prefix_a[i] > 0 && prefix_b[i] > 0 && prefix_c[i] > 0 {
            let next_index_a: usize = prefix_a.partition_point(|&num_a| num_a <= prefix_a[i]);
            let next_index_b: usize = prefix_b.partition_point(|&num_b| num_b <= prefix_b[i]);
            let next_index_c: usize = prefix_c.partition_point(|&num_c| num_c <= prefix_c[i]);
            let next_index: usize = next_index_a.max(next_index_b).max(next_index_c);
            count += max_j_exclusive.saturating_sub(next_index);
        }
    }
    println!("{}", count);
}
