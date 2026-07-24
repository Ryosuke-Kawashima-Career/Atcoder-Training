use proconio::input;

fn main() {
    input! {n: usize, m: usize, mut bird_color_days: [(usize, usize, usize); n]}
    bird_color_days.sort_by_key(|tup| tup.1);
    let mut colors_of_birds: Vec<Vec<usize>> = vec![0; n];
    let mut colors_count: Vec<usize> = vec![0; n + 1];
    let mut cur_index: usize = 0;
    let mut different_colors: usize = 0;
    for i in 0..n {
        colors_count[bird_color_days[i].0] += 1;
        if colors_count[bird_color_days[i].0] == 1 {
            different_colors += 1;
        }
    }
    for day in 1..=m {
        while cur_index < n && bird_color_days[cur_index].1 <= day {
            let pre_color: usize = bird_color_days[cur_index].0;
            let post_color: usize = bird_color_days[cur_index].2;
            colors_count[pre_color] -= 1;
            if colors_count[pre_color] == 0 {
                different_colors -= 1;
            }
            colors_count[post_color] += 1;
            if colors_count[post_color] == 1 {
                different_colors += 1;
            }
            cur_index += 1;
        }
        println!("{}", different_colors);
    }
}
