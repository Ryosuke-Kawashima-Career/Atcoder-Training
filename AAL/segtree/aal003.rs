use proconio::input;

fn main() {
    input! {n: usize, q: usize, a: [i64; n], queries: [(usize, usize, i64); q]}
    let mut segtree = SegmentTree::new(&a);
    for query in 0..q {
        let query_type: usize = queries[query].0;
        if query_type == 1 {
            let (i, x) = (queries[query].1 - 1, queries[query].2);
            segtree.update(i, x);
        } else if query_type == 2 {
            let (l, t) = (queries[query].1 - 1, queries[query].2 - 1);
            let smallest: usize = segtree.binary_search_right(t, l);
            println!("{}", smallest + 1);
        } else {
            let (r, t) = (queries[query].1 - 1, queries[query].2 - 1);
            let largest: usize = segtree.binary_search_left(t, r);
            println!("{}", largest + 1);
        }
    }
}
