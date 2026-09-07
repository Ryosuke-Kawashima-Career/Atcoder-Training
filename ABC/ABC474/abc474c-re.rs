use proconio::input;
// ABC474C
// Q. Move the chosen value to the tail of permutation
// A. Bi-directional LinkedList
// In the implementation, you should careful about Null(which is 0)
fn main() {
    input! {n: usize, q: usize, p: [usize; n], aq: [usize; q]}
    // 0 means Null
    let mut prev: Vec<usize> = vec![0; n + 1];
    let mut next: Vec<usize> = vec![0; n + 1];
    for i in 0..n {
        if i > 0 {
            prev[p[i]] = p[i - 1];
        }
        if i < n - 1 {
            next[p[i]] = p[i + 1];
        }
    }
    let mut curr_tail: usize = p[n - 1];
    let mut curr_head: usize = p[0];
    for query in 0..q {
        let new_tail_label = aq[query];
        if new_tail_label == curr_tail {
            continue;
        }
        let prev_label: usize = prev[new_tail_label];
        let next_label: usize = next[new_tail_label];
        // if x is the head
        if curr_head == new_tail_label {
            curr_head = next[new_tail_label];
        }
        if prev_label != 0 {
            next[prev_label] = next_label;
        }
        if next_label != 0 {
            prev[next_label] = prev_label;
        }
        prev[new_tail_label] = curr_tail;
        next[curr_tail] = new_tail_label;
        next[new_tail_label] = 0;
        curr_tail = new_tail_label;
    }
    for _i in 0..n {
        print!("{} ", curr_head);
        curr_head = next[curr_head];
    }
    println!("");
}
