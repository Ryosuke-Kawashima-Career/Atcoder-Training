use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        c: [Usize1; n],
    }
    let rle: Vec<(usize, usize, usize)> = get_rle(&c);
    for i in 0..n {
        let dist: usize = get_nearest(&rle, i);
        print!("{}{}", dist, if i + 1 == n { "\n" } else { " " });
    }
}

fn get_nearest(rle: &[(usize, usize, usize)], idx: usize) -> usize {
    let n_rle: usize = rle.len();
    let range_idx: usize = rle.partition_point(|&(_color, start, _length)| start <= idx) - 1;
    let (_color, start, length) = rle[range_idx];
    if range_idx == 0 {
        return start + length - idx;
    }
    if range_idx == n_rle - 1 {
        return idx - start + 1;
    }
    (idx - start + 1).min(start + length - idx)
}

fn get_rle(arr: &[usize]) -> Vec<(usize, usize, usize)> {
    let n: usize = arr.len();
    let mut i: usize = 0;
    let mut rle: Vec<(usize, usize, usize)> = Vec::new();
    while i < n {
        let mut j: usize = i;
        while j < n && arr[i] == arr[j] {
            j += 1;
        }
        rle.push((arr[i], i, j - i));
        i = j;
    }
    rle
}
