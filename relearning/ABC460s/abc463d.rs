use proconio::input;

fn main() {
    input! {n: usize, k: usize, mut lr: [(usize, usize); n]}
    lr.sort_by_key(|tup| tup.1);
    // dist means the minimum distance between
    let mut dist_l: usize = 0;
    let mut dist_r: usize = 1 << 32;
    while dist_r - dist_l > 1 {
        let dist_mid: usize = (dist_l + dist_r) / 2;
        if count_clothes(dist_mid, &lr) < k {
            dist_r = dist_mid;
        } else {
            dist_l = dist_mid;
        }
    }
    if dist_l == 0 {
        println!("{}", -1);
    } else {
        println!("{}", dist_l);
    }
}

fn count_clothes(min_dist: usize, lr: &Vec<(usize, usize)>) -> usize {
    let mut count: usize = 1;
    let mut cur_pos: usize = lr[0].1;
    let n: usize = lr.len();
    for i in 1..n {
        if cur_pos + min_dist <= lr[i].0 {
            cur_pos = lr[i].1;
            count += 1;
        }
    }
    return count;
}
