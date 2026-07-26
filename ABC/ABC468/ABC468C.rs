use itertools::Itertools;
use proconio::input;

fn main() {
    input! {n: usize, p: [usize; n], q: [usize; n]}
    let mut index_p: isize = -1;
    let mut index_q: isize = -1;
    for (i, perm) in (1..=n).permutations(n).enumerate() {
        if perm == p {
            index_p = i as isize;
        }
        if perm == q {
            index_q = i as isize;
        }
    }
    let ans = 0.max(index_q - index_p - 1);
    println!("{}", ans);
}
