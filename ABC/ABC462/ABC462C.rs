use proconio::input;

fn main() {
    input! {
        n: usize,
        pts: [(usize, usize); n],
    }

    let mut y_at_x = vec![0; n + 1];
    for &(x, y) in &pts {
        y_at_x[x] = y;
    }

    let mut ans = 0;
    let mut min_y = usize::MAX;

    for x in 1..=n {
        let y = y_at_x[x];
        if y < min_y {
            ans += 1;
            min_y = y;
        }
    }

    println!("{}", ans);
}
