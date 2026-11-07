use proconio::input;

fn main() {
    input!{n: usize, q: usize, k: [usize; q]}
    let mut state: Vec<i64> = (1..=n as i64).collect();
    for query in 0..q {
        let mut new_state: Vec<i64> = Vec::new();
        for index in (0..k[query]).rev() {
            new_state.push(-state[index]);
        }
        for index in k[query]..n {
            new_state.push(state[index]);
        }
        state = new_state;
    }
    for i in 0..n {
        println!("{}", state[i]);
    }
}
