use proconio::input;

fn main() {
    input!{n: usize, a: [usize; n], b: [usize; n]}
    let mut initial_diff_pos: Option<usize> = None;
    let mut last_diff_pos: Option<usize> = None;
    for i in 0..n {
        if a[i] != b[i] {
            initial_diff_pos = Some(i);
            break;
        }
    }
    for i in (0..n).rev() {
        if a[i] != b[i] {
            last_diff_pos = Some(i);
            break;
        }
    }
    if initial_diff_pos.is_none() && last_diff_pos.is_none() {
        println!("Yes");
        return;
    }
    let is_adjustable: bool = insert(&a, &b, initial_diff_pos.unwrap(), last_diff_pos.unwrap());
    if is_adjustable {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn insert(a: &Vec<usize>, b: &Vec<usize>, initial_diff_pos: usize, last_diff_pos: usize) -> bool {
    let n: usize = a.len();
    if initial_diff_pos == last_diff_pos {
        return false;
    }

    if a[last_diff_pos] == b[initial_diff_pos] {
        let mut all_same: bool = true;
        for i in initial_diff_pos..last_diff_pos {
            if a[i] != b[i+1] {
                all_same = false;
            }
        }
        if all_same {
            return true;
        }
    }
    if a[initial_diff_pos] == b[last_diff_pos] {
        let mut all_same: bool = true;
        for i in initial_diff_pos..last_diff_pos {
            if a[i+1] != b[i] {
                all_same = false;
            }
        }
        if all_same {
            return true;
        }
    }
    return false;
}
