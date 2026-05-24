use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    // a[i] stores the total number of times we placed a block in cell i.
    let mut a = vec![0usize; n];

    // freq[v] stores the number of cells i such that a[i] == v.
    let mut freq = vec![0usize; q + 2];
    freq[0] = n;

    // suffix_sum[v] stores the number of cells i such that a[i] >= v.
    let mut suffix_sum = vec![0usize; q + q + 2];
    suffix_sum[0] = n;

    // d represents the total number of times we decremented all cells.
    let mut d = 0usize;

    for _ in 0..q {
        input! {
            query_type: usize,
        }
        if query_type == 1 {
            input! {
                x: Usize1,
            }
            let v = a[x];
            freq[v] -= 1;
            a[x] += 1;
            freq[v + 1] += 1;
            suffix_sum[v + 1] += 1;

            if freq[d] == 0 {
                d += 1;
            }
        } else {
            input! {
                y: usize,
            }
            // We want to find the number of cells with at least y blocks.
            // Actual block count for cell i is a[i] - d.
            // a[i] - d >= y  <=>  a[i] >= y + d.
            let target = y + d;
            if target < suffix_sum.len() {
                println!("{}", suffix_sum[target]);
            } else {
                println!("0");
            }
        }
    }
}
