use proconio::input;

fn main() {
    input! {n: usize, m: usize, k: usize, x: usize, y: usize, mut a: [usize; n], mut b: [usize; m]};
    a.sort();
    b.sort();
    let mut b_bills: Vec<usize> = Vec::new();
    for i in 0..m {
        let necessary_bills: usize = (b[i] + k - 1) / k;
        b_bills.push(necessary_bills);
    }
    b_bills.sort();

    let mut prefix_a: Vec<usize> = vec![0; n + 1];
    for i in 0..n {
        prefix_a[i + 1] = prefix_a[i] + a[i];
    }
    let mut prefix_b: Vec<usize> = vec![0; m + 1];
    for i in 0..m {
        prefix_b[i + 1] = prefix_b[i] + b[i];
    }
    let mut prefix_b_bills: Vec<usize> = vec![0; m + 1];
    for i in 0..m {
        prefix_b_bills[i + 1] = prefix_b_bills[i] + b_bills[i];
    }

    let mut ans: usize = 0;
    let mut left: usize = 0;
    for nb in 0..=m {
        let mut right: usize = left;
        while right + 1 <= m && prefix_b_bills[right + 1] <= y {
            right += 1;
        }
        let remain_y: usize = y - prefix_b_bills[right];
        let usuable_money = x + k * remain_y;
        let num_a: usize = a.partition_point(|&dessert| dessert <= usuable_money);
        let cur = num_a + right;
        ans = ans.max(cur);
        left = right;
    }
    println!("{}", ans);
}
