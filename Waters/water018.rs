use proconio::input;

fn main() {
    input! {n: usize, mut s: [usize; n], q: usize, t: [usize; q]}
    s.sort();
    let mut ans: usize = 0;
    for i in 0..q {
        let index: usize = lower_bound(&s, t[i]);
        if index < n && s[index] == t[i] {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn lower_bound(array: &[usize], target: usize) -> usize {
    let n: usize = array.len();
    let mut left: isize = -1;
    let mut right: isize = n as isize;
    while right - left > 1 {
        let mid: isize = (left + right) / 2;
        if array[mid as usize] < target {
            left = mid;
        } else {
            right = mid;
        }
    }
    return right as usize;
}
