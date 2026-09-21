use proconio::input;

fn main() {
    input! {n: usize, start: usize, l: usize, a: [usize; n-1]}
    let mut prefix: Vec<usize> = vec![0; n + 1];
    for i in 1..n {
        prefix[i] = prefix[i - 1] + a[i - 1];
    }
    let mut ans: usize = 0;
    for left in 0..start {
        for right in start..n {
            let length_left: usize = prefix[start] - prefix[left];
            let length_right: usize = prefix[right + 1] - prefix[start];

            let length1: usize = 2 * length_left + length_right;
            let length2: usize = 2 * length_right + length_left;

            if length1 <= l || length2 <= l {
                ans = ans.max(right - left + 1);
            }
        }
    }
    println!("{}", ans);
}
