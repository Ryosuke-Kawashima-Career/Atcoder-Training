use proconio::input;

fn main() {
    input! {n: usize, m: usize, k: usize, x: usize, y: usize, mut a: [usize; n], mut b: [usize; m]};
    b.sort();
    let mut b_bill_remainders: Vec<(usize, usize)> = Vec::new();
    for i in 0..m {
        let necessary_bills: usize = (b[i] + k - 1) / k;
        b_bill_remainders.push((necessary_bills, necessary_bills * k - b[i]));
    }
    b_bill_remainders.sort_by(|a, b| {
        if a.0 == b.0 {
            b.1.cmp(&a.1)
        } else {
            a.0.cmp(&b.0)
        }
    });
    a.sort();
    let mut prefix_a: Vec<usize> = vec![0; n + 1];
    for i in 0..n {
        prefix_a[i + 1] = prefix_a[i] + a[i];
    }
    let mut prefix_b: Vec<usize> = vec![0; m + 1];
    for i in 0..m {
        prefix_b[i + 1] = prefix_b[i] + b_bill_remainders[i].1;
    }

    let mut ans: usize = 0;
    let mut used_y: usize = 0;
    for nb in 0..=m {
        used_y += b_bill_remainders[nb].0;
        if used_y > y {
            break;
        }
        let usuable_money = x + k * (y - used_y);
        let num_a: usize = a.partition_point(|&dessert| dessert < usuable_money);
        let cur = num_a + nb;
        ans = ans.max(cur);
    }
    println!("{}", ans);
}
