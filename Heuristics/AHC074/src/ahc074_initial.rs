use std::io::{self, BufRead, BufWriter, Write};

const N: usize = 64;
const K: usize = 192;
const P: usize = 256;
const HU: usize = 7;
const HS: usize = 18;
const HS_TOP: usize = 10;
const H: usize = HU + HS;

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
    debug_assert!(is_visible(stack, sheets, k), "Sheet {} not visible!", k);
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
    layer_max_d: usize,
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
            apply_op(stack, sheets, top_q, target_pos, ops);
        } else {
            let mut opp_pos = None;
            for (idx, &s) in stack.iter().enumerate().skip(top_pos + 1) {
                if idx > layer_max_d {
                    break;
                }
                if handle_win[s] == w && handle_bit[s] != t_bit {
                    opp_pos = Some(idx);
                    break;
                }
            }
            let d = match opp_pos {
                Some(op) => target_pos.max(op),
                None => layer_max_d,
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
    let mut hu_a = [0u64; P];
    let mut hs_a = [0u64; P];
    let mut data_a = [0u64; P];

    for p in 0..P {
        let line = lines.next().unwrap().unwrap();
        let bytes = line.trim().as_bytes();
        let mut um = 0u64;
        let mut sm = 0u64;
        let mut dm = 0u64;
        for w in 0..N {
            a_bytes[p][w] = bytes[w];
            if bytes[w] == b'1' {
                if w < HU {
                    um |= 1u64 << w;
                } else if w < H {
                    sm |= 1u64 << (w - HU);
                } else {
                    dm |= 1u64 << (w - H);
                }
            }
        }
        hu_a[p] = um;
        hs_a[p] = sm;
        data_a[p] = dm;
    }

    // Step 1: Greedily pair 64 closest pairs (j1, j2) on data zone [H..N) that agree on >= 2 windows in [0..HU)
    let mut cand_pairs = Vec::with_capacity(P * (P - 1) / 2);
    for u in 0..P {
        for v in (u + 1)..P {
            let agree_hu = HU as u32 - (hu_a[u] ^ hu_a[v]).count_ones();
            if agree_hu >= 2 {
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
        let agree_hs = HS as u32 - (hs_a[i_idx] ^ hs_a[j]).count_ones();
        if agree_hs < 2 {
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
    let compute_s_masks = |match_i: &[usize]| -> ([u64; 128], [u64; 128]) {
        let mut valid_m = [0u64; 128];
        let mut val_m = [0u64; 128];
        for slot in 0..128 {
            let ir = match_i[slot];
            let jr = j_list[slot];
            let d = d_masks[slot];
            let hm = (data_a[ir] ^ data_a[jr]) & d;
            valid_m[slot] = full_data_mask ^ hm;
            val_m[slot] = data_a[ir] & (full_data_mask ^ hm);
        }
        (valid_m, val_m)
    };

    let exact_slot_cost =
        |i_idx: usize, slot: usize, valid_m: &[u64; 128], val_m: &[u64; 128]| -> i64 {
            let j = j_list[slot];
            let d = d_masks[slot];
            let agree_hs = HS as u32 - (hs_a[i_idx] ^ hs_a[j]).count_ones();
            if agree_hs < 2 {
                return 10_000_000;
            }
            let hm = (data_a[i_idx] ^ data_a[j]) & d;
            let h = hm.count_ones() as i64;
            let ti = data_a[i_idx] & hm;
            let tj = data_a[j] & hm;
            let mut ci = 0i64;
            let mut cj = 0i64;
            for b in 0..128 {
                if b == slot {
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
        let (valid_m, val_m) = compute_s_masks(&rem);
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

    let (s_valid_m, s_val_m) = compute_s_masks(&rem);

    // Step 3a: Assign U-handles in [0..HU) for the 64 U-sheets (conceptual 128..192)
    let mut c_handle_win = [0usize; K];
    let mut c_handle_bit = [b'0'; K];
    let mut u_counts = [[0i32; 2]; HU];

    for g in 0..64 {
        let (j1, j2) = pairs[g];
        let mut best_w = 0;
        let mut best_b = b'0';
        let mut best_c = i32::MAX;
        for w in 0..HU {
            if a_bytes[j1][w] == a_bytes[j2][w] {
                let b = a_bytes[j1][w];
                let mut extra_cov = 0i32;
                for w2 in 0..w {
                    if a_bytes[j1][w2] == a_bytes[j2][w2] {
                        extra_cov += 1;
                    }
                }
                let c = u_counts[w][(b - b'0') as usize] * 12
                    + (u_counts[w][0] + u_counts[w][1]) * 4
                    - extra_cov * 5;
                if c < best_c {
                    best_c = c;
                    best_w = w;
                    best_b = b;
                }
            }
        }
        c_handle_win[128 + g] = best_w;
        c_handle_bit[128 + g] = best_b;
        u_counts[best_w][(best_b - b'0') as usize] += 1;
    }
    // 2-opt swap on U-handles preserving exact (window, bit) counts
    let u_cov_score = |g: usize, w: usize| -> i32 {
        let (j1, j2) = pairs[g];
        let mut sc = 0i32;
        for w2 in 0..w {
            if a_bytes[j1][w2] == a_bytes[j2][w2] {
                sc += 1;
            }
        }
        sc
    };
    for _ in 0..20 {
        let mut imp = false;
        for g1 in 0..64 {
            for g2 in (g1 + 1)..64 {
                let w1 = c_handle_win[128 + g1];
                let b1 = c_handle_bit[128 + g1];
                let w2 = c_handle_win[128 + g2];
                let b2 = c_handle_bit[128 + g2];
                if w1 == w2 {
                    continue;
                }
                let (j1_1, j2_1) = pairs[g1];
                let (j1_2, j2_2) = pairs[g2];
                if a_bytes[j1_1][w2] == a_bytes[j2_1][w2]
                    && a_bytes[j1_1][w2] == b2
                    && a_bytes[j1_2][w1] == a_bytes[j2_2][w1]
                    && a_bytes[j1_2][w1] == b1
                {
                    let cur = u_cov_score(g1, w1) + u_cov_score(g2, w2);
                    let nxt = u_cov_score(g1, w2) + u_cov_score(g2, w1);
                    if nxt > cur {
                        c_handle_win[128 + g1] = w2;
                        c_handle_bit[128 + g1] = b2;
                        c_handle_win[128 + g2] = w1;
                        c_handle_bit[128 + g2] = b1;
                        imp = true;
                    }
                }
            }
        }
        if !imp {
            break;
        }
    }

    // Step 3b: Assign S-handles in [HU..H) for the 128 S-sheets (conceptual 0..128)
    // Prefer putting sheets that are valid bottom sheets for many barcodes into S_top [HU .. HU + HS_TOP)!
    let mut b_deg = [0i32; 128];
    for slot in 0..128 {
        let ir = rem[slot];
        let jr = j_list[slot];
        let hm = (data_a[ir] ^ data_a[jr]) & d_masks[slot];
        if hm == 0 {
            continue;
        }
        for &(p, _) in &[(ir, false), (jr, true)] {
            let tp = data_a[p] & hm;
            for b in 0..128 {
                if b != slot && (s_valid_m[b] & hm) == hm && (s_val_m[b] & hm) == tp {
                    b_deg[b] += 1;
                }
            }
        }
    }

    let mut allowed_s_wins: Vec<Vec<(usize, u8, bool)>> = vec![Vec::new(); 128];
    let mut s_counts = [[0i32; 2]; HS];
    // Sort slots by b_deg descending so high-degree bottom sheets get first pick of S_top windows
    let mut slot_order: Vec<usize> = (0..128).collect();
    slot_order.sort_unstable_by_key(|&s| b_deg[s]);

    for &slot in &slot_order {
        let g = slot / 2;
        let (j1, j2) = pairs[g];
        let ir = rem[slot];
        let jr = j_list[slot];
        for w in HU..H {
            if a_bytes[ir][w] == a_bytes[jr][w] {
                let u_hole = a_bytes[j1][w] != a_bytes[j2][w];
                allowed_s_wins[slot].push((w, a_bytes[ir][w], u_hole));
            }
        }
        let mut best_w = HU;
        let mut best_b = b'0';
        let mut best_c = i32::MAX;
        for &(w, b, u_hole) in &allowed_s_wins[slot] {
            let idx = w - HU;
            let is_top = idx < HS_TOP;
            let mut extra_cov = 0i32;
            if !is_top {
                for w2 in HU..w {
                    if a_bytes[ir][w2] == a_bytes[jr][w2] {
                        extra_cov += 1;
                    }
                }
            }
            let c = s_counts[idx][(b - b'0') as usize] * 12
                + (s_counts[idx][0] + s_counts[idx][1]) * 4
                - if u_hole { 4 } else { 0 }
                - extra_cov * 3;
            if c < best_c {
                best_c = c;
                best_w = w;
                best_b = b;
            }
        }
        c_handle_win[slot] = best_w;
        c_handle_bit[slot] = best_b;
        s_counts[best_w - HU][(best_b - b'0') as usize] += 1;
    }

    let mut p_group = [0usize; P];
    let mut p_branch = [0usize; P];
    let mut p_is_j = [false; P];
    let mut p_hm = [0u64; P];
    let mut p_data_cands: Vec<Vec<usize>> = vec![Vec::new(); P];

    for g in 0..64 {
        let (j1, j2) = pairs[g];
        let d = d_masks[2 * g];
        for r in 0..2 {
            let slot = 2 * g + r;
            let ir = rem[slot];
            let jr = if r == 0 { j1 } else { j2 };
            let hm = (data_a[ir] ^ data_a[jr]) & d;

            for &(p, is_j) in &[(ir, false), (jr, true)] {
                p_group[p] = g;
                p_branch[p] = r;
                p_is_j[p] = is_j;
                p_hm[p] = hm;
                let tp = data_a[p] & hm;
                for b in 0..128 {
                    if b == slot {
                        continue;
                    }
                    if (s_valid_m[b] & hm) == hm && (s_val_m[b] & hm) == tp {
                        p_data_cands[p].push(b);
                    }
                }
            }
        }
    }

    // Only S_top sheets (hw[b] < HU + HS_TOP) can act as zero-return-cost bottom sheets B!
    let count_valid_b = |p: usize, hw: &[usize; K], hb: &[u8; K]| -> usize {
        if p_hm[p] == 0 {
            return 100;
        }
        let g = p_group[p];
        let r = p_branch[p];
        let (j1, j2) = pairs[g];
        let slot_p = 2 * g + r;
        let ir = rem[slot_p];
        let jr = j_list[slot_p];
        let w_s = hw[slot_p];
        let mut cnt = 0;
        for &b in &p_data_cands[p] {
            let wb = hw[b];
            if wb >= HU + HS_TOP || wb == w_s {
                continue;
            }
            let u_covers_wb = p_is_j[p] && (a_bytes[j1][wb] == a_bytes[j2][wb]);
            let s_covers_wb = (w_s >= HU + HS_TOP) && (a_bytes[ir][wb] == a_bytes[jr][wb]);
            if u_covers_wb || s_covers_wb || hb[b] == a_bytes[p][wb] {
                cnt += 1;
            }
        }
        cnt
    };

    for _ in 0..30 {
        let mut changed = false;
        for p in 0..P {
            if count_valid_b(p, &c_handle_win, &c_handle_bit) < 2 {
                for &b in &p_data_cands[p] {
                    let old_w = c_handle_win[b];
                    let old_bit = c_handle_bit[b];
                    for &(nw, nbit, _) in &allowed_s_wins[b] {
                        if nw >= HU + HS_TOP || nw == old_w {
                            continue;
                        }
                        let g_p = p_group[p];
                        let r_p = p_branch[p];
                        let slot_p = 2 * g_p + r_p;
                        if nw == c_handle_win[slot_p] {
                            continue;
                        }
                        let (j1, j2) = pairs[g_p];
                        let ir = rem[slot_p];
                        let jr = j_list[slot_p];
                        let u_covers = p_is_j[p] && (a_bytes[j1][nw] == a_bytes[j2][nw]);
                        let s_covers = (c_handle_win[slot_p] >= HU + HS_TOP) && (a_bytes[ir][nw] == a_bytes[jr][nw]);
                        if !u_covers && !s_covers && nbit != a_bytes[p][nw] {
                            continue;
                        }
                        let mut ok = true;
                        for p2 in 0..P {
                            let old_c = count_valid_b(p2, &c_handle_win, &c_handle_bit);
                            c_handle_win[b] = nw;
                            c_handle_bit[b] = nbit;
                            let new_c = count_valid_b(p2, &c_handle_win, &c_handle_bit);
                            c_handle_win[b] = old_w;
                            c_handle_bit[b] = old_bit;
                            if new_c < old_c && new_c < 1 {
                                ok = false;
                                break;
                            }
                        }
                        if ok {
                            c_handle_win[b] = nw;
                            c_handle_bit[b] = nbit;
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
    // 0..128: S-sheets
    for slot in 0..128 {
        let w_s = c_handle_win[slot];
        c_sheets[slot][w_s] = c_handle_bit[slot];
        // If this S-sheet is in S_bot (w_s >= HU + HS_TOP), fill ALL agreed windows in [HU .. w_s)!
        if w_s >= HU + HS_TOP {
            let ir = rem[slot];
            let jr = j_list[slot];
            for w in HU..w_s {
                if a_bytes[ir][w] == a_bytes[jr][w] {
                    c_sheets[slot][w] = a_bytes[ir][w];
                }
            }
        }
        for dw in 0..(N - H) {
            if (s_valid_m[slot] >> dw) & 1 == 1 {
                c_sheets[slot][H + dw] = if (s_val_m[slot] >> dw) & 1 == 1 {
                    b'1'
                } else {
                    b'0'
                };
            }
        }
    }
    // 128..192: U-sheets, where each U-queue w_u in 0..HU is its OWN 1-window layer!
    for g in 0..64 {
        let u_id = 128 + g;
        let (j1, j2) = pairs[g];
        let w_u = c_handle_win[u_id];
        c_sheets[u_id][w_u] = c_handle_bit[u_id];

        for w in 0..w_u {
            if a_bytes[j1][w] == a_bytes[j2][w] {
                c_sheets[u_id][w] = a_bytes[j1][w];
            }
        }
        for w in HU..H {
            if a_bytes[j1][w] == a_bytes[j2][w] {
                c_sheets[u_id][w] = a_bytes[j1][w];
            }
        }
        let d = d_masks[2 * g];
        let u_valid = full_data_mask ^ d;
        let u_val = data_a[j1] & u_valid;
        for dw in 0..(N - H) {
            if (u_valid >> dw) & 1 == 1 {
                c_sheets[u_id][H + dw] = if (u_val >> dw) & 1 == 1 { b'1' } else { b'0' };
            }
        }
    }

    // Step 4: Order initial stack:
    // 1. S_top: all S-sheets with w_s in [HU .. HU + HS_TOP), interleaved
    // 2. S_bot: for each w in [HU + HS_TOP .. H), contiguous 1-window slice
    // 3. U: for each w in [0 .. HU), contiguous 1-window slice
    let mut init_perm = Vec::with_capacity(K);
    let k_stop: usize;
    {
        let mut q0: Vec<Vec<usize>> = vec![Vec::new(); HS_TOP];
        let mut q1: Vec<Vec<usize>> = vec![Vec::new(); HS_TOP];
        let mut stop_total = 0;
        for slot in 0..128 {
            let w = c_handle_win[slot];
            if w < HU + HS_TOP {
                stop_total += 1;
                if c_handle_bit[slot] == b'0' {
                    q0[w - HU].push(slot);
                } else {
                    q1[w - HU].push(slot);
                }
            }
        }
        k_stop = stop_total;
        let mut p0 = [0usize; HS_TOP];
        let mut p1 = [0usize; HS_TOP];
        let mut tb = [0u8; HS_TOP];
        while init_perm.len() < k_stop {
            for w in 0..HS_TOP {
                if tb[w] == 0 {
                    if p0[w] < q0[w].len() {
                        init_perm.push(q0[w][p0[w]]);
                        p0[w] += 1;
                    } else if p1[w] < q1[w].len() {
                        init_perm.push(q1[w][p1[w]]);
                        p1[w] += 1;
                    }
                    tb[w] = 1;
                } else {
                    if p1[w] < q1[w].len() {
                        init_perm.push(q1[w][p1[w]]);
                        p1[w] += 1;
                    } else if p0[w] < q0[w].len() {
                        init_perm.push(q0[w][p0[w]]);
                        p0[w] += 1;
                    }
                    tb[w] = 0;
                }
            }
        }
    }

    let mut win_stack_ranges = [(0usize, 0usize); H];
    for w in HU..(HU + HS_TOP) {
        win_stack_ranges[w] = (0, k_stop);
    }
    for w in (HU + HS_TOP)..H {
        let start_idx = init_perm.len();
        let mut q0 = Vec::new();
        let mut q1 = Vec::new();
        for slot in 0..128 {
            if c_handle_win[slot] == w {
                if c_handle_bit[slot] == b'0' {
                    q0.push(slot);
                } else {
                    q1.push(slot);
                }
            }
        }
        let (mut p0, mut p1, mut tb) = (0, 0, 0);
        while p0 < q0.len() || p1 < q1.len() {
            if tb == 0 {
                if p0 < q0.len() {
                    init_perm.push(q0[p0]);
                    p0 += 1;
                } else {
                    init_perm.push(q1[p1]);
                    p1 += 1;
                }
                tb = 1;
            } else {
                if p1 < q1.len() {
                    init_perm.push(q1[p1]);
                    p1 += 1;
                } else {
                    init_perm.push(q0[p0]);
                    p0 += 1;
                }
                tb = 0;
            }
        }
        win_stack_ranges[w] = (start_idx, init_perm.len());
    }

    for w in 0..HU {
        let start_idx = init_perm.len();
        let mut q0 = Vec::new();
        let mut q1 = Vec::new();
        for g in 0..64 {
            let u_id = 128 + g;
            if c_handle_win[u_id] == w {
                if c_handle_bit[u_id] == b'0' {
                    q0.push(u_id);
                } else {
                    q1.push(u_id);
                }
            }
        }
        let (mut p0, mut p1, mut tb) = (0, 0, 0);
        while p0 < q0.len() || p1 < q1.len() {
            if tb == 0 {
                if p0 < q0.len() {
                    init_perm.push(q0[p0]);
                    p0 += 1;
                } else {
                    init_perm.push(q1[p1]);
                    p1 += 1;
                }
                tb = 1;
            } else {
                if p1 < q1.len() {
                    init_perm.push(q1[p1]);
                    p1 += 1;
                } else {
                    init_perm.push(q0[p0]);
                    p0 += 1;
                }
                tb = 0;
            }
        }
        win_stack_ranges[w] = (start_idx, init_perm.len());
    }

    let mut c_to_actual = [0usize; K];
    let mut sheets = [[b'.'; N]; K];
    let mut handle_win = [0usize; K];
    let mut handle_bit = [b'0'; K];
    let mut is_u_sheet = [false; K];
    let mut is_sbot_sheet = [false; K];

    for (actual_id, &c_id) in init_perm.iter().enumerate() {
        c_to_actual[c_id] = actual_id;
        sheets[actual_id] = c_sheets[c_id];
        let hw = c_handle_win[c_id];
        handle_win[actual_id] = hw;
        handle_bit[actual_id] = c_handle_bit[c_id];
        is_u_sheet[actual_id] = c_id >= 128;
        is_sbot_sheet[actual_id] = c_id < 128 && hw >= HU + HS_TOP;
    }

    for s in 0..K {
        let row_str = std::str::from_utf8(&sheets[s]).unwrap();
        writeln!(out, "{}", row_str).unwrap();
    }
    out.flush().unwrap();

    let mut stack: Vec<usize> = (0..K).collect();

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
        let hm = p_hm[p];
        let is_j = p_is_j[p];

        let s_fg = c_to_actual[2 * g + r];
        let u_fg = if is_j { c_to_actual[128 + g] } else { usize::MAX };

        let mut ops: Vec<(usize, usize)> = Vec::new();

        // Step 0a: If stack[0] is a U-sheet from the previous turn, return it to its 1-window U-sublayer!
        if is_u_sheet[stack[0]] {
            let u_prev = stack[0];
            let wu = handle_win[u_prev];
            let (ul_start, ul_end) = win_stack_ranges[wu];
            if u_prev == u_fg || handle_bit[u_prev] == a_bytes[p][wu] {
                apply_op(&mut stack, &sheets, u_prev, ul_start, &mut ops);
            } else {
                let mut tq_pos = ul_start;
                for (idx, &s) in stack.iter().enumerate().take(ul_end).skip(ul_start + 1) {
                    if handle_win[s] == wu && handle_bit[s] == a_bytes[p][wu] {
                        tq_pos = idx;
                        break;
                    }
                }
                apply_op(&mut stack, &sheets, u_prev, tq_pos, &mut ops);
            }
        }

        // Step 0b: Now if stack[0] is an S_bot sheet from the previous turn, return it to its 1-window S_bot sublayer!
        if is_sbot_sheet[stack[0]] {
            let s_prev = stack[0];
            let ws = handle_win[s_prev];
            let (sl_start, sl_end) = win_stack_ranges[ws];
            if s_prev == s_fg || (is_j && sheets[u_fg][ws] != b'.') || handle_bit[s_prev] == a_bytes[p][ws] {
                apply_op(&mut stack, &sheets, s_prev, sl_start, &mut ops);
            } else {
                let mut tq_pos = sl_start;
                for (idx, &s) in stack.iter().enumerate().take(sl_end).skip(sl_start + 1) {
                    if handle_win[s] == ws && handle_bit[s] == a_bytes[p][ws] {
                        tq_pos = idx;
                        break;
                    }
                }
                apply_op(&mut stack, &sheets, s_prev, tq_pos, &mut ops);
            }
        }

        // Verify resting layer invariant!
        debug_assert!(stack[0..k_stop].iter().all(|&s| !is_u_sheet[s] && !is_sbot_sheet[s]));

        let s_is_bot = is_sbot_sheet[s_fg];
        let b_target_d = if s_is_bot { 0 } else { 1 };

        if !s_is_bot {
            // Pull s_fg (in S_top) to depth 0 now
            pull_sheet_to_depth(
                s_fg,
                0,
                k_stop - 1,
                &mut stack,
                &sheets,
                &handle_win,
                &handle_bit,
                &mut ops,
            );
        }

        // Helper: check if hm will be wrong after s_fg (and u_fg) are placed above stack[0..k_stop]
        let check_hm_wrong = |stk: &[usize]| -> bool {
            if hm == 0 {
                return false;
            }
            let start_scan = if s_is_bot { 0 } else { 1 };
            for dw in 0..(N - H) {
                if (hm >> dw) & 1 == 1 {
                    let w = H + dw;
                    let mut cur_c = b'0';
                    for &s in stk[start_scan..].iter() {
                        if sheets[s][w] != b'.' {
                            cur_c = sheets[s][w];
                            break;
                        }
                    }
                    if cur_c != a_bytes[p][w] {
                        return true;
                    }
                }
            }
            false
        };

        let pull_best_b = |stk: &mut Vec<usize>, ops_vec: &mut Vec<(usize, usize)>| -> bool {
            let mut best_b = usize::MAX;
            let mut best_net_cost = i32::MAX;

            for b in 0..K {
                if is_u_sheet[b] || is_sbot_sheet[b] || b == s_fg {
                    continue;
                }
                let wb = handle_win[b];
                if wb == handle_win[s_fg] {
                    continue;
                }
                let u_covers_wb = is_j && sheets[u_fg][wb] != b'.';
                let s_covers_wb = s_is_bot && sheets[s_fg][wb] != b'.';
                if !u_covers_wb && !s_covers_wb && handle_bit[b] != a_bytes[p][wb] {
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

                let pull_cost = if stk[b_target_d] == b {
                    0
                } else if is_visible(stk, &sheets, b) {
                    1
                } else {
                    let mut q_above = 0i32;
                    for &s in stk[0..k_stop].iter() {
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
                let wb_wrong = !u_covers_wb
                    && !s_covers_wb
                    && vis_wb != usize::MAX
                    && sheets[vis_wb][wb] != a_bytes[p][wb];
                let net_cost = pull_cost - if wb_wrong { 1 } else { 0 };

                if net_cost < best_net_cost {
                    best_net_cost = net_cost;
                    best_b = b;
                }
            }

            if best_b != usize::MAX {
                pull_sheet_to_depth(
                    best_b,
                    b_target_d,
                    k_stop - 1,
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

        let run_s3 = |stk: &mut Vec<usize>, ops_vec: &mut Vec<(usize, usize)>| {
            // 1. S_top windows [HU .. HU + HS_TOP)
            for w in HU..(HU + HS_TOP) {
                if (is_j && sheets[u_fg][w] != b'.') || (s_is_bot && sheets[s_fg][w] != b'.') {
                    continue;
                }
                let vis_s = visible_sheet_at(stk, &sheets, w);
                let cur_c = if vis_s == usize::MAX { b'0' } else { sheets[vis_s][w] };
                if cur_c != a_bytes[p][w] {
                    let mut target_q = None;
                    for &s in stk[0..k_stop].iter() {
                        if handle_win[s] == w && handle_bit[s] == a_bytes[p][w] {
                            target_q = Some(s);
                            break;
                        }
                    }
                    if let Some(tq) = target_q {
                        loop {
                            let cur_top = visible_sheet_at(stk, &sheets, w);
                            if cur_top == tq || sheets[cur_top][w] == a_bytes[p][w] {
                                break;
                            }
                            let tq_pos = stk.iter().position(|&x| x == tq).unwrap();
                            apply_op(stk, &sheets, cur_top, tq_pos, ops_vec);
                        }
                    }
                }
            }
            // 2. S_bot windows [HU + HS_TOP .. H) and U windows [0 .. HU)
            for &w in (HU + HS_TOP..H).chain(0..HU).collect::<Vec<_>>().iter() {
                if (is_j && sheets[u_fg][w] != b'.') || (s_is_bot && sheets[s_fg][w] != b'.') {
                    continue;
                }
                let (l_start, l_end) = win_stack_ranges[w];
                let vis_s = visible_sheet_at(stk, &sheets, w);
                let cur_c = if vis_s == usize::MAX { b'0' } else { sheets[vis_s][w] };
                if cur_c != a_bytes[p][w] {
                    let mut target_q = None;
                    for &s in stk[l_start..l_end].iter() {
                        if handle_win[s] == w && handle_bit[s] == a_bytes[p][w] {
                            target_q = Some(s);
                            break;
                        }
                    }
                    if let Some(tq) = target_q {
                        loop {
                            let cur_top = visible_sheet_at(stk, &sheets, w);
                            if cur_top == tq || sheets[cur_top][w] == a_bytes[p][w] {
                                break;
                            }
                            let tq_pos = stk.iter().position(|&x| x == tq).unwrap();
                            apply_op(stk, &sheets, cur_top, tq_pos, ops_vec);
                        }
                    }
                }
            }
        };

        let s_bot_max_d = if s_is_bot {
            win_stack_ranges[handle_win[s_fg]].1 - 1
        } else {
            k_stop - 1
        };
        let u_max_d = if is_j {
            win_stack_ranges[handle_win[u_fg]].1 - 1
        } else {
            191
        };

        let mut stack_a = stack.clone();
        let mut ops_a = ops.clone();
        {
            let mut b_pulled = false;
            if check_hm_wrong(&stack_a) {
                b_pulled = pull_best_b(&mut stack_a, &mut ops_a);
            }
            run_s3(&mut stack_a, &mut ops_a);
            if !b_pulled && check_hm_wrong(&stack_a) {
                pull_best_b(&mut stack_a, &mut ops_a);
            }
            if s_is_bot {
                pull_sheet_to_depth(
                    s_fg,
                    0,
                    s_bot_max_d,
                    &mut stack_a,
                    &sheets,
                    &handle_win,
                    &handle_bit,
                    &mut ops_a,
                );
            }
            if is_j {
                pull_sheet_to_depth(
                    u_fg,
                    0,
                    u_max_d,
                    &mut stack_a,
                    &sheets,
                    &handle_win,
                    &handle_bit,
                    &mut ops_a,
                );
            }
        }

        let mut stack_b = stack.clone();
        let mut ops_b = ops.clone();
        {
            run_s3(&mut stack_b, &mut ops_b);
            if check_hm_wrong(&stack_b) {
                pull_best_b(&mut stack_b, &mut ops_b);
            }
            if s_is_bot {
                pull_sheet_to_depth(
                    s_fg,
                    0,
                    s_bot_max_d,
                    &mut stack_b,
                    &sheets,
                    &handle_win,
                    &handle_bit,
                    &mut ops_b,
                );
            }
            if is_j {
                pull_sheet_to_depth(
                    u_fg,
                    0,
                    u_max_d,
                    &mut stack_b,
                    &sheets,
                    &handle_win,
                    &handle_bit,
                    &mut ops_b,
                );
            }
        }

        if ops_b.len() < ops_a.len() {
            stack = stack_b;
            ops = ops_b;
        } else {
            stack = stack_a;
            ops = ops_a;
        }

        writeln!(out, "{}", ops.len()).unwrap();
        for &(k, d) in &ops {
            writeln!(out, "{} {}", k, d).unwrap();
        }
        out.flush().unwrap();
    }
}
