use proconio::input;

fn main() {
    input! {n: usize, l: [i64; n]}
    let bits: usize = 1 << n;
    let mut ans: usize = 0;
    for bit in 0..bits {
        let mut count: usize = 0;
        let mut cur_pos: i64 = 0;
        let mut cur_sign: i64 = 1;
        for i in 0..n {
            if bit >> i & 1 == 1 {
                cur_pos += l[i];
            } else {
                cur_pos -= l[i];
            }
            let next_sign: i64 = if cur_pos >= 0 { 1 } else { -1 };
            if cur_sign * next_sign == -1 {
                count += 1;
            }
            cur_sign = next_sign;
        }
        ans = ans.max(count);
    }
    println!("{}", ans);
}
