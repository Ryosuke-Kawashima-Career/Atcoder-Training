use proconio::input;

fn main() {
    input!{n: usize, m: usize}
    let mut cur_m: usize = m;
    let mut cur_n: usize = n;
    let mut ans: usize = 0;
    while cur_m > 0 {
        let x: usize = cur_n % cur_m;
        cur_m = x;
        ans += 1;
    }
    println!("{}", ans);
}
