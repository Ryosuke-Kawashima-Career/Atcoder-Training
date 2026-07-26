use proconio::input;

fn main() {
    input! {n: usize, p: [usize; n], q: [usize; n]}
    let mut perm: Vec<usize> = (1..=n).collect();
    let mut index_p: isize = -1;
    let mut index_q: isize = -1;
    let mut current_index: isize = 0;
    loop {
        if p == perm {
            index_p = current_index;
        }
        if q == perm {
            index_q = current_index;
        }
        if next_permutation(&mut perm) {
            current_index += 1;
        } else {
            break;
        }
    }
    println!("{}", (index_q - index_p - 1).max(0));
}

fn next_permutation<T: Ord>(perm: &mut [T]) -> bool {
    /*
    1. Find the largest index i such that perm[i] < perm[i + 1]
    2. Find the largest index j > i such that perm[i] < perm[j]
    3. Swap perm[i] and perm[j]
    4. Reverse perm[i + 1 ..]
    */
    let n: usize = perm.len();
    if n < 2 {
        return false;
    }
    let mut i: usize = n - 2;
    // Search from the right edge
    while i > 0 && perm[i] >= perm[i + 1] {
        i -= 1;
    }
    // if all the permutations are explored
    if perm[i] >= perm[i + 1] {
        perm.reverse();
        return false;
    }
    // Search for the value bigger than perm[i]
    let mut j: usize = n - 1;
    while perm[i] >= perm[j] {
        j -= 1;
    }
    // Swap the value
    perm.swap(i, j);
    // Reverse the array to get the next permutation
    perm[i + 1..].reverse();
    return true;
}
