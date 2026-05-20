use proconio::input;
// abc365e
// Q. ∑_{i=1}^{n-1} ∑_{j=i+1}^{n} a_i ^ ... ^ a_j
// A. Calculate the contribution of each bit using prefix XOR sum.
// A. Xorの問題についてBitの桁ごとに計算する。
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    // P[i] = a[0] ^ a[1] ^ ... ^ a[i-1]
    let mut p = vec![0; n + 1];
    for i in 0..n {
        p[i + 1] = p[i] ^ a[i];
    }

    let mut total_xor_sum: usize = 0;

    // A_i <= 10^8, so it fits in 27 bits (2^26 = 67M, 2^27 = 134M)
    for bit in 0..28 {
        let mut count1: usize = 0;
        let mut count0: usize = 0;

        // Count how many prefix XORs have the `bit`-th bit set to 1 vs 0
        for i in 0..=n {
            if (p[i] >> bit) & 1 == 1 {
                count1 += 1;
            } else {
                count0 += 1;
            }
        }

        // Pairs of (0, 1) in prefix XORs mean the XOR sum between them has the bit set.
        // There are count1 * count0 such pairs.
        let pairs = count0 * count1;

        // Add the contribution of this bit to the total sum
        total_xor_sum += pairs * (1 << bit);
    }

    // The total_xor_sum includes subarrays of length 1 (i.e., just a_i).
    // The problem asks for j = i + 1..n, which means length >= 2.
    // So we subtract the sum of a_i.
    let mut sum_a: usize = 0;
    for x in &a {
        sum_a += x;
    }

    let ans = total_xor_sum - sum_a;
    println!("{}", ans);
}
