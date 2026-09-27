use proconio::input;

fn main() {
    input! {n: usize, q: usize}
    let mut colors: Vec<char> = vec!['a'; n];
    let mut is_occupied: Vec<bool> = vec![false; n];
    let mut queries: Vec<(Option<usize>, Option<char>)> = vec![];
    for _query in 0..q {
        input! {query_type: usize}
        if query_type == 1 {
            input! {x: usize}
            if !is_occupied[x - 1] {
                is_occupied[x - 1] = true;
                occupy_queries.push((Some(x - 1), None));
            } else {
                is_occupied[x - 1] = false;
                occupy_queries.push((Some(x - 1), None));
            }
        } else {
            input! {c: usize};
            queries.push((None, Some(c)));
        }
    }

    for _query in (0..q).rev() {
        match _query {
            (Some(x), None) => {}
            (None, Some(c)) => {}
            _ => unreachable!(),
        }
    }
}
