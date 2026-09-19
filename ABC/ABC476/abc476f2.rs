use proconio::input;

// Helper to compute 1D all-pairs weighted distances in O(K)
fn compute_1d_distances(weights: &[u64]) -> Vec<u64> {
    let k = weights.len();
    let mut count_pref = vec![0u64; k + 1];
    let mut sum_pref = vec![0u64; k + 1];

    for i in 0..k {
        count_pref[i + 1] = count_pref[i] + weights[i];
        sum_pref[i + 1] = sum_pref[i] + (i as u64) * weights[i];
    }

    let total_count = count_pref[k];
    let total_sum = sum_pref[k];

    let mut dist = vec![0u64; k];
    for i in 0..k {
        let left_count = count_pref[i + 1];
        let left_sum = sum_pref[i + 1];
        let left_contrib = (i as u64) * left_count - left_sum;

        let right_count = total_count - left_count;
        let right_sum = total_sum - left_sum;
        let right_contrib = right_sum - (i as u64) * right_count;

        dist[i] = left_contrib + right_contrib;
    }

    dist
}

fn main() {
    input! {
        n: usize,
        m: u64,
        a: [u64; n],
        b: [u64; n],
    }

    let num_diagonals = 2 * n - 1;
    let mut weight_u = vec![0u64; num_diagonals];
    let mut weight_v = vec![0u64; num_diagonals];

    // Step 1: Accumulate weights along the two 45-degree diagonal axes
    for r in 0..n {
        let ar = a[r];
        for c in 0..n {
            let w = (ar * b[c]) % m;
            let u = r + c;
            let v = r + (n - 1) - c;
            weight_u[u] += w;
            weight_v[v] += w;
        }
    }

    // Step 2: Precompute 1D Manhattan distances for all u and v in O(N)
    let g = compute_1d_distances(&weight_u);
    let h = compute_1d_distances(&weight_v);

    // Step 3: Combine g and h for all cells and compute bitwise XOR sum
    let mut xor_sum = 0u64;
    for r in 0..n {
        for c in 0..n {
            let u = r + c;
            let v = r + (n - 1) - c;
            let f_rc = (g[u] + h[v]) / 2;
            let cell_id = (r as u64) * (n as u64) + (c as u64);
            let val = f_rc + cell_id;
            xor_sum ^= val;
        }
    }

    println!("{}", xor_sum);
}
