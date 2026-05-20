use proconio::{input, marker::Chars};

// Bitwise packing to safely cram states safely into ultra-fast 32-bit registers avoiding 16MB cache misses
fn pack(r: usize, c: usize, d: usize) -> u32 {
    ((r as u32) << 13) | ((c as u32) << 3) | (d as u32)
}

fn unpack(val: u32) -> (usize, usize, usize) {
    let r = (val >> 13) as usize;
    let c = ((val >> 3) & 1023) as usize;
    let d = (val & 7) as usize;
    (r, c, d)
}

fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],
    }

    // Locate our dynamically starting coordinate internally
    let mut s_r = h;
    let mut s_c = w;
    for r in 0..h {
        for c in 0..w {
            if grid[r][c] == 'S' {
                s_r = r;
                s_c = c;
            }
        }
    }

    // A flat 1D array modeling a 3D Tensor dynamically natively [R][C][Last_Dir]
    let mut parent = vec![u32::MAX; h * w * 4];
    let mut q = Vec::with_capacity(h * w * 4);

    let dr = [!0usize, 1, 0, 0];
    let dc = [0usize, 0, !0usize, 1];
    let dir_letters = ['U', 'D', 'L', 'R'];

    // Map initial Start moves dynamically (Start cell lacks "previous" constraints logically)
    for nd in 0..4 {
        let nr = s_r.wrapping_add(dr[nd]);
        let nc = s_c.wrapping_add(dc[nd]);
        if nr < h && nc < w && grid[nr][nc] != '#' {
            let idx = (nr * w + nc) * 4 + nd;
            parent[idx] = pack(s_r, s_c, 4);

            // Reached Goal upon Step 1 mapping flawlessly natively!
            if grid[nr][nc] == 'G' {
                println!("Yes\n{}", dir_letters[nd]);
                return;
            }
            q.push(pack(nr, nc, nd));
        }
    }

    // Process BFS
    let mut head = 0;
    while head < q.len() {
        let (r, c, d) = unpack(q[head]);
        head += 1;

        let cell = grid[r][c];
        let mut allowed_dirs = [false; 4];

        // Logical Rules explicitly mapped out functionally natively
        if cell == '.' || cell == 'S' || cell == 'G' {
            allowed_dirs = [true; 4];
        } else if cell == 'o' {
            allowed_dirs[d] = true;
        } else if cell == 'x' {
            for i in 0..4 {
                if i != d {
                    allowed_dirs[i] = true;
                }
            }
        }

        // Branch off valid next moves directly dynamically mapped
        for nd in 0..4 {
            if !allowed_dirs[nd] {
                continue;
            }

            let nr = r.wrapping_add(dr[nd]);
            let nc = c.wrapping_add(dc[nd]);

            if nr < h && nc < w && grid[nr][nc] != '#' {
                let idx = (nr * w + nc) * 4 + nd;

                // If completely unvisited via this specific vector mapped route natively:
                if parent[idx] == u32::MAX {
                    parent[idx] = pack(r, c, d); // Breadcrumb tracking

                    if grid[nr][nc] == 'G' {
                        // Tracing paths logically reconstructing limits linearly mapped
                        let mut path = Vec::new();
                        let mut curr_r = nr;
                        let mut curr_c = nc;
                        let mut curr_d = nd;

                        while curr_d != 4 {
                            path.push(dir_letters[curr_d]);
                            let p_val = parent[(curr_r * w + curr_c) * 4 + curr_d];
                            let (pr, pc, pd) = unpack(p_val);
                            curr_r = pr;
                            curr_c = pc;
                            curr_d = pd;
                        }

                        path.reverse();
                        let path_str: String = path.into_iter().collect();
                        println!("Yes\n{}", path_str);
                        return;
                    }

                    q.push(pack(nr, nc, nd));
                }
            }
        }
    }

    println!("No");
}
