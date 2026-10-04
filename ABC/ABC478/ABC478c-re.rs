use proconio::input;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}
    let mut sorted_a: Vec<usize> = a.clone();
    sorted_a.sort();
    let mut count: usize = 0;
    let mut index_start_incorrect: isize = -1;
    let mut index_end_incorrect: isize = -1;
    for i in 0..n {
        if a[i] != sorted_a[i] {
            if index_start_incorrect == -1 {
                index_start_incorrect = i as isize;
            } else {
                index_end_incorrect = i as isize;
            }
        }
    }
    if index_start_incorrect == -1 || index_end_incorrect == -1 {
        println!("Yes");
    } else {
        let length_of_incorrect: isize = index_end_incorrect - index_start_incorrect + 1;
        if length_of_incorrect <= k as isize {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
