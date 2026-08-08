use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }
    let mut index_p: isize = -1;
    let mut index_q: isize = -1;
    let mut array: Vec<usize> = (1..=n).collect();
    let mut cur_index: isize = 0;

    loop {
        if array == p {
            index_p = cur_index;
        }
        if array == q {
            index_q = cur_index;
        }
        if !next_permutation(&mut array) {
            break;
        }
        cur_index += 1;
    }
    println!("{}", (index_p - index_q).abs());
}

fn next_permutation(array: &mut [usize]) -> bool {
    let n = array.len();
    if n <= 1 {
        return false;
    }
    let mut i = n - 1;
    while i > 0 && array[i - 1] >= array[i] {
        i -= 1;
    }
    if i == 0 {
        array.reverse();
        return false;
    }
    let mut j = n - 1;
    while array[j] <= array[i - 1] {
        j -= 1;
    }
    array.swap(i - 1, j);
    array[i..].reverse();
    true
}
