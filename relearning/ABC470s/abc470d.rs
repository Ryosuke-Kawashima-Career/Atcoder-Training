use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, q: usize, mut pointers: [Usize1; n]}
    let mut sources: Vec<usize> = vec![n; n];
    for source in 0..n {
        sources[pointers[source]] = source;
    }
    let mut is_reversed: bool = false;
    for _query in 0..q {
        input! {query_type: usize}
        if query_type == 1 {
            input! {x: Usize1, y: Usize1}
            if is_reversed {
                // x and y are pointers
                let x_source: usize = sources[x];
                let y_source: usize = sources[y];
                pointers[x_source] = y;
                pointers[y_source] = x;
                sources[x] = y_source;
                sources[y] = x_source;
            } else {
                // x and y are sources
                let x_pointer: usize = pointers[x];
                let y_pointer: usize = pointers[y];
                pointers[x] = y_pointer;
                pointers[y] = x_pointer;
                sources[x_pointer] = y;
                sources[y_pointer] = x;
            }
        } else {
            is_reversed = !is_reversed;
        }
    }
    if is_reversed {
        for i in 0..n {
            print!("{} ", sources[i] + 1);
        }
    } else {
        for i in 0..n {
            print!("{} ", pointers[i] + 1);
        }
    }
    println!("");
}
