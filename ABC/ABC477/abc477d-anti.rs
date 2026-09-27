use proconio::input;

enum Query {
    Type1(usize),
    Type2(char),
}

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut queries = Vec::with_capacity(q);
    for _ in 0..q {
        input! { t: u8 }
        if t == 1 {
            input! { x: usize }
            queries.push(Query::Type1(x - 1));
        } else {
            input! { c: char }
            queries.push(Query::Type2(c));
        }
    }

    // 1. Group the time intervals during which each square has a tile
    let mut tile_intervals: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    let mut placed_at = vec![None; n];

    for (i, query) in queries.iter().enumerate() {
        if let Query::Type1(x) = *query {
            if let Some(start) = placed_at[x] {
                // Tile was removed at time i
                tile_intervals[x].push((start, i - 1));
                placed_at[x] = None;
            } else {
                // Tile was placed at time i
                placed_at[x] = Some(i);
            }
        }
    }

    for x in 0..n {
        if let Some(start) = placed_at[x] {
            tile_intervals[x].push((start, q - 1));
        }
    }

    // 2. Collect all Type 2 queries with their indices: (query_idx, color)
    let type2_queries: Vec<(usize, char)> = queries
        .iter()
        .enumerate()
        .filter_map(|(i, q)| match q {
            Query::Type2(c) => Some((i, *c)),
            _ => None,
        })
        .collect();

    // 3. For each square, find the latest Type 2 query in its empty intervals
    let mut ans = vec!['a'; n];

    for x in 0..n {
        let intervals = &tile_intervals[x];
        let mut empty_intervals = Vec::new();

        let mut curr = 0;
        for &(s, e) in intervals {
            if curr < s {
                empty_intervals.push((curr, s - 1));
            }
            curr = e + 1;
        }
        if curr < q {
            empty_intervals.push((curr, q - 1));
        }

        // Iterate empty intervals in reverse order (from latest to earliest)
        let mut found_color = None;
        for &(l, r) in empty_intervals.iter().rev() {
            let idx = type2_queries.partition_point(|&(t_idx, _)| t_idx <= r);
            if idx > 0 {
                let (last_t, c) = type2_queries[idx - 1];
                if last_t >= l {
                    found_color = Some(c);
                    break;
                }
            }
        }

        if let Some(c) = found_color {
            ans[x] = c;
        }
    }

    let out: String = ans.into_iter().collect();
    println!("{}", out);
}
