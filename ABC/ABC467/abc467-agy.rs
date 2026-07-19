use proconio::input;

fn main() {
    input! {
        n: usize,
        _m: usize,
        a: [usize; n],
        b: [usize; n - 1],
    }

    let mut ans = usize::MAX;

    for p1 in 0..2 {
        let mut p = vec![0; n];
        p[0] = p1;
        for i in 0..n - 1 {
            p[i + 1] = (b[i] + 2 - p[i]) % 2;
        }

        let mut cost = 0;
        for i in 0..n {
            if a[i] % 2 != p[i] {
                cost += 1;
            }
        }
        ans = std::cmp::min(ans, cost);
    }

    println!("{}", ans);
}
