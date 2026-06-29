use proconio::input;

fn main() {
    input! {n: usize, mut hl: [(usize, usize); n], q: usize, times: [usize; q]}
    hl.sort_by_key(|tup| tup.1);
    let mut superiors: Vec<(usize, usize)> = Vec::new();
    for &(height, leaving_time) in hl.iter() {
        while let Some(&(prev_height, _prev_time)) = superiors.last() {
            if prev_height <= height {
                superiors.pop();
            } else {
                break;
            }
        }
        superiors.push((height, leaving_time));
    }
    for query in 0..q {
        let target_time: usize = times[query];
        let index: usize =
            superiors.partition_point(|&(_, leaving_time)| leaving_time <= target_time);
        if index < superiors.len() {
            println!("{}", superiors[index].0);
        } else {
            println!("{}", 0);
        }
    }
}
