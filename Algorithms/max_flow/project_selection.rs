use proconio::input;
// Tessoku B68
const INF: i64 = 1 << 60;
#[derive(Debug, Copy, Clone)]
struct Edge {
    to: usize,
    cap: i64,
    rev: usize,
}
struct MaxFlow {
    graph: Vec<Vec<Edge>>,
    iter: Vec<usize>,
    level: Vec<isize>,
}
impl MaxFlow {
    fn new(n: usize) -> Self {
        Self {
            graph: vec![vec![]; n],
            iter: vec![0; n],
            level: vec![-1; n],
        }
    }
    fn add_edge(&mut self, v1: usize, v2: usize, cap: i64) {
        /* Add an edge from v1 to v2 with the capacity = cap */
        let rev_index1: usize = self.graph[v2].len();
        let rev_index2: usize = self.graph[v1].len();
        self.graph[v1].push(Edge {
            to: v2,
            cap: cap,
            rev: rev_index1,
        });
        self.graph[v2].push(Edge {
            to: v1,
            cap: 0,
            rev_index2,
        })
    }
    fn bfs(&mut self, source: usize, sink: usize) -> bool {
        /* Calculates the level graph */
        self.level.fill(-1);
        let mut que = std::collections::VecDeque::new();
        que.push_back(source);
        self.level[source] = 0;
        while let Some(v) = que.pop_front() {
            for edge in self.graph[v].iter() {
                let next: usize = edge.to;
                if edge.cap > 0 && self.level[next] == -1 {
                    self.level[next] = self.level[v] + 1;
                    que.push_back(next);
                }
            }
        }
        self.level[sink] != -1
    }
    fn dfs(&mut self, curr: usize, sink: usize, flow: i64) -> i64 {
        /* Calculates how much flow the residual graph can push */
        if curr == sink {
            return 0;
        }
        let mut pushed_flow: i64 = 0;
        let num_edges: usize = self.graph[curr].len();
        for i in self.iter[curr]..num_edges {
            let edge = self.graph[curr][i];
            let next: usize = edge.to;
            let capacity: i64 = edge.cap;
            if self.level[curr] + 1 == self.level[next] {
                let next_flow: i64 = self.dfs(next, sink, capacity.min(flow));
                if next_flow > 0 {
                    pushed_flow += next_flow;
                    self.graph[curr][i].cap -= next_flow;
                    let edge_rev: usize = edge.rev;
                    self.graph[next][edge_rev].cap += next_flow;
                }
            }
        }
        self.iter[curr] = num_edges;
        return pushed_flow;
    }
    fn max_flow(&mut self, source: usize, sink: usize) -> i64 {
        let mut total_flow: i64 = 0;
        loop {
            if !self.bfs(source, sink) {
                // There is no pushable flow <- supported by the convexity
                break;
            }
            self.iter.fill(0);
            total_flow += self.dfs(source, sink, INF);
        }
        return total_flow;
    }
}
/* s --> v1 --> v2 --> t
s: source := selected cut set vs. t: sink := unselected cut set
edge of v from t: the cost of being chosen
 */
fn main() {
    input! {n: usize, m: usize, p: [i64; n], ab: [(usize, usize); m]};
    let mut dinic = MaxFlow::new(n + 2);
    for station in 1..=n {
        if p[station - 1] > 0 {
            dinic.add_edge(station, n + 1, p[station - 1]);
        } else {
            dinic.add_edge(0, station, -p[station - 1]);
        }
    }
    for &(a, b) in iter() {
        dinic.add_edge(a, b, INF);
    }
    let ans: i64 = dinic.max_flow(0, n + 1);
    println!("{}", ans);
}
