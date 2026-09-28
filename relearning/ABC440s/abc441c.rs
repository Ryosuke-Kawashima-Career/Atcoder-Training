use proconio::input;
fn main() {
    input! {n: usize, k: usize, x: usize, mut a: [usize; n]}
    a.sort_by(|x1, x2| x2.cmp(&x1));
    // offense vs defense
    let mut min_sake: usize = 0;
    if min_sake < x {
        println!("-1");
        return;
    }

    let mut cur_water: usize = 0;
    let mut ans: usize = 0;
    for i in 0..n - k {
        cur_water += a[i];
        ans += 1;
        if cur_water >= x {
            break;
        }
    }
    println!("{}", ans);
}
