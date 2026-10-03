use proconio::input;
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
        if query_type == 1 {
            input! {x: usize}
            queries.push(Query::Type1(x - 1));
            is_covered[x - 1] = true;
        } else {
            input! {y: char}
            queries.push(Query::Type2(y));
        }
    }

    let mut last_query2: Vec<Option<Query>> = vec![None; n];
    for &query in queries.iter().rev() {
        match query {
            Query::Type1(x) => {
                is_covered[x - 1] = !is_covered[x - 1];
            }
            Query::Type2(y) => {
                for cell in 0..n {
                    match last_query2[cell] {
                        Some(query2) => {
                            continue;
                        }
                        None => {
                            last_query2[cell] = Some(query);
                        }
                    }
                }
            }
        }
    }

    for cell in 0..n {
        match last_query2[cell] {
            Some(Query::Type2(y)) => {
                print!("{}", y);
            }
            None => {
                print!("a");
            }
            _ => unreachable!(),
        }
    }
    println!("");
}
