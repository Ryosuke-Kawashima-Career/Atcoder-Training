use proconio::input;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}
    let mut is_less: bool = a[0] < k;
    let mut is_equal: bool = a[0] == k;
    let mut is_greater: bool = a[0] > k;
    let mut ans: usize = 0;
    for i in 1..n {
        match (is_less, is_equal, is_greater) {
            (true, _, _) => {
                if a[i] > k {
                    ans += 1;
                    is_less = false;
                    is_greater = true
                } else if a[i] == k {
                    ans += 1;
                    is_less = false;
                    is_equal = true;
                }
            }
            (_, true, _) => {
                if a[i] == k {
                    ans += 1;
                }
            }
            (_, _, true) => {
                if a[i] < k {
                    ans += 1;
                    is_greater = false;
                    is_less = true
                } else if a[i] == k {
                    ans += 1;
                    is_greater = false;
                    is_equal = true
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{}", ans);
}
