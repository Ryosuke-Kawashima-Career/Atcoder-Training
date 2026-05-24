use proconio::input;

fn main() {
    input! { x: usize }
    let text: Vec<char> = "HelloWorld".chars().collect();
    for i in 0..text.len() {
        if i != x - 1 {
            print!("{}", text[i]);
        }
    }
    println!("");
}
