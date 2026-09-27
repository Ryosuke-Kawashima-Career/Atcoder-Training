use proconio::{input, marker::Usize1};
#[derive(Debug, Copy, Clone)]
enum Query {
    Type1(usize),
    Type2(char),
}
fn main() {
    input! {n: usize, q: usize}
    let mut queries: Vec<Query> = Vec::new();
    let mut is_covered: Vec<bool> = vec![false; n];
    for _query in 0..q {
        input! {query_type: usize}
        match query_type {
            1 => {
                input! {x: Usize1};
                queries.push(Query::Type1(x));
                is_covered[x] ^= true;
            }
            2 => {
                input! {c: char};
                queries.push(Query::Type2(c));
            }
            _ => unreachable!(),
        }
    }
    let mut cells_to_change: Vec<usize> = Vec::new();
    for i in 0..n {
        if !is_covered[i] {
            cells_to_change.push(i);
        }
    }
    let mut colors: Vec<char> = vec!['a'; n];
    let mut is_cell_used: Vec<bool> = vec![false; n];
    for query in queries.iter() {
        match query {
            Query::Type1(x) => {
                if !is_covered[*x] && !is_cell_used[*x] {
                    cells_to_change.push(*x);
                }
                if is_covered[*x] && !is_cell_used[*x] {
                    let index = cells_to_change.iter().position(|&r| r == *x).unwrap();
                    cells_to_change.remove(index);
                }
                is_covered[*x] ^= true;
            }
            Query::Type2(c) => {
                for cell in cells_to_change.iter() {
                    if !is_cell_used[*cell] {
                        colors[*cell] = *c;
                        is_cell_used[*cell] = true;
                    }
                }
                cells_to_change.clear();
            }
        }
    }
    println!("{}", colors.iter().collect::<String>());
}
