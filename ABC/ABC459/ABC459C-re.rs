use proconio::{input, marker::Usize1};
// ABC459 C: Frequency Queries
// Q. We have n cells, initially empty. We perform q queries of two types:
//    1. Place a block in cell x. If every cell has at least d blocks, we can remove one block from every cell (decrement d).
//    2. Find the number of cells with at least y blocks.
// A. We maintain three data structures:
//    1. a[i]: the total number of blocks placed in cell i.
//    2. freq[v]: the number of cells i such that a[i] == v.
//    3. suffix_sum[v]: the number of cells i such that a[i] >= v.
fn main() {
    input!{n: usize, q: usize}
    let mut blocks_in_cell: Vec<usize> = vec![0; n];
    // freq[number of blocks]
    let mut freq: Vec<usize> = vec![0; q + 2];
    let mut suffix_sum: Vec<usize> = vec![0; q + 2];
    freq[0] = n;
    suffix_sum[0] = n;
    let mut decrement: usize = 0;
    for _ in 0..q {
        input!{query_type: usize}
        if query_type == 1 {
            input!{cell_x: Usize1}
            freq[blocks_in_cell[cell_x]] -= 1;
            blocks_in_cell[cell_x] += 1;
            freq[blocks_in_cell[cell_x]] += 1;
            // suffix_sum does not decrease when we place a block, because the number of cells with at least v blocks can only increase or stay the same.
            suffix_sum[blocks_in_cell[cell_x]] += 1;
            // when all cells have at least decrement blocks, we can decrement all cells by 1
            if freq[decrement] == 0 {
                decrement += 1;
            }
        } else {
            input!{y: usize}
            let target: usize = y + decrement;
            // Check the boundary to avoid out-of-range access
            if target < suffix_sum.len() {
                println!("{}", suffix_sum[target]);
            } else {
                println!("0");
            }
        }
    }
}