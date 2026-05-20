use proconio::input;

fn solve(m: usize, a: &[usize]) -> usize {
    let n = a.len();
    let k = n / 2;
    
    // b_i tracks the difference between symmetric pairs
    let mut b = vec![0; k + 2];
    for i in 1..=k {
        let left = a[i - 1];
        let right = a[n - i];
        b[i] = (right + m - left) % m;
    }
    
    // d_i represents the difference array of b
    let mut d = vec![0; k + 1];
    let mut sum_d = 0;
    for i in 1..=k+1 {
        d[i - 1] = (b[i] + m - b[i - 1]) % m;
        sum_d += d[i - 1];
    }
    
    // c represents exactly how many shifts of -M we must apply
    let c = sum_d / m;
    d.sort_unstable();
    
    let mut cost = 0;
    for i in 0..(k + 1) {
        // Apply the -M shifts to the C largest elements to minimize the absolute values
        if i >= (k + 1) - c {
            cost += m - d[i];
        } else {
            cost += d[i];
        }
    }
    
    // Since each range operation alters exactly two bounds by +1 and -1, operations = sum(|D|) / 2
    cost / 2
}

fn main() {
    input! { t: usize }
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            a: [usize; n]
        }
        let min_ops = solve(m, &a);
        println!("{}", min_ops);
    }
}
