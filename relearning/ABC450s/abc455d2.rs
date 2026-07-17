use proconio::{input, marker::Usize1};
const NIL: usize = usize::MAX;
// abc455d
// Q. Move card "c" to the top of card "p"
// A. Two way pointers
fn main() {
    input! {n: usize, q: usize, cp: [(Usize1, Usize1); q]}
    // two way pointers
    let mut up: Vec<usize> = vec![NIL; n << 1];
    let mut down: Vec<usize> = vec![NIL; n << 1];
    for i in 0..n {
        up[n + i] = i;
        down[i] = n + i;
    }
    for &(card_c, card_p) in cp.iter() {
        let down_of_c: usize = down[card_c];
        up[down_of_c] = NIL;
        up[card_p] = card_c;
        down[card_c] = card_p;
    }
    for i in 0..n {
        let mut count: usize = 0;
        let mut current: usize = n + i;
        while up[current] != NIL {
            count += 1;
            current = up[current];
        }
        print!("{} ", count);
    }
    println!("");
}
