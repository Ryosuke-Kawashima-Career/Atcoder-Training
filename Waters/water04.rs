use proconio::input;

fn main() {
    input! {
        n: usize, m: usize, a: [[usize; m]; n]
    }
    let mut ans: usize = 0;
    for song1 in 0..m {
        for song2 in song1 + 1..m {
            let mut count: usize = 0;
            for person in 0..n {
                count += a[person][song1].max(a[person][song2]);
            }
            if count > ans {
                ans = count;
            }
        }
    }
    println!("{}", ans);
}
