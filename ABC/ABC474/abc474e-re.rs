use proconio::input;

fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {n: usize, ab: [(i64, i64); n]}
        let mut diff_ba: Vec<i64> = (0..n).map(|i| ab[i].1 - ab[i].0).collect();
        diff_ba.sort();
        // price without using any coupons
        let upper_bound: i64 = ab.iter().map(|tup| tup.0).sum();
        let min_a: i64 = ab.iter().map(|tup| tup.0).min().unwrap();
        let mut ans: i64 = upper_bound;
        let mut curr: i64 = upper_bound;
        // k: the number of items purchased with coupons
        for k in 1..=n {
            curr += diff_ba[k - 1];
            let coupon_purchases: i64 = 0.max(2 * (k as i64) - n as i64);
            ans = ans.min(curr + coupon_purchases * min_a);
        }
        println!("{}", ans);
    }
}
