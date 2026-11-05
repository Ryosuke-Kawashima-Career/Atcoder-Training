use std::io::{self, BufRead, BufWriter, Write};

const N: usize = 64;
const K: usize = 192;
const P: usize = 256;
const R: usize = 1024;
const H: usize = 14;

#[derive(Clone)]
struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 88172645463325252 } else { seed },
        }
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
    fn gen_range(&mut self, n: usize) -> usize {
        (self.next_u64() as usize) % n
    }
}

fn is_visible(stack: &[usize], sheets: &[[u8; N]; K], target_s: usize) -> bool {
    for w in 0..N {
        for &s in stack {
            let c = sheets[s][w];
            if c != b'.' {
                if s == target_s {
                    return true;
                }
                break;
            }
        }
    }
    false
}

fn visible_sheet_at(stack: &[usize], sheets: &[[u8; N]; K], w: usize) -> usize {
    for &s in stack {
        if sheets[s][w] != b'.' {
            return s;
        }
    }
    usize::MAX
}

fn apply_op(
    stack: &mut Vec<usize>,
    sheets: &[[u8; N]; K],
    k: usize,
    d: usize,
    ops: &mut Vec<(usize, usize)>,
) {
    debug_assert!(is_visible(stack, sheets, k));
    let pos = stack.iter().position(|&x| x == k).unwrap();
    if pos == d {
        return;
    }
    stack.remove(pos);
    stack.insert(d, k);
    ops.push((k, d));
}

fn pull_sheet_to_depth(
    target_s: usize,
    target_d: usize,
    stack: &mut Vec<usize>,
    sheets: &[[u8; N]; K],
    handle_win: &[usize; K],
    handle_bit: &[u8; K],
    ops: &mut Vec<(usize, usize)>,
) {
    if stack[target_d] == target_s {
        return;
    }
    let w = handle_win[target_s];
    let t_bit = handle_bit[target_s];

    while !is_visible(stack, sheets, target_s) {
        // Find topmost sheet in queue w
        let mut top_q = usize::MAX;
        let mut top_pos = usize::MAX;
        for (idx, &s) in stack.iter().enumerate() {
            if handle_win[s] == w {
                top_q = s;
                top_pos = idx;
                break;
            }
        }
        let target_pos = stack.iter().position(|&x| x == target_s).unwrap();
        assert!(top_pos < target_pos, "top_q must be above target_s");

        if handle_bit[top_q] != t_bit {
            // Insert top_q immediately below target_s
            apply_op(stack, sheets, top_q, target_pos, ops);
        } else {
            // Find first sheet in queue w below top_q with opposite bit
            let mut opp_pos = None;
            for (idx, &s) in stack.iter().enumerate().skip(top_pos + 1) {
                if handle_win[s] == w && handle_bit[s] != t_bit {
                    opp_pos = Some(idx);
                    break;
                }
            }
            let d = match opp_pos {
                Some(op) => target_pos.max(op),
                None => K - 1,
            };
            apply_op(stack, sheets, top_q, d, ops);
        }
    }

    if stack[target_d] != target_s {
        apply_op(stack, sheets, target_s, target_d, ops);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let first_line = match lines.next() {
        Some(Ok(l)) => l,
        _ => return,
    };
    let mut parts = first_line.split_whitespace();
    let _n: usize = parts.next().unwrap().parse().unwrap();
    let _k: usize = parts.next().unwrap().parse().unwrap();
    let _p: usize = parts.next().unwrap().parse().unwrap();
    let r_turns: usize = parts.next().unwrap().parse().unwrap();

    let mut a_bytes = [[b'0'; N]; P];
    let mut handle_a = [0u64; P];
    let mut data_a = [0u64; P];

    for p in 0..P {
        let line = lines.next().unwrap().unwrap();
        let bytes = line.trim().as_bytes();
        let mut hm = 0u64;
        let mut dm = 0u64;
        for w in 0..N {
            a_bytes[p][w] = bytes[w];
            if bytes[w] == b'1' {
                if w < H {
                    hm |= 1u64 << w;
                } else {
                    dm |= 1u64 << (w - H);
                }
            }
        }
        handle_a[p] = hm;
        data_a[p] = dm;
    }

    // Step 1: Greedily pair 64 closest pairs (j1, j2) on data zone [H..N)
    let mut cand_pairs = Vec::with_capacity(P * (P - 1) / 2);
    for u in 0..P {
        for v in (u + 1)..P {
            let agree_h = H as u32 - (handle_a[u] ^ handle_a[v]).count_ones();
            if agree_h >= 4 {
                let d_dist = (data_a[u] ^ data_a[v]).count_ones();
                cand_pairs.push((d_dist, u, v));
            }
        }
    }
    cand_pairs.sort_unstable_by_key(|&(d, _, _)| d);

    let mut used = [false; P];
    let mut pairs = Vec::with_capacity(64);
    for &(_, u, v) in &cand_pairs {
        if !used[u] && !used[v] && pairs.len() < 64 {
            used[u] = true;
            used[v] = true;
            pairs.push((u, v));
        }
    }

    let mut rem: Vec<usize> = (0..P).filter(|&u| !used[u]).collect();
    let mut j_list = [0usize; 128];
    let mut d_masks = [0u64; 128];
    for g in 0..64 {
        let (j1, j2) = pairs[g];
        let d = data_a[j1] ^ data_a[j2];
        j_list[2 * g] = j1;
        j_list[2 * g + 1] = j2;
        d_masks[2 * g] = d;
        d_masks[2 * g + 1] = d;
    }

    // Step 2: Match 128 i-barcodes to 128 j-barcodes
    let base_cost = |i_idx: usize, slot: usize| -> i64 {
        let j = j_list[slot];
        let d = d_masks[slot];
        let agree_h = H as u32 - (handle_a[i_idx] ^ handle_a[j]).count_ones();
        if agree_h < 3 {
            return 1_000_000;
        }
        let h = ((data_a[i_idx] ^ data_a[j]) & d).count_ones() as i64;
        h * h * 10 + if h >= 5 { 500 } else { 0 } + if h >= 6 { 5000 } else { 0 }
    };

    for _ in 0..40 {
        let mut improved = false;
        for s1 in 0..128 {
            let mut c1 = base_cost(rem[s1], s1);
            for s2 in (s1 + 1)..128 {
                let i1 = rem[s1];
                let i2 = rem[s2];
                let c2 = base_cost(i2, s2);
                let nc1 = base_cost(i2, s1);
                let nc2 = base_cost(i1, s2);
                if nc1 + nc2 < c1 + c2 {
                    rem.swap(s1, s2);
                    c1 = nc1;
                    improved = true;
                }
            }
        }
        if !improved {
            break;
        }
    }

    let full_data_mask = (1u64 << (N - H)) - 1;
    let compute_sheet_masks = |match_i: &[usize]| -> ([u64; K], [u64; K]) {
        let mut valid_m = [0u64; K];
        let mut val_m = [0u64; K];
        for g in 0..64 {
            let (j1, j2) = pairs[g];
            let d = d_masks[2 * g];
            valid_m[3 * g] = full_data_mask ^ d;
            val_m[3 * g] = data_a[j1] & (full_data_mask ^ d);
            for r in 0..2 {
                let ir = match_i[2 * g + r];
                let jr = if r == 0 { j1 } else { j2 };
                let hm = (data_a[ir] ^ data_a[jr]) & d;
                valid_m[3 * g + 1 + r] = full_data_mask ^ hm;
                val_m[3 * g + 1 + r] = data_a[ir] & (full_data_mask ^ hm);
            }
        }
        (valid_m, val_m)
    };

    let exact_slot_cost =
        |i_idx: usize, slot: usize, valid_m: &[u64; K], val_m: &[u64; K]| -> i64 {
            let j = j_list[slot];
            let d = d_masks[slot];
            let agree_h = H as u32 - (handle_a[i_idx] ^ handle_a[j]).count_ones();
            if agree_h < 3 {
                return 10_000_000;
            }
            let hm = (data_a[i_idx] ^ data_a[j]) & d;
            let h = hm.count_ones() as i64;
            let ti = data_a[i_idx] & hm;
            let tj = data_a[j] & hm;
            let g_self = slot / 2;
            let mut ci = 0i64;
            let mut cj = 0i64;
            for b in 0..K {
                if b / 3 == g_self {
                    continue;
                }
                if (valid_m[b] & hm) == hm {
                    let vb = val_m[b] & hm;
                    if vb == ti {
                        ci += 1;
                    }
                    if vb == tj {
                        cj += 1;
                    }
                }
            }
            let mc = ci.min(cj);
            let pen = if mc < 6 { (6 - mc) * (6 - mc) * 3000 } else { 0 };
            pen + h * h * 25 - mc * 2
        };

    for _ in 0..15 {
        let (valid_m, val_m) = compute_sheet_masks(&rem);
        let mut improved = false;
        for s1 in 0..128 {
            let mut c1 = exact_slot_cost(rem[s1], s1, &valid_m, &val_m);
            for s2 in (s1 + 1)..128 {
                let i1 = rem[s1];
                let i2 = rem[s2];
                let c2 = exact_slot_cost(i2, s2, &valid_m, &val_m);
                let nc1 = exact_slot_cost(i2, s1, &valid_m, &val_m);
                let nc2 = exact_slot_cost(i1, s2, &valid_m, &val_m);
                if nc1 + nc2 < c1 + c2 {
                    rem.swap(s1, s2);
                    c1 = nc1;
                    improved = true;
                }
            }
        }
        if !improved {
            break;
        }
    }

    let (valid_m, val_m) = compute_sheet_masks(&rem);

    // Step 3: Assign Handle Windows [0..H) to the 192 conceptual sheets
    // Conceptual sheet index: 3*g = U_g, 3*g+1 = S_{g,0}, 3*g+2 = S_{g,1}
    let mut allowed_wins: Vec<Vec<(usize, u8)>> = vec![Vec::new(); K];
    for g in 0..64 {
        let (j1, j2) = pairs[g];
        for w in 0..H {
            if a_bytes[j1][w] == a_bytes[j2][w] {
                allowed_wins[3 * g].push((w, a_bytes[j1][w]));
            }
        }
        for r in 0..2 {
            let ir = rem[2 * g + r];
            let jr = if r == 0 { j1 } else { j2 };
            for w in 0..H {
                if a_bytes[ir][w] == a_bytes[jr][w] {
                    allowed_wins[3 * g + 1 + r].push((w, a_bytes[ir][w]));
                }
            }
        }
    }

    // Initial greedy balanced assignment of handle windows
    let mut c_handle_win = [0usize; K];
    let mut c_handle_bit = [b'0'; K];
    let mut win_bit_counts = [[0i32; 2]; H];

    for g in 0..64 {
        // Assign U_g, S_{g,0}, S_{g,1} to distinct handle windows
        let mut best_cost = i32::MAX;
        let mut best_triplet = (0, 0, 0);
        let mut best_bits = (b'0', b'0', b'0');
        for &(w0, b0) in &allowed_wins[3 * g] {
            let c0 = win_bit_counts[w0][(b0 - b'0') as usize] * 10
                + (win_bit_counts[w0][0] + win_bit_counts[w0][1]) * 3;
            for &(w1, b1) in &allowed_wins[3 * g + 1] {
                if w1 == w0 {
                    continue;
                }
                let c1 = win_bit_counts[w1][(b1 - b'0') as usize] * 10
                    + (win_bit_counts[w1][0] + win_bit_counts[w1][1]) * 3;
                for &(w2, b2) in &allowed_wins[3 * g + 2] {
                    if w2 == w0 || w2 == w1 {
                        continue;
                    }
                    let c2 = win_bit_counts[w2][(b2 - b'0') as usize] * 10
                        + (win_bit_counts[w2][0] + win_bit_counts[w2][1]) * 3;
                    if c0 + c1 + c2 < best_cost {
                        best_cost = c0 + c1 + c2;
                        best_triplet = (w0, w1, w2);
                        best_bits = (b0, b1, b2);
                    }
                }
            }
        }
        // Fallback if w1 == w2 is needed
        if best_cost == i32::MAX {
            for &(w0, b0) in &allowed_wins[3 * g] {
                for &(w1, b1) in &allowed_wins[3 * g + 1] {
                    if w1 == w0 {
                        continue;
                    }
                    for &(w2, b2) in &allowed_wins[3 * g + 2] {
                        if w2 == w0 {
                            continue;
                        }
                        best_triplet = (w0, w1, w2);
                        best_bits = (b0, b1, b2);
                        break;
                    }
                }
            }
        }
        let (w0, w1, w2) = best_triplet;
        let (b0, b1, b2) = best_bits;
        c_handle_win[3 * g] = w0;
        c_handle_bit[3 * g] = b0;
        win_bit_counts[w0][(b0 - b'0') as usize] += 1;

        c_handle_win[3 * g + 1] = w1;
        c_handle_bit[3 * g + 1] = b1;
        win_bit_counts[w1][(b1 - b'0') as usize] += 1;

        c_handle_win[3 * g + 2] = w2;
        c_handle_bit[3 * g + 2] = b2;
        win_bit_counts[w2][(b2 - b'0') as usize] += 1;
    }

    // Precompute for each barcode p: its group g, branch r, whether it's j (true) or i (false),
    // its hole mask hm on data zone, and its list of data-valid bottom sheets B.
    let mut p_group = [0usize; P];
    let mut p_branch = [0usize; P];
    let mut p_is_j = [false; P];
    let mut p_hm = [0u64; P];
    let mut p_data_cands: Vec<Vec<usize>> = vec![Vec::new(); P];

    for g in 0..64 {
        let (j1, j2) = pairs[g];
        let d = d_masks[2 * g];
        for r in 0..2 {
            let ir = rem[2 * g + r];
            let jr = if r == 0 { j1 } else { j2 };
            let hm = (data_a[ir] ^ data_a[jr]) & d;

            for &(p, is_j) in &[(ir, false), (jr, true)] {
                p_group[p] = g;
                p_branch[p] = r;
                p_is_j[p] = is_j;
                p_hm[p] = hm;
                let tp = data_a[p] & hm;
                for b in 0..K {
                    if b / 3 == g {
                        continue;
                    }
                    if (valid_m[b] & hm) == hm && (val_m[b] & hm) == tp {
                        p_data_cands[p].push(b);
                    }
                }
            }
        }
    }

    // Optimize handle assignments so every barcode p has >= 2 fully valid bottom sheets B
    let count_valid_b = |p: usize, hw: &[usize; K], hb: &[u8; K]| -> usize {
        if p_hm[p] == 0 {
            return 100;
        }
        let g = p_group[p];
        let r = p_branch[p];
        let w_s = hw[3 * g + 1 + r];
        let w_u = if p_is_j[p] { hw[3 * g] } else { usize::MAX };
        let mut cnt = 0;
        for &b in &p_data_cands[p] {
            let wb = hw[b];
            if wb != w_s && wb != w_u && hb[b] == a_bytes[p][wb] {
                cnt += 1;
            }
        }
        cnt
    };

    for _ in 0..20 {
        let mut changed = false;
        for p in 0..P {
            if count_valid_b(p, &c_handle_win, &c_handle_bit) < 2 {
                // Try changing the handle window of one of p's data_cands
                for &b in &p_data_cands[p] {
                    let g_b = b / 3;
                    let old_w = c_handle_win[b];
                    let old_bit = c_handle_bit[b];
                    for &(nw, nbit) in &allowed_wins[b] {
                        if nw == old_w {
                            continue;
                        }
                        // Check group g_b constraint: U_{g_b} must differ from S_{g_b,0} and S_{g_b,1}
                        let w_u = if b % 3 == 0 { nw } else { c_handle_win[3 * g_b] };
                        let w_s0 = if b % 3 == 1 {
                            nw
                        } else {
                            c_handle_win[3 * g_b + 1]
                        };
                        let w_s1 = if b % 3 == 2 {
                            nw
                        } else {
                            c_handle_win[3 * g_b + 2]
                        };
                        if w_u == w_s0 || w_u == w_s1 {
                            continue;
                        }
                        let g_p = p_group[p];
                        let r_p = p_branch[p];
                        let w_sp = c_handle_win[3 * g_p + 1 + r_p];
                        let w_up = if p_is_j[p] {
                            c_handle_win[3 * g_p]
                        } else {
                            usize::MAX
                        };
                        if nw == w_sp || nw == w_up || nbit != a_bytes[p][nw] {
                            continue;
                        }
                        // Tentatively apply and check no other barcode drops below min(old_cnt, 2)
                        let mut ok = true;
                        for p2 in 0..P {
                            let old_c = count_valid_b(p2, &c_handle_win, &c_handle_bit);
                            c_handle_win[b] = nw;
                            c_handle_bit[b] = nbit;
                            let new_c = count_valid_b(p2, &c_handle_win, &c_handle_bit);
                            c_handle_win[b] = old_w;
                            c_handle_bit[b] = old_bit;
                            if new_c < old_c && new_c < 2 {
                                ok = false;
                                break;
                            }
                        }
                        if ok {
                            c_handle_win[b] = nw;
                            c_handle_bit[b] = nbit;
                            win_bit_counts[old_w][(old_bit - b'0') as usize] -= 1;
                            win_bit_counts[nw][(nbit - b'0') as usize] += 1;
                            changed = true;
                            break;
                        }
                    }
                    if count_valid_b(p, &c_handle_win, &c_handle_bit) >= 2 {
                        break;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    // Build conceptual sheets [u8; N]
    let mut c_sheets = [[b'.'; N]; K];
    for s in 0..K {
        let w_h = c_handle_win[s];
        c_sheets[s][w_h] = c_handle_bit[s];
        for dw in 0..(N - H) {
            if (valid_m[s] >> dw) & 1 == 1 {
                c_sheets[s][H + dw] = if (val_m[s] >> dw) & 1 == 1 {
                    b'1'
                } else {
                    b'0'
                };
            }
        }
    }

    // Step 4: Order initial stack 0..K so that each handle queue [0..H) alternates '0' and '1'
    let mut queues0: Vec<Vec<usize>> = vec![Vec::new(); H];
    let mut queues1: Vec<Vec<usize>> = vec![Vec::new(); H];
    for s in 0..K {
        let w = c_handle_win[s];
        if c_handle_bit[s] == b'0' {
            queues0[w].push(s);
        } else {
            queues1[w].push(s);
        }
    }

    let mut init_perm = Vec::with_capacity(K);
    let mut ptr0 = [0usize; H];
    let mut ptr1 = [0usize; H];
    let mut turn_bit = [0u8; H];
    while init_perm.len() < K {
        for w in 0..H {
            if turn_bit[w] == 0 {
                if ptr0[w] < queues0[w].len() {
                    init_perm.push(queues0[w][ptr0[w]]);
                    ptr0[w] += 1;
                } else if ptr1[w] < queues1[w].len() {
                    init_perm.push(queues1[w][ptr1[w]]);
                    ptr1[w] += 1;
                }
                turn_bit[w] = 1;
            } else {
                if ptr1[w] < queues1[w].len() {
                    init_perm.push(queues1[w][ptr1[w]]);
                    ptr1[w] += 1;
                } else if ptr0[w] < queues0[w].len() {
                    init_perm.push(queues0[w][ptr0[w]]);
                    ptr0[w] += 1;
                }
                turn_bit[w] = 0;
            }
        }
    }

    // Map conceptual sheet ID -> actual output sheet ID (0..K)
    let mut c_to_actual = [0usize; K];
    let mut sheets = [[b'.'; N]; K];
    let mut handle_win = [0usize; K];
    let mut handle_bit = [b'0'; K];

    for (actual_id, &c_id) in init_perm.iter().enumerate() {
        c_to_actual[c_id] = actual_id;
        sheets[actual_id] = c_sheets[c_id];
        handle_win[actual_id] = c_handle_win[c_id];
        handle_bit[actual_id] = c_handle_bit[c_id];
    }

    // Output Phase 1: K lines of length N
    for s in 0..K {
        let row_str = std::str::from_utf8(&sheets[s]).unwrap();
        writeln!(out, "{}", row_str).unwrap();
    }
    out.flush().unwrap();

    // Maintain live stack of actual sheet IDs (initially 0, 1, ..., K-1)
    let mut stack: Vec<usize> = (0..K).collect();

    // Phase 2: Reactive loop for R turns
    for _turn in 0..r_turns {
        let line = match lines.next() {
            Some(Ok(l)) => l,
            _ => break,
        };
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }
        let a_r: i32 = trimmed.parse().unwrap();
        if a_r < 0 {
            break;
        }
        let p = a_r as usize;
        let g = p_group[p];
        let r = p_branch[p];

        let mut ops: Vec<(usize, usize)> = Vec::new();

        // Foreground sheets for p
        let fg: Vec<usize> = if p_is_j[p] {
            vec![c_to_actual[3 * g], c_to_actual[3 * g + 1 + r]]
        } else {
            vec![c_to_actual[3 * g + 1 + r]]
        };
        let m0 = fg.len();

        // Stage 1: Pull foreground sheets to depths 0..m0
        for (idx, &fg_s) in fg.iter().enumerate() {
            pull_sheet_to_depth(fg_s, idx, &mut stack, &sheets, &handle_win, &handle_bit, &mut ops);
        }

        let hm = p_hm[p];
        // Helper closure to check if any hole in hm currently displays the wrong bit
        let check_hm_wrong = |stk: &[usize]| -> bool {
            if hm == 0 {
                return false;
            }
            for dw in 0..(N - H) {
                if (hm >> dw) & 1 == 1 {
                    let w = H + dw;
                    let vis_s = visible_sheet_at(stk, &sheets, w);
                    let cur_c = if vis_s == usize::MAX {
                        b'0'
                    } else {
                        sheets[vis_s][w]
                    };
                    if cur_c != a_bytes[p][w] {
                        return true;
                    }
                }
            }
            false
        };

        let pull_best_b =
            |stk: &mut Vec<usize>, ops_vec: &mut Vec<(usize, usize)>| -> bool {
                let mut best_b = usize::MAX;
                let mut best_net_cost = i32::MAX;

                for b in 0..K {
                    if fg.contains(&b) {
                        continue;
                    }
                    let wb = handle_win[b];
                    if fg.iter().any(|&fs| handle_win[fs] == wb) {
                        continue;
                    }
                    if handle_bit[b] != a_bytes[p][wb] {
                        continue;
                    }
                    let mut covers = true;
                    for dw in 0..(N - H) {
                        if (hm >> dw) & 1 == 1 {
                            let w = H + dw;
                            if sheets[b][w] != a_bytes[p][w] {
                                covers = false;
                                break;
                            }
                        }
                    }
                    if !covers {
                        continue;
                    }

                    let pull_cost = if stk[m0] == b {
                        0
                    } else if is_visible(stk, &sheets, b) {
                        1
                    } else {
                        let mut q_above = 0i32;
                        for &s in stk.iter() {
                            if s == b {
                                break;
                            }
                            if handle_win[s] == wb {
                                q_above += 1;
                            }
                        }
                        q_above + 1
                    };

                    let vis_wb = visible_sheet_at(stk, &sheets, wb);
                    let wb_wrong = vis_wb != usize::MAX && sheets[vis_wb][wb] != a_bytes[p][wb];
                    let net_cost = pull_cost - if wb_wrong { 1 } else { 0 };

                    if net_cost < best_net_cost {
                        best_net_cost = net_cost;
                        best_b = b;
                    }
                }

                if best_b != usize::MAX {
                    pull_sheet_to_depth(
                        best_b,
                        m0,
                        stk,
                        &sheets,
                        &handle_win,
                        &handle_bit,
                        ops_vec,
                    );
                    true
                } else {
                    false
                }
            };

        let mut b_pulled = false;
        if check_hm_wrong(&stack) {
            b_pulled = pull_best_b(&mut stack, &mut ops);
        }

        // Stage 3: Fix any handle windows w in 0..H that display wrong bit
        for w in 0..H {
            let vis_s = visible_sheet_at(&stack, &sheets, w);
            let cur_c = if vis_s == usize::MAX {
                b'0'
            } else {
                sheets[vis_s][w]
            };
            if cur_c != a_bytes[p][w] {
                let mut target_q = None;
                for &s in &stack {
                    if handle_win[s] == w && handle_bit[s] == a_bytes[p][w] {
                        target_q = Some(s);
                        break;
                    }
                }
                if let Some(tq) = target_q {
                    loop {
                        let cur_top = visible_sheet_at(&stack, &sheets, w);
                        if cur_top == tq || sheets[cur_top][w] == a_bytes[p][w] {
                            break;
                        }
                        let tq_pos = stack.iter().position(|&x| x == tq).unwrap();
                        apply_op(&mut stack, &sheets, cur_top, tq_pos, &mut ops);
                    }
                }
            }
        }

        // If we skipped pulling B before Stage 3, check if Stage 3 disturbed hm
        if !b_pulled && check_hm_wrong(&stack) {
            pull_best_b(&mut stack, &mut ops);
        }

        // Output operations for this turn
        writeln!(out, "{}", ops.len()).unwrap();
        for &(k, d) in &ops {
            writeln!(out, "{} {}", k, d).unwrap();
        }
        out.flush().unwrap();
    }
}
