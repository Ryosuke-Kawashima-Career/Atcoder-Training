use proconio::input;

fn main() {
    input! {n: usize}
    let mut good_integers = get_good_integers(n);
    good_integers.sort();
    let k: usize = good_integers.len();
    println!("{}", k);
    for i in 0..k {
        print!("{} ", good_integers[i]);
    }
    println!("");
}

fn get_good_integers(n: usize) -> Vec<usize> {
    let mut num: usize = 1;
    let mut good_integers: Vec<usize> = Vec::new();
    let mut squares: Vec<usize> = Vec::new();
    while num * num <= n {
        squares.push(num * num);
        num += 1;
    }

    let mut set = std::collections::HashSet::new();
    let n_squares: usize = squares.len();
    for i in 0..n_squares {
        for j in (i + 1)..n_squares {
            if squares[i] + squares[j] <= n {
                set.insert(squares[i] + squares[j]);
            } else {
                break;
            }
        }
    }
    set.into_iter().collect::<Vec<usize>>()
}
