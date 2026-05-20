use proconio::input;
// ABC368F: Two Players Game
// Q. Given a[0..n] and repalce a[i] (> 2) with its divisor x. The person who fails to do the operation loses the game. = Unbiased Fair Game between the two players.
// A. The solution might be Nim (take as many as you want) or Grundy number
// A. Implement Sieve to precompute Smallest Prime Factor (SPF)
// = Seive of Eratosthenes = エラトステネスの篩
fn main() {
    input! {n: usize, a: [usize; n]}
    let mut smallest_prime_factors: Vec<usize> = get_smallest_prime_factors(&a);
    let mut xor_sum_nim: usize = 0;
    for i in 0..n {
        xor_sum_nim ^= get_prime_factors(a[i], &smallest_prime_factors);
    }
    if xor_sum_nim > 0 {
        println!("Anna");
    } else {
        println!("Bruno");
    }
}

fn get_prime_factors(mut num: usize, smallest_prime_factors: &Vec<usize>) -> usize {
    let mut count_prime_factors: usize = 0;
    while num > 1 {
        let spf_num: usize = smallest_prime_factors[num];
        count_prime_factors += 1;
        num /= spf_num;
    }
    return count_prime_factors;
}

fn get_smallest_prime_factors(nums: &Vec<usize>) -> Vec<usize> {
    let max_num = *nums.iter().max().unwrap();
    let mut sieve: Vec<usize> = vec![0; max_num + 1];
    for num in 2..=max_num {
        if sieve[num] == 0 {
            sieve[num] = num;
            let mut k: usize = 2;
            while k * num <= max_num {
                if sieve[k * num] == 0 {
                    sieve[k * num] = num;
                }
                k += 1;
            }
        }
    }
    return sieve;
}
