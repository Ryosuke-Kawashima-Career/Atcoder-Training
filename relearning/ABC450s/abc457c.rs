use proconio::input;

fn main() {
    input! {n: usize, k: usize}
    let mut a: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        input! {l: usize, a_cur: [usize; l]}
        a.push(a_cur);
    }
    input! {c: [usize; n]}
    let mut remain: usize = k - 1;
    for i in 0..n {
        let max_consume: usize = a[i].len() * c[i];
        let mut index: usize = a[i].len();
        if remain >= max_consume {
            remain -= max_consume;
            index -= 1;
        } else {
            index = remain % a[i].len();
            remain = 0;
        }
        if remain == 0 {
            println!("{}", a[i][index]);
            return;
        }
    }
}
