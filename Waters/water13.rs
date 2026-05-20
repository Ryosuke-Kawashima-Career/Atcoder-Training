use proconio::input;

fn main() {
    input! {r: usize, c: usize, a: [[usize; c]; r]}
    let mask_r: usize = 1 << r;
    let mut ans: usize = 0;
    for mask in 0..mask_r {
        let mut count: usize = 0;
        for col in 0..c {
            let mut count0: usize = 0;
            let mut count1: usize = 0;
            for row in 0..r {
                if (mask >> row) & 1 == 1 {
                    if a[row][col] == 0 {
                        count1 += 1;
                    } else {
                        count0 += 1;
                    }
                } else {
                    if a[row][col] == 0 {
                        count0 += 1;
                    } else {
                        count1 += 1;
                    }
                }
            }
            count += count0.max(count1);
        }
        if count > ans {
            ans = count;
        }
    }
    println!("{}", ans);
}
