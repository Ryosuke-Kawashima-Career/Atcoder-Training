use proconio::input;

fn main() {
    input! {a: usize, b: usize, k: usize}
    let mut takahashi: usize = a;
    let mut aoki: usize = b;

    let mut remain_operations: usize = k;
    if remain_operations >= a {
        takahashi = 0;
        remain_operations = remain_operations - a;
    } else {
        takahashi = takahashi - remain_operations;
        remain_operations = 0;
    }
    if remain_operations >= b {
        aoki = 0;
        remain_operations = remain_operations - b;
    } else {
        aoki = aoki - remain_operations;
        remain_operations = 0;
    }

    println!("{} {}", takahashi, aoki);
}
