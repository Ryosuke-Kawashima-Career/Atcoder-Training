fn main() {
    let mut dinic = Dinic::new(5);
    dinic.add_edge(0, 1, 10); // S -> A
    dinic.add_edge(0, 2, 10); // S -> B
    dinic.add_edge(1, 3, 4); // A -> C
    dinic.add_edge(1, 4, 8); // A -> T
    dinic.add_edge(2, 3, 9); // B -> C
    dinic.add_edge(3, 4, 10); // C -> T
    println!("Maximum Flow: {}", dinic.max_flow(0, 4)); // Output: 18
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    to: usize,
    cap: i64,
    rev_idx: usize,
}

struct Dinic {
    graph: Vec<Vec<Edge>>,
    level: Vec<usize>,
    ptr: Vec<usize>,
}

impl Dinic {
    fn new(n: usize) -> Self {
        Self {
            graph: vec![vec![]; n],
            level: vec![0; n],
            ptr: vec![0; n],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
        let rev_idx_from: usize = self.graph[to].len();
        let rev_idx_to: usize = self.graph[from].len();
        self.graph[from].push(Edge {
            to,
            cap,
            rev_idx: rev_idx_from,
        });
        self.graph[to].push(Edge {
            to: from,
            cap: 0,
            rev_idx: rev_idx_to,
        });
    }

    fn bfs(&mut self, source: usize, sink: usize) -> bool {
        let n: usize = self.graph.len();
        self.level.fill(n);
        self.level[source] = 0;

        let mut que = std::collections::VecDeque::new();
        que.push_back(source);

        while let Some(u) = que.pop_front() {
            for edge in &self.graph[u] {
                if edge.cap > 0 && self.level[edge.to] == n {
                    self.level[edge.to] = self.level[u] + 1;
                    que.push_back(edge.to);
                }
            }
        }
        self.level[sink] != n
    }

    fn dfs(&mut self, u: usize, sink: usize, pushed: i64) -> i64 {
        if pushed == 0 || u == sink {
            return pushed;
        }

        // Index boundary check
        while self.ptr[u] < self.graph[u].len() {
            let idx = self.ptr[u];
            // Index boundary check takes effect here.
            let edge = self.graph[u][idx];

            // Level check
            if edge.cap > 0 && self.level[edge.to] == self.level[u] + 1 {
                let tr = self.dfs(edge.to, sink, pushed.min(edge.cap));
                if tr > 0 {
                    self.graph[u][idx].cap -= tr;
                    self.graph[edge.to][edge.rev_idx].cap += tr;
                    return tr;
                }
            }
            // if the edge was not consumed, it will be re-considered next time.
            self.ptr[u] += 1;
        }
        0
    }

    fn max_flow(&mut self, source: usize, sink: usize) -> i64 {
        let mut total_flow: i64 = 0;
        while self.bfs(source, sink) {
            // Reset of the edge count is needed.
            self.ptr.fill(0);
            loop {
                let push = self.dfs(source, sink, i64::MAX);
                if push == 0 {
                    break;
                }
                total_flow += push;
            }
        }
        total_flow
    }
}
