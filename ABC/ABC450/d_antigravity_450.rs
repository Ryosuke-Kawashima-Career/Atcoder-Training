use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    if n == 1 {
        println!("0");
        return;
    }

    // 1. Find the absolute maximum in the initial state.
    let max_a = *a.iter().max().unwrap();

    // 2. Add K as many times as possible to each element so that it approaches max_a,
    //    without exceeding max_a. 
    //    Since we can only ADD K, max_a is the lowest possible maximum bound we can aim for initially.
    let mut b: Vec<i64> = a
        .iter()
        .map(|&x| {
            let diff = max_a - x;
            let count = diff / k;
            x + count * k
        })
        .collect();

    // 3. Sort the normalized array
    b.sort_unstable();

    // 4. Initial best difference is just sweeping between the normalized max and min
    let mut min_diff = b[n - 1] - b[0];

    // 5. Test shifting the smallest elements past max_a
    for i in 0..n - 1 {
        // We simulate adding K to b[0], b[1], ..., b[i].
        // The new maximum inherently becomes b[i] + K (since b[i] is the largest among the shifted ones, 
        // and b[i] + K > max_a).
        let current_max = b[i] + k;
        
        // The new minimum is simply the next element in the sorted original array, b[i + 1], 
        // because all elements before it were pushed > max_a.
        let current_min = b[i + 1];

        let diff = current_max - current_min;
        if diff < min_diff {
            min_diff = diff;
        }
    }

    println!("{}", min_diff);
}
