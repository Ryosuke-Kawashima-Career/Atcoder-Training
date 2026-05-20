use proconio::input;

fn main() {
    input! {h: usize, w: usize}
    if h == 1 && w == 1 {
        println!("0");
        return;
    }
    if h == 1 {
        for j in 0..w {
            if j == 0 || j == w - 1 {
                print!("1 ");
            } else {
                print!("2 ");
            }
        }
        println!("");
        return;
    }
    if w == 1 {
        for i in 0..h {
            if i == 0 || i == h - 1 {
                print!("1 ");
            } else {
                print!("2 ");
            }
        }
        println!("");
        return;
    }

    for i in 0..h {
        for j in 0..w {
            if i == 0 && (j == 0 || j == w - 1) {
                print!("{} ", 2);
            } else if i == h - 1 && (j == 0 || j == w - 1) {
                print!("{} ", 2);
            } else if i == 0 || i == h - 1 {
                print!("{} ", 3);
            } else if j == 0 || j == w - 1 {
                print!("{} ", 3);
            } else {
                print!("{} ", 4);
            }
        }
        println!("");
    }
}
