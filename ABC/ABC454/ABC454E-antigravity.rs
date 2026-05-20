use proconio::{input, marker::Usize1};

fn solve() {
    input! {
        n: usize,
        a: Usize1,
        b: Usize1,
    }

    if n % 2 != 0 || (a + b) % 2 == 0 {
        println!("No");
        return;
    }

    println!("Yes");
    let mut votqi = String::with_capacity(n * n);
    let h = b / 2;

    for s in 0..(n / 2) {
        if s < h {
            for _ in 0..(n - 1) { votqi.push('D'); }
            votqi.push('R');
            for _ in 0..(n - 1) { votqi.push('U'); }
        } else if s > h {
            for _ in 0..(n - 1) { votqi.push('U'); }
            votqi.push('R');
            for _ in 0..(n - 1) { votqi.push('D'); }
        } else {
            let l = 2 * s;
            let r = 2 * s + 1;
            let mut curr_c = l;
            for row in 0..n {
                if row < a {
                    if row % 2 == 0 {
                        votqi.push('R');
                        curr_c = r;
                    } else {
                        votqi.push('L');
                        curr_c = l;
                    }
                } else if row > a {
                    if row % 2 == 0 {
                        votqi.push('L');
                        curr_c = l;
                    } else {
                        votqi.push('R');
                        curr_c = r;
                    }
                }
                
                if row < n - 1 {
                    votqi.push('D');
                }
            }
        }
        
        if s < n / 2 - 1 {
            votqi.push('R');
        }
    }
    println!("{}", votqi);
}

fn main() {
    input! { t: usize }
    for _ in 0..t {
        solve();
    }
}
