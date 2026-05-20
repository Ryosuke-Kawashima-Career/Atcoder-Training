use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
    }

    // Total sum of Li is 2*10^5, so we can store all sequences in memory.
    let mut lengths = Vec::with_capacity(n);
    let mut a = Vec::with_capacity(n);

    for _ in 0..n {
        input! {
            l: usize,
            al: [u64; l],
        }
        lengths.push(l);
        a.push(al);
    }

    input! {
        c: [u64; n]
    }

    let mut current_k = k;

    for i in 0..n {
        // Calculate the total length added by repeating Ai, Ci times.
        let block_len = c[i] * (lengths[i] as u64);

        if current_k <= block_len {
            // The K-th element is within the current block of Ci * Ai.
            // (current_k - 1) gives the 0-indexed position within the block.
            // Taking modulo Li gives the position within a single copy of Ai.
            let offset = (current_k - 1) % (lengths[i] as u64);
            println!("{}", a[i][offset as usize]);
            return;
        }

        // Move to the next block by subtracting the current block's length.
        current_k -= block_len;
    }
}
