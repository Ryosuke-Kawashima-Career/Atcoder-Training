use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, m: usize, cs: [(Usize1, isize); n]}
    let mut sizes_color: Vec<isize> = vec![-1; m];
    for i in 0..n {
        let (color, size) = cs[i];
        sizes_color[color] = sizes_color[color].max(size);
    }

    for k in 0..m {
        if sizes_color[k] == -1 {
            print!("-1 ");
        } else {
            print!("{} ", sizes_color[k]);
        }
    }
    println!("");
}
