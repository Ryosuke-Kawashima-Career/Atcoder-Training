use proconio::input;

fn main() {
    input! {n: usize, m: usize, k: usize, x: usize, y: usize, mut a: [usize; n], mut b: [usize; m]}
    a.sort();
    b.sort();
    let mut prefix_a: Vec<usize> = vec![0; n + 1];
    let mut prefix_b: Vec<usize> = vec![0; m + 1];
    for i in 0..n {
        prefix_a[i + 1] = prefix_a[i] + a[i];
    }
    for j in 0..m {
        prefix_b[j + 1] = prefix_b[j] + b[j];
    }
    let mut ans: usize = 0;
    let mut cur_y: usize = 0;
    for j in 0..=m {
        if j == 0 {
            let remain_asset: usize = x + k * y;
            let index: usize = prefix_a.partition_point(|&a| a <= remain_asset);
            ans = ans.max(index);
            continue;
        }
        cur_y = cur_y + (b[j - 1] + k - 1) / k;
        if cur_y > y {
            break;
        }
        let remain_asset: usize = x + (k - cur_y) * y;
        let index: usize = prefix_a.partition_point(|&a| a <= remain_asset);
        ans = ans.max(index + j);
    }
    println!("{}", ans);
}
