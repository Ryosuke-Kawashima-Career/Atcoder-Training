use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    // Pre-calculate all valid "powers of 2" that can be used up to the 10^9 boundary.
    // Also store their positional multipliers so we can append them mathematically.
    let mut p2s = Vec::new();
    let mut p = 1u64;
    while p <= 1_000_000_000 {
        let s = p.to_string();
        let mult = 10u64.pow(s.len() as u32);
        p2s.push((p, mult));
        p *= 2;
    }

    // The maximum theoretical combinations generated under limits is roughly ~1.4 million.
    // We pre-allocate to avoid any memory reallocation jitters.
    let mut results = Vec::with_capacity(1_500_000);
    
    // Generate all valid string numbers
    dfs(0, &p2s, &mut results);

    // Filter, Unique, and Fetch
    results.sort_unstable();
    results.dedup();

    println!("{}", results[n - 1]);
}

fn dfs(val: u64, p2s: &Vec<(u64, u64)>, results: &mut Vec<u64>) {
    if val > 0 {
        results.push(val);
    }
    for &(p, mult) in p2s.iter() {
        // Concatenating "val" with "p"
        let nxt = val * mult + p;
        if nxt <= 1_000_000_000 {
            dfs(nxt, p2s, results);
        }
    }
}
