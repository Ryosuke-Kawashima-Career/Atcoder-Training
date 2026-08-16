use proconio::{input, marker::Usize1};
// self inverse of xor
fn main() {
    input! {n: usize, q: usize}
    let mut more_than_zeros: Vec<usize> = Vec::new();
    let mut a: Vec<usize> = vec![0; n];
    let mut nim: usize = 0;
    for _query in 0..q {
        input! {query_type: usize}
        if query_type == 1 {
            input! {x: Usize1}
            let mut prev_ax: usize = a[x];
            a[x] += 1;
            nim ^= prev_ax ^ a[x];
            if a[x] == 1 {
                more_than_zeros.push(x);
            }
        } else {
            let mut next_more_than_zeros: Vec<usize> = Vec::new();
            for &index in more_than_zeros.iter() {
                let prev_ax: usize = a[index];
                a[index] -= 1;
                nim ^= prev_ax ^ a[index];
                if a[index] >= 1 {
                    next_more_than_zeros.push(index);
                }
            }
            more_than_zeros = next_more_than_zeros;
        }
        println!("{}", nim);
    }
}
