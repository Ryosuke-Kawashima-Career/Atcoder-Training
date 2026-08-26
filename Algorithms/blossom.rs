/* This algorithm is for solving least common ancestor finding
It is used for the maximum matching problem of general graphs
 */
use std::collections::VecDeque;
fn main() {
    let mut eb = EdmondsMatching::new(5);
    eb.add_edge(0, 1);
    eb.add_edge(1, 2);
    eb.add_edge(2, 3);
    eb.add_edge(3, 4);
    eb.add_edge(4, 0);
    println!("{}", eb.max_matching());
}

struct BlossomLCA {
    parent: Vec<usize>,
    // base of the node u: the root of each node
    base: Vec<usize>,
    // match_with[u]: the node matched with u for the maximum matching
    match_with: Vec<Option<usize>>,
}
impl BlossomLCA {
    fn new(n: usize) -> Self {
        Self {
            parent: vec![usize::MAX; n],
            base: (0..n).collect(),
            match_with: vec![None; n],
        }
    }

    fn find_lca(
        &mut self,
        mut u: usize,
        mut v: usize,
        visited_token: isize,
        used: &mut Vec<isize>,
    ) -> usize {
        /* Return the least common ancestor of u and v
        Args:
            u, v: The nodes to find the least common ancestor of
            visited_token: The token to mark the visited nodes (-1: not visited, 0: Even, 1: Odd)
            used: The vector to mark the visited nodes
        Returns:
            The least common ancestor of u and v
         */
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

pub struct EdmondsMatching {
    pub adj: Vec<Vec<usize>>,
    // -1: unvisited, 0: Even, 1: Odd
    tree_type: Vec<isize>,
    lca: BlossomLCA,
}

impl EdmondsMatching {
    pub fn new(n: usize) -> Self {
        Self {
            adj: vec![vec![]; n],
            tree_type: vec![-1; n],
            lca: BlossomLCA::new(n),
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
        self.adj[v].push(u);
    }

    pub fn max_matching(&mut self) -> usize {
        /* Increases the number of matching by flipping the matched and unmatched edges
        along the augmenting path from the unmatched vertex start
         */
        let mut matching_size: usize = 0;
        let n: usize = self.adj.len();
        for v in 0..n {
            if self.lca.match_with[v].is_none() {
                if self.augment_path(v) {
                    matching_size += 1;
                }
            }
        }
        return matching_size;
    }

    fn augment_path(&mut self, start: usize) -> bool {
        /* Finds an augmenting path from the unmatched vertex start and increases
         the number of matching by flipping the matched and unmatched edges along the path
        Args:
            start: The unmatched vertex to find an augmenting path from
        Returns:
            true if an augmenting path is found, false otherwise
         */
        self.tree_type.fill(-1);
        self.lca.parent.fill(usize::MAX);
        let n: usize = self.adj.len();
        for v in 0..n {
            self.lca.base[v] = v;
        }
        let mut que = VecDeque::new();
        // Assign Even to the start
        self.tree_type[start] = 0;
        que.push_back(start);
        while let Some(u) = que.pop_front() {
            for i in 0..self.adj[u].len() {
                let v = self.adj[u][i];
                if self.lca.base[u] == self.lca.base[v] || self.lca.match_with[v] == Some(u) {
                    continue;
                }
                if self.tree_type[v] == -1 {
                    if let Some(next) = self.lca.match_with[v] {
                        // proceeding to the already matched node
                        self.tree_type[v] = 1;
                        self.lca.parent[v] = u;
                        self.lca.parent[next] = v;
                        self.tree_type[v] = 1;
                        self.tree_type[next] = 0;
                        que.push_back(next);
                    } else {
                        // Found an augmenting path
                        self.lca.parent[v] = u;
                        let mut curr: usize = v;
                        while curr != usize::MAX {
                            let curr_parent: usize = self.lca.parent[curr];
                            self.lca.match_with[curr_parent] = Some(curr);
                            self.lca.match_with[curr] = Some(curr_parent);
                            self.tree_type[curr] = 0;
                            self.tree_type[curr_parent] = 1;

                            curr = if self.lca.parent[curr_parent] != usize::MAX {
                                self.lca.parent[curr_parent]
                            } else {
                                break;
                            };
                        }
                        return true;
                    }
                } else if self.tree_type[v] == 0 {
                    // Found a blossom = outer cycle (Even -> Even edge)
                    let lca = self.lca.find_lca(u, v, 0, &mut self.tree_type);
                    self.contract_blossom(u, v, lca, &mut que);
                }
            }
        }
        return false;
    }

    fn contract_blossom(
        &mut self,
        mut u: usize,
        mut v: usize,
        lca: usize,
        que: &mut VecDeque<usize>,
    ) {
        /* Contracts the blossom
        Args:
            u, v: The vertices to contract the blossom
            lca: The least common ancestor of u and v
            que: The queue to add the vertices to
         */
        while self.lca.base[u] != lca {
            self.lca.parent[u] = v;
            let next: usize = if let Some(next) = self.lca.match_with[u] {
                next
            } else {
                break;
            };
            self.lca.parent[next] = u;
            if self.tree_type[next] == 1 {
                self.tree_type[next] = 0;
                que.push_back(next);
            }
            self.lca.base[u] = lca;
            self.lca.base[next] = lca;
            v = next;
            u = if self.lca.parent[v] != usize::MAX {
                self.lca.parent[v]
            } else {
                break;
            };
        }
        for i in 0..n {
            if self.lca.base[i] == self.lca.base[u] || self.lca.base[i] == self.lca.base[v] {
                self.lca.parent[i] = lca;
            }
        }
    }
}
