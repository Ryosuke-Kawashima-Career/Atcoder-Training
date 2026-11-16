use proconio::{input, marker::Usize1};

fn main() {
    input! {n: usize, m: usize, ab: [(Usize1, Usize1); m]}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for &(a, b) in &ab {
        // directed graph
        graph[a].push(b);
    }
    let sc = SCC::new(&graph);
    let groups: Vec<Vec<usize>> = sc.scc();
    let mut ans: usize = 0;
    for group in groups {
        let n_members: usize = group.len();
        ans += n_members * (n_members - 1) / 2;
    }
    println!("{}", ans);
}

struct SCC<'a> {
    graph: &'a [Vec<usize>],
}

impl<'a> SCC<'a> {
    fn new(graph: &'a [Vec<usize>]) -> Self {
        Self { graph }
    }

    fn scc(&self) -> Vec<Vec<usize>> {
        let n = self.graph.len();
        let mut order = vec![usize::MAX; n];
        let mut low_link = vec![0; n];
        let mut on_stack = vec![false; n];
        let mut tarjan_stack = Vec::new();
        let mut groups: Vec<Vec<usize>> = Vec::new();
        let mut dfs_stack: Vec<(usize, usize)> = Vec::new();
        let mut timer = 0;

        for v in 0..n {
            if order[v] != usize::MAX {
                continue;
            }

            order[v] = timer;
            low_link[v] = timer;
            timer += 1;
            tarjan_stack.push(v);
            on_stack[v] = true;
            dfs_stack.push((v, 0));

            while let Some(&(current, edge_index)) = dfs_stack.last() {
                if edge_index < self.graph[current].len() {
                    let next = self.graph[current][edge_index];
                    dfs_stack.last_mut().unwrap().1 += 1;

                    if order[next] == usize::MAX {
                        order[next] = timer;
                        low_link[next] = timer;
                        timer += 1;
                        tarjan_stack.push(next);
                        on_stack[next] = true;
                        dfs_stack.push((next, 0));
                    } else if on_stack[next] {
                        low_link[current] = low_link[current].min(order[next]);
                    }
                } else {
                    dfs_stack.pop();

                    if let Some(&(parent, _)) = dfs_stack.last() {
                        low_link[parent] = low_link[parent].min(low_link[current]);
                    }

                    if low_link[current] == order[current] {
                        let mut group = Vec::new();
                        loop {
                            let member = tarjan_stack.pop().unwrap();
                            on_stack[member] = false;
                            group.push(member);
                            if member == current {
                                break;
                            }
                        }
                        groups.push(group);
                    }
                }
            }
        }

        groups
    }
}
