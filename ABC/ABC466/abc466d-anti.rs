use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        rc: [(usize, usize); m],
    }

    let mut last_row = vec![0; n + 1];
    let mut last_col = vec![0; n + 1];

    for i in 0..m {
        let (r, c) = rc[i];
        last_row[r] = i + 1;
        last_col[c] = i + 1;
    }

    let mut ans = 0;
    for i in 0..m {
        let (r, c) = rc[i];
        if last_row[r] == i + 1 && last_col[c] == i + 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
