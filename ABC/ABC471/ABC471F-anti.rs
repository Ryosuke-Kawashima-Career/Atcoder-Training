use proconio::input;

fn cmp_custom(a: &str, b: &str) -> std::cmp::Ordering {
    let mut ab = [0u8; 20];
    let mut ba = [0u8; 20];
    let len_a = a.len();
    let len_b = b.len();
    let tot = len_a + len_b;
    ab[..len_a].copy_from_slice(a.as_bytes());
    ab[len_a..tot].copy_from_slice(b.as_bytes());

    ba[..len_b].copy_from_slice(b.as_bytes());
    ba[len_b..tot].copy_from_slice(a.as_bytes());
    ba[..tot].cmp(&ab[..tot])
}

fn max_possible(d: usize, rem_k: usize, cnt: &[usize; 11]) -> usize {
    let mut rem = rem_k;
    let mut sum = 0;
    for len in (1..=d).rev() {
        let take = cnt[len].min(rem);
        sum += take * len;
        rem -= take;
        if rem == 0 {
            break;
        }
    }
    sum
}

fn min_possible(d: usize, rem_k: usize, cnt: &[usize; 11]) -> usize {
    let mut rem = rem_k;
    let mut sum = 0;
    for len in 1..=d {
        let take = cnt[len].min(rem);
        sum += take * len;
        rem -= take;
        if rem == 0 {
            break;
        }
    }
    sum
}

fn dfs(
    d: usize,
    rem_k: usize,
    cur_len: usize,
    target_len: usize,
    cnt: &[usize; 11],
    current_c: &mut [usize; 11],
    valid_tuples: &mut Vec<[usize; 11]>,
) {
    if d == 0 {
        if rem_k == 0 && cur_len == target_len {
            valid_tuples.push(*current_c);
        }
        return;
    }
    if rem_k == 0 {
        if cur_len == target_len {
            valid_tuples.push(*current_c);
        }
        return;
    }
    if cur_len + max_possible(d, rem_k, cnt) < target_len {
        return;
    }
    if cur_len + min_possible(d, rem_k, cnt) > target_len {
        return;
    }

    let max_take = cnt[d].min(rem_k);
    for take in (0..=max_take).rev() {
        current_c[d] = take;
        dfs(
            d - 1,
            rem_k - take,
            cur_len + take * d,
            target_len,
            cnt,
            current_c,
            valid_tuples,
        );
        current_c[d] = 0;
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [String; n],
    }

    let is_all_zero = s.iter().all(|x| x.chars().all(|c| c == '0'));
    if is_all_zero {
        println!("0");
        return;
    }

    let mut lists: Vec<Vec<String>> = vec![vec![]; 11];
    let mut cnt = [0usize; 11];

    for str_val in s {
        let l = str_val.len();
        lists[l].push(str_val);
    }

    for d in 1..=10 {
        lists[d].sort_by(|a, b| b.cmp(a));
        cnt[d] = lists[d].len();
    }

    let max_len = max_possible(10, k, &cnt);

    for target_len in (1..=max_len).rev() {
        let mut valid_tuples = Vec::new();
        let mut current_c = [0usize; 11];
        dfs(
            10,
            k,
            0,
            target_len,
            &cnt,
            &mut current_c,
            &mut valid_tuples,
        );

        let mut best_str: Option<String> = None;

        for tuple in &valid_tuples {
            let mut has_positive = false;
            for d in 1..=10 {
                for i in 0..tuple[d] {
                    if lists[d][i].chars().any(|c| c != '0') {
                        has_positive = true;
                        break;
                    }
                }
                if has_positive {
                    break;
                }
            }

            if !has_positive {
                continue;
            }

            let mut chosen = Vec::with_capacity(k);
            for d in 1..=10 {
                for i in 0..tuple[d] {
                    chosen.push(&lists[d][i]);
                }
            }

            chosen.sort_by(|a, b| cmp_custom(a, b));

            let mut cand = String::with_capacity(target_len);
            for str_ref in chosen {
                cand.push_str(str_ref);
            }

            match &best_str {
                None => best_str = Some(cand),
                Some(prev) => {
                    if &cand > prev {
                        best_str = Some(cand);
                    }
                }
            }
        }

        if let Some(res) = best_str {
            let trimmed = res.trim_start_matches('0');
            if trimmed.is_empty() {
                println!("0");
            } else {
                println!("{}", trimmed);
            }
            return;
        }
    }

    println!("0");
}
