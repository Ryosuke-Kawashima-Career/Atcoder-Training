use proconio::input;
fn main() {
    input! {
        n: usize,
        q: usize,
        mut lrx: [(usize, usize, usize); q],
    }
    lrx.sort_unstable_by_key(|&(l, _, x)| (x, l));

    let mut diff = vec![0i32; n + 2];

    let mut i = 0;
    while i < q {
        let x = lrx[i].2;
        let mut cur_l = lrx[i].0;
        let mut cur_r = lrx[i].1;
        i += 1;
        while i < q && lrx[i].2 == x {
            let (l, r, _) = lrx[i];
            if l <= cur_r + 1 {
                cur_r = cur_r.max(r);
            } else {
                diff[cur_l] += 1;
                diff[cur_r + 1] -= 1;
                cur_l = l;
                cur_r = r;
            }
            i += 1;
        }

        diff[cur_l] += 1;
        diff[cur_r + 1] -= 1;
    }
    let mut cur = 0;
    for i in 1..=n {
        cur += diff[i];
        if i > 1 {
            print!(" ");
        }
        print!("{}", cur);
    }
    println!("");
}
