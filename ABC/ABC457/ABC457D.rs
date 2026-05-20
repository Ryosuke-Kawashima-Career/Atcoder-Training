use proconio::input;
const INF: usize = 1 << 40;
// Q. Max of min(A) when a[i] + i is repeated k times.
fn main() {
    input! {n: usize, k: usize, a: [usize; n]}
    let mut val_l: usize = 0;
    let mut val_r: usize = INF;

    while val_r - val_l > 1 {
        let mid = (val_l + val_r) / 2;
        if get_operations(mid, &a) < k {
            val_l = mid;
        } else {
            val_r = mid;
        }
    }

    println!("{}", val_r);
}

fn get_operations(min_a: usize, a: &[usize]) -> usize {
    let mut operations: usize = 0;
    let n: usize = a.len();
    for i in 0..n {
        if a[i] < min_a {
            let diff = min_a - a[i];
            let mut op: usize = diff / (i + 1);
            if op % (i + 1) != 0 {
                op += 1;
            }
            operations += op;
        }
    }
    operations
}
