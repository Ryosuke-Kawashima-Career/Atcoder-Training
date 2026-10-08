use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n]}
    let mut stack: Vec<usize> = Vec::new();
    for i in 0..n {
        stack.push(a[i]);
        while stack.len() >= 4
            && stack[stack.len() - 4] == stack[stack.len() - 3]
            && stack[stack.len() - 3] == stack[stack.len() - 2]
            && stack[stack.len() - 2] == stack[stack.len() - 1]
        {
            stack.pop();
            stack.pop();
            stack.pop();
            stack.pop();
        }
    }
    let ans: usize = stack.len();
    println!("{}", ans);
}
