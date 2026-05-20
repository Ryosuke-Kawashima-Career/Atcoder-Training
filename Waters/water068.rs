use proconio::input;

fn main() {
    input! {n: usize}
    let is_primes: Vec<bool> = eratosthenes(n);
    get_prime_factors(n, &is_primes);
}

fn get_prime_factors(mut num: usize, is_primes: &Vec<bool>) {
    print!("{}:", num);
    let mut cur_prime: usize = 2;
    while num > 1 {
        while num % cur_prime == 0 {
            num /= cur_prime;
            print!(" {}", cur_prime);
        }
        while cur_prime + 1 < is_primes.len() && !is_primes[(cur_prime + 1) as usize] {
            cur_prime += 1;
        }
        cur_prime += 1;
    }
    if num > 1 {
        print!(" {}", num);
    }
    println!();
}

fn eratosthenes(n: usize) -> Vec<bool> {
    let mut is_primes: Vec<bool> = vec![true; n + 1];
    is_primes[0] = false;
    is_primes[1] = false;

    for num in 2..=n {
        if is_primes[num] {
            let mut k: usize = 2;
            while k * num <= n {
                is_primes[k * num] = false;
                k += 1;
            }
        }
    }

    return is_primes;
}
