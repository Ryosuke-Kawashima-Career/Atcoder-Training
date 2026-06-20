use proconio::input;
fn alpha_to_num(letter: char) -> usize {
    match letter {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        _ => unreachable!(),
    }
}
fn main() {
    input! {n: usize, x: char, s: [Chars; n]}
    let mut is_ok: bool = false;
    for row in 0..n {
        if s[row][alpha_to_num(x)] == 'o' {
            is_ok = true;
        }
    }
    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
