use proconio::{input, marker::Chars};

fn solve() {
    input! {
        a_str: String,
        b_str: String,
    }

    let simplify = |s: String| {
        let mut votqi = Vec::new();
        for c in s.chars() {
            votqi.push(c);
            let n = votqi.len();
            if n >= 4 {
                // Check if the last 4 characters form a removable pair (xx)
                if votqi[n - 4] == '('
                    && votqi[n - 3] == 'x'
                    && votqi[n - 2] == 'x'
                    && votqi[n - 1] == ')'
                {
                    votqi.pop(); // pop ')'
                    votqi.pop(); // pop 'x'
                    votqi.pop(); // pop 'x'
                    votqi.pop(); // pop '('
                    votqi.push('x');
                    votqi.push('x');
                }
            }
        }
        votqi
    };

    if simplify(a_str) == simplify(b_str) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn main() {
    input! { t: usize }
    for _ in 0..t {
        solve();
    }
}
