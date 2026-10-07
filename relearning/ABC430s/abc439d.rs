use proconio::input;
#[derive(Debug, Copy, Clone)]
struct Divident {
    divisor: usize,
    multiplier: usize,
    index: usize,
}
// abc439d
// Q. how many triples (i, j, k) such that i < j < k and (a[i], a[j], a[k]) is divisible by 3,5,7 respectively?
// A. Event Sort and remove complementary combinations
fn main() {
    input! {n: usize, a: [usize; n]}
    let mut events753: Vec<Divident> = Vec::new();
    for i in 0..n {
        for div in [3, 5, 7] {
            if a[i] % div == 0 {
                events753.push(Divident {
                    divisor: div,
                    multiplier: a[i] / div,
                    index: i,
                });
            }
        }
    }
    // Only combine roles 7, 5, and 3 that have the same multiplier.
    events753.sort_unstable_by_key(|event| (event.multiplier, event.index));
    let mut ans: u64 = 0;
    let mut left: usize = 0;
    while left < events753.len() {
        let mut right: usize = left + 1;
        while right < events753.len() && events753[right].multiplier == events753[left].multiplier {
            right += 1;
        }
        ans += sweepline(&events753[left..right]);
        left = right;
    }
    println!("{}", ans);
}

fn sweepline(events: &[Divident]) -> u64 {
    // complementary counting
    let mut sum7: u64 = 0;
    let mut sum5: u64 = 0;
    let mut sum3: u64 = 0;
    for event in events {
        match event.divisor {
            3 => sum3 += 1,
            5 => sum5 += 1,
            7 => sum7 += 1,
            _ => unreachable!(),
        }
    }
    let mut result: u64 = sum3 * sum5 * sum7;
    // sweepline
    let mut count7: u64 = 0;
    let mut count3: u64 = 0;
    for event in events {
        match event.divisor {
            3 => {
                count3 += 1;
                sum3 -= 1;
            }
            5 => {
                // Exclude triples where the 5-role index lies in the middle.
                result -= count3 * sum7;
                result -= count7 * sum3;
            }
            7 => {
                count7 += 1;
                sum7 -= 1;
            }
            _ => unreachable!(),
        }
    }
    result
}
