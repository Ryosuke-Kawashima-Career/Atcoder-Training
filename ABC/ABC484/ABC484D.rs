use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut cnt = vec![0usize; n + 1];
    let mut ans: u64 = 1;

    for r in 0..n {
        if r > 0 && a[r] != a[r - 1] {
            ans += (r - cnt[a[r]]) as u64;
        }
        cnt[a[r]] += 1;
    }

    println!("{}", ans);
}
