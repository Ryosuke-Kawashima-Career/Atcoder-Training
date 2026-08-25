/* This algorithm is for solving least common ancestor finding
 */

struct BlossomLCA {
    n: usize,
    parent: Vec<usize>,
    base: Vec<usize>,
    match_with: Vec<Option<usize>>,
}
impl BlossomLCA {
    fn new(n: usize) -> Self {
        Self {
            n: n,
            parent: vec![usize::MAX; n],
            base: (0..n).collect(),
            match_with: vec![None; n],
        }
    }
    fn find_lca(
        &mut self,
        mut u: usize,
        mut v: usize,
        visited_token: usize,
        used: &mut Vec<usize>,
    ) -> usize {
        loop {
            u = self.base[u];
            used[u] = visited_token;
            if let Some(m) = self.match_with[u] {
                if self.parent[m] != usize::MAX {
                    u = self.parent[m];
                }
            }
            v = self.base[v];
            used[v] = visited_token;
            if let Some(m) = self.match_with[v] {
                if self.parent[m] != usize::MAX {
                    v = self.parent[m];
                }
            }

            if used[u] == visited_token && used[v] == visited_token {
                return u;
            }
            if self.match_with[u].is_none() && self.match_with[v].is_none() {
                if used[u] == visited_token {
                    return u;
                }
                if used[v] == visited_token {
                    return v;
                }
            }
        }
    }
}
