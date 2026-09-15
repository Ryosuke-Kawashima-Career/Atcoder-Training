use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n]}
    // the numbers of 1 yen, 10 yen, 100 yen
    let mut count: [usize; 3] = [0, 0, 0];

    for i in 0..n {
        let number_of_bills = if a[i] % 1000 == 0 {
            a[i] / 1000
        } else {
            a[i] / 1000 + 1
        };
        let mut diff: usize = number_of_bills * 1000 - a[i];
        count[2] += diff / 100;
        diff %= 100;
        count[1] += diff / 10;
        diff %= 10;
        count[0] += diff;
    }
    for i in 0..3 {
        println!("{} {} {}", count[0], count[1], count[2]);
    }
}
