use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        adb: [(Usize1, usize, Usize1); n],
    }

    // day_to_query[day] stores the color changes happening on that day
    let mut day_to_query: Vec<Vec<(usize, usize)>> = vec![vec![]; m + 1];
    let mut count: Vec<usize> = vec![0; n];
    for &(color_a, day, color_b) in adb.iter() {
        if color_a != color_b {
            day_to_query[day].push((color_a, color_b));
        }
        count[color_a] += 1;
    }

    // Initial kinds of colors (on day 0, before day 1's transitions)
    let mut kinds: usize = count.iter().filter(|&&c| c > 0).count();

    for day in 1..=m {
        for &(before, after) in day_to_query[day].iter() {
            if count[before] == 1 {
                kinds -= 1;
            }
            count[before] -= 1;

            if count[after] == 0 {
                kinds += 1;
            }
            count[after] += 1;
        }
        println!("{}", kinds);
    }
}
