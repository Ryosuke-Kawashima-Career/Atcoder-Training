use proconio::input;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}
    let mut sorted_a: Vec<usize> = a.clone();
    sorted_a.sort();
    let mut incorrect_start: Option<usize> = None;
    let mut incorrect_end: Option<usize> = None;
    for i in 0..n {
        if sorted_a[i] != a[i] {
            match incorrect_start {
                Some(_) => incorrect_end = Some(i),
                None => incorrect_start = Some(i),
            }
        }
    }
    match (incorrect_start, incorrect_end) {
        (Some(start), Some(end)) => {
            let length: usize = end - start + 1;
            if length <= k {
                println!("Yes");
            } else {
                println!("No");
            }
        }
        (Some(start), None) => {
            let length: usize = n - start;
            if length <= k {
                println!("Yes");
            } else {
                println!("No");
            }
        }
        (None, None) => println!("Yes"),
        _ => unreachable!(),
    }
}
