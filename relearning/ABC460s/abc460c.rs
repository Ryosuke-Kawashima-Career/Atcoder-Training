use proconio::input;

fn main() {
    input! {n: usize, m: usize, mut a: [usize; n], mut b: [usize; m]}
    a.sort();
    b.sort();
    let mut ans: usize = 0;
    let mut cur_index: usize = 0;
    for i in 0..n {
        while cur_index + 1 < m && b[cur_index + 1] <= a[i] * 2 {
            cur_index += 1;
        }
        if cur_index < m && b[cur_index] <= 2 * a[i] {
            ans += 1;
            cur_index += 1;
        }
    }
    println!("{}", ans);
}
