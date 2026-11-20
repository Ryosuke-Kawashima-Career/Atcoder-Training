use proconio::input;

const MOD: usize = 100_000;

fn main() {
    input! {
        n: usize,
        k: u64,
    }

    let mut values: Vec<usize> = Vec::new();
    let mut visited: Vec<Option<usize>> = vec![None; MOD];
    let mut x: usize = n;
    let mut cur_index: usize = 0;

    while visited[x].is_none() {
        visited[x] = Some(cur_index);
        values.push(x);
        x = (x + sum_digits(x)) % MOD;
        cur_index += 1;
    }

    let cycle_start = visited[x].unwrap();
    let period = cur_index - cycle_start;
    let target_index: usize = if k >= cycle_start as u64 {
        ((k - cycle_start as u64) % period as u64) as usize + cycle_start
    } else {
        k as usize
    };

    println!("{}", values[target_index]);
}

fn sum_digits(mut x: usize) -> usize {
    let mut y: usize = 0;
    while x > 0 {
        y += x % 10;
        x /= 10;
    }
    y
}
