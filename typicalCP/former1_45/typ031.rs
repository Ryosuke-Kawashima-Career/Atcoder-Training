use proconio::input;
#[derive(Debug, Clone, Copy)]
enum Operation {
    Type1,
    Type2(usize),
}
struct State {
    whites: Vec<usize>,
    blues: Vec<usize>,
}
impl State {
    fn execute(self, pos: usize, operation: Operation) -> bool {
        match operation {
            Type1 => {
                if self.whites[pos] == 0 {
                    return false;
                }
                self.blues[pos] += self.whites[pos];
                self.whites[pos] -= 1;
            }
            Type2(k) => {
                if k <= 0 || self.blues[pos] / 2 < k {
                    return false;
                }
                self.blues[pos] -= k;
            }
        }
        return true;
    }
}
fn main() {
    input! {n: usize, w: [usize; n], b: [usize; n]}
    let mut state = State {
        whites: w.clone(),
        blues: b.clone(),
    };
    dfs(&mut state);
}
