use proconio::input;
// ABC484D
// Q. Count the number of arrays, which can be created by inserting one element into a prior position
// A. Count the numbers of items and record them
fn main() {
    input!{n: usize, a: [usize; n]}
    let mut count: Vec<usize> = vec![0; n+1];
    let mut ans: usize = 0;
    for i in 0..n {
        if i >= 1 && a[i] != a[i-1] {
            ans += (i - count[a[i]]);
        }
        count[a[i]] += 1;
    }
    // +1 means the original array
    println!("{}", ans+1);
}
