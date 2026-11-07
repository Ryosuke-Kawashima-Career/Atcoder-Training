use proconio::input;
fn grundy_init() -> Vec<usize> {
    let mut grundy: Vec<usize> = vec![vec![0; 55]; 1555];
    for white in 0..=50 {
        for blue in 0..=1500 {
            let mut mex: Vec<usize> = vec![0; 1555];
            if white >= 1 {
                mex[grundy[white][blue]] = 1;
            }
            if blue >= 2 {
                for k in 1..=(blue/2) {
                    mex[grundy[i][j-k]] = 1;
                }
            }
            for k in 0..1555 {
                if mex[k] == 0 {
                    grundy[i][j] = k;
                    break;
                }
            }
        }
    }
}
fn main() {
    input!{n: usize, w: [usize; n], b: [usize; n]}
    let grundy = grundy_init();
    let mut nim_sum: usize = 0;
    for i in 0..n {
        nim_sum ^= grundy[w[i]][b[i]];
    }
    if nim_sum == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}