use proconio::input;
use proconio::marker::Usize1;
// ABC470D
// Pointer and Source array
fn main() {
    input! {n: usize, q: usize, mut p: [Usize1; n]}
    // pointer -> source
    let mut source: Vec<usize> = vec![n; n];
    for i in 0..n {
        source[p[i]] = i;
    }
    let mut inversed: bool = false;
    for _query in 0..q {
        input! {query_type: usize}
        if query_type == 1 {
            input! {x: Usize1, y: Usize1}
            if !inversed {
                let pointer1: usize = p[x];
                let pointer2: usize = p[y];
                p.swap(x, y);
                source.swap(pointer1, pointer2);
            } else {
                let source_x: usize = source[x];
                let source_y: usize = source[y];
                source.swap(x, y);
                p.swap(source_x, source_y);
            }
        } else {
            inversed = !inversed;
        }
    }
    if inversed {
        for i in 0..n {
            print!("{} ", source[i] + 1);
        }
    } else {
        for i in 0..n {
            print!("{} ", p[i] + 1);
        }
    }

    println!();
}
