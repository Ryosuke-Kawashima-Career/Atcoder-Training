use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, m: usize, f: [Usize1; n]}
    let mut clothes: Vec<usize> = vec![0; m];
    let mut do_all_different: bool = true;
    for person in 0..n {
        if clothes[f[person]] != 0 {
            do_all_different = false;
        }
        clothes[f[person]] += 1;
    }
    let mut are_all_worn: bool = true;
    for cloth in 0..m {
        if clothes[cloth] == 0 {
            are_all_worn = false;
        }
    }
    if do_all_different {
        println!("Yes");
    } else {
        println!("No");
    }
    if are_all_worn {
        println!("Yes");
    } else {
        println!("No");
    }
}
