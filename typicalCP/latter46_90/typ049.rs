use proconio::input;
// Typical CP 049
// Q. Turn S(0000..) into T(bit 000..=1111..) and what is the minimum cost to achieve all the combination of bits
// A. Solve this by Kruskal algorithm
const INF: i64 = 1 << 60;
fn main() {
    input!{n: usize, m: usize, clr: [(i64, usize, usize); m]}
    let bits: usize = 1 << n;
    
}
