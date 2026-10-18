use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        t: i64,
        mut a: [i64; n],
    }

    // Sort Aoki's press times in chronological order
    a.sort_unstable();

    let mut cur_end: i64 = 0; // Water is continuously covered across [0, cur_end]
    let mut ans: i64 = 0;

    for &ai in &a {
        if ai >= t {
            break; // Presses at or after T cannot help cover any uncovered gap in [0, T)
        }

        if ai > cur_end {
            // Uncovered gap [cur_end, ai] must be filled by Takahashi
            let gap = ai - cur_end;
            ans += (gap + k - 1) / k;
        }

        // Aoki's press at `ai` guarantees water flows until at least `ai + k`
        cur_end = ai + k;
    }

    // Cover any remaining gap [cur_end, t]
    if cur_end < t {
        let gap = t - cur_end;
        ans += (gap + k - 1) / k;
    }

    println!("{}", ans);
}
