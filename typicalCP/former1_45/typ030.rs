use proconio::input;
// Typical CP 0303
// Q. Count the number of integers whose prime factors are >= k kinds.
// A. Eratosthenes Sieve
fn main() {
    input! {n: usize, k: u8}
    let factor_counts: Vec<u8> = count_prime_factors(n);
    let ans: usize = factor_counts[2..=n].iter().filter(|&&cnt| cnt >= k).count();
    println!("{}", ans);
}

fn count_prime_factors(n: usize) -> Vec<u8> {
    /* Count distinct prime factors for all numbers up to n using the Sieve of Eratosthenes */
    let mut cnt: Vec<u8> = vec![0; n + 1];
    for num in 2..=n {
        // If cnt[num] == 0, `num` has no smaller prime factors, so `num` is prime.
        if cnt[num] == 0 {
            for multiple in (num..=n).step_by(num) {
                cnt[multiple] += 1;
            }
        }
    }
    cnt
}
