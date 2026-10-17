use proconio::input;

fn main() {
    input! {n: usize, l: i64, k: usize, a: [i64; n]}
    println!("{}", max_score(l, k, &a));
}

fn max_score(l: i64, k: usize, a: &[i64]) -> i64 {
    // left is feasible; right is infeasible.
    let mut left: i64 = 0;
    let mut right: i64 = l / (k as i64 + 1) + 1;
    while right - left > 1 {
        let mid: i64 = (left + right) / 2;
        if count_pieces(mid, l, a) >= k + 1 {
            left = mid;
        } else {
            right = mid;
        }
    }
    left
}

fn count_pieces(x: i64, l: i64, a: &[i64]) -> usize {
    let mut cnt = 0;
    let mut last = 0;
    // Greedily finish each piece at the earliest possible endpoint.
    // Include L: the final piece ends at the right edge, not at A_N.
    for position in a.iter().copied().chain(std::iter::once(l)) {
        if position - last >= x {
            cnt += 1;
            last = position;
        }
    }
    // Any short remaining tail can be merged into the preceding piece.
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_and_boundaries() {
        assert_eq!(max_score(34, 1, &[8, 13, 26]), 13);
        assert_eq!(max_score(45, 2, &[7, 11, 16, 20, 28, 34, 38]), 12);
        assert_eq!(max_score(10, 1, &[9]), 1);
        assert_eq!(max_score(10, 3, &[2, 5, 9]), 1);
        assert_eq!(max_score(1_000_000_000, 1, &[500_000_000]), 500_000_000);
    }

    #[test]
    fn compare_with_exhaustive_cut_selection() {
        for l in 2..=10i64 {
            for available in 1usize..(1usize << (l - 1)) {
                let a: Vec<i64> = (1..l)
                    .filter(|&p| available & (1usize << (p - 1)) != 0)
                    .collect();
                for k in 1..=a.len() {
                    let mut expected = 0;
                    for chosen in 0usize..(1usize << a.len()) {
                        if chosen.count_ones() as usize != k {
                            continue;
                        }
                        let mut last = 0;
                        let mut shortest = l;
                        for (i, &p) in a.iter().enumerate() {
                            if chosen & (1usize << i) != 0 {
                                shortest = shortest.min(p - last);
                                last = p;
                            }
                        }
                        shortest = shortest.min(l - last);
                        expected = expected.max(shortest);
                    }
                    assert_eq!(max_score(l, k, &a), expected);
                }
            }
        }
    }
}
