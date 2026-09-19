use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: u128,
        x: u128,
        y: u128,
        mut a: [u128; n],
        mut b: [u128; m],
    }

    // Sort both desserts and drinks in ascending order of price
    a.sort_unstable();
    b.sort_unstable();

    // Prefix sums of dessert prices
    let mut pref_a = vec![0u128; n + 1];
    for i in 0..n {
        pref_a[i + 1] = pref_a[i] + a[i];
    }

    // Prefix sums of drink prices and required K-dollar bills
    let mut pref_b = vec![0u128; m + 1];
    let mut pref_k_bills = vec![0u128; m + 1];
    for j in 0..m {
        pref_b[j + 1] = pref_b[j] + b[j];
        // Ceiling division: (B_j + K - 1) / K
        let bills_needed = (b[j] + k - 1) / k;
        pref_k_bills[j + 1] = pref_k_bills[j] + bills_needed;
    }

    let total_wealth: u128 = x + y * k;
    let mut max_products = 0;

    // Try buying k drinks (0 <= k <= m)
    for num_drinks in 0..=m {
        // Check if we have enough K-dollar bills
        if pref_k_bills[num_drinks] > y {
            break; // Since prices are positive, larger k will also exceed y
        }

        // Check if drink costs exceed total wealth
        if pref_b[num_drinks] > total_wealth {
            break;
        }

        let remaining_wealth = total_wealth - pref_b[num_drinks];

        // Binary search for maximum number of desserts we can buy
        // partition_point finds the first index where pref_a[d] > remaining_wealth
        let num_desserts = pref_a.partition_point(|&cost| cost <= remaining_wealth) - 1;

        max_products = max_products.max(num_drinks + num_desserts);
    }

    println!("{}", max_products);
}
