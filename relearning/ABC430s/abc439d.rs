use proconio::input;
#[derive(Debug, Copy, Clone)]
struct Divident {
    divisor: usize,
    divent: usize,
    index: usize,
}
// Event Sort
fn main() {
    input! {n: usize, a: [usize; n]}
    let mut events753: Vec<Divident> = Vec::new();
    for i in 0..n {
        for div in [3, 5, 7] {
            if a[i] % div == 0 {
                events753.push(Divident {
                    divisor: div,
                    divent: a[i],
                    index: i,
                });
            }
        }
    }
    events753.sort_by_key(|k| k.index);
    let ans: usize = sweepline(&events753);
    println!("{}", ans);
}

fn sweepline(events: &Vec<Divident>) -> usize {
    // complementary counting
    let mut sum7: usize = 0;
    let mut sum5: usize = 0;
    let mut sum3: usize = 0;
    for event in events {
        let (divisor, divident, index) = (event.divisor, event.divent, event.index);
        match divisor {
            3 => sum3 += 1,
            5 => sum5 += 1,
            7 => sum7 += 1,
            _ => unreachable!(),
        }
    }
    let mut result: usize = sum3 * sum5 * sum7;
    // sweepline
    let mut count7: usize = 0;
    let mut count3: usize = 0;
    for event in events {
        let (divisor, divident, index) = (event.divisor, event.divent, event.index);
        match divisor {
            3 => {
                count3 += 1;
                sum3 -= 1;
            }
            5 => {
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
