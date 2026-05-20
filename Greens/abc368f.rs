use proconio::input;
// ABC368F: Two Players Game
// Q. Given a[0..n] and repalce a[i] (> 2) with its divisor x. The person who fails to do the operation loses the game. = Unbiased Fair Game between the two players.
// A. The solution might be Nim (take as many as you want) or Grundy number
// A. Implement Sieve to precompute Smallest Prime Factor (SPF)
fn main() {
    input! {n: usize, a: [usize; n]}
    let mut divisors_a: Vec<usize> = vec![];
    for i in 0..n {
        divisors_a.push(count_divisors(a[i]));
    }
    let mut xor_sum: usize = 0;
    for i in 0..n {
        xor_sum ^= divisors_a[i];
    }
    if xor_sum > 0 {
        println!("Anna");
    } else {
        println!("Bruno");
    }
}

fn count_divisors(num: usize) -> usize {
    let mut divisors: Vec<usize> = vec![];
    let mut div: usize = 1;
    let mut target: usize = num;
    while div * div <= target {
        if target % div == 0 {
            divisors.push(div);
            if target / div != div {
                divisors.push(target / div);
            }
            target /= div;
        }
        div += 1;
    }
    divisors.len()
}
