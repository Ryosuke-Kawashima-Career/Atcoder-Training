use proconio::{input, marker::Usize1};
use std::collections::VecDeque;
// Typical CP Problem 40
// Q. Project selection problem := there are some dependecies between tasks
// A. minimum cut = max flow problem
const INF: i64 = 1 << 60;

#[derive(Debug, Copy, Clone)]
struct Edge {
    to: usize,
    capacity: i64,
    rev: usize,
}

impl Edge {
    fn new(to: usize, capacity: i64, rev: usize) -> Self {
        Self { to, capacity, rev }
    }
}

struct Dinic {
    graph: Vec<Vec<Edge>>,
    level: Vec<usize>,
    iter: Vec<usize>,
}

impl Dinic {
    const INF_LEVEL: usize = usize::MAX;

    fn new(n: usize) -> Self {
        Self {
            graph: vec![vec![]; n],
            level: vec![0; n],
            iter: vec![0; n],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, capacity: i64) {
        let rev_from = self.graph[to].len();
        let rev_to = self.graph[from].len();
        self.graph[from].push(Edge::new(to, capacity, rev_from));
        self.graph[to].push(Edge::new(from, 0, rev_to));
    }

    fn get_max_flow(&mut self, source: usize, sink: usize) -> i64 {
        let mut max_flow = 0;
        while self.bfs_get_level(source, sink) {
            self.iter = vec![0; self.graph.len()];
            loop {
                let flow = self.send_flow(source, sink, i64::MAX);
                if flow == 0 {
                    break;
                }
                max_flow += flow;
            }
        }
        max_flow
    }

    fn bfs_get_level(&mut self, source: usize, sink: usize) -> bool {
        self.level = vec![Self::INF_LEVEL; self.graph.len()];
        self.level[source] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(source);
        while let Some(current) = queue.pop_front() {
            for edge in self.graph[current].iter() {
                if edge.capacity > 0 && self.level[edge.to] == Self::INF_LEVEL {
                    self.level[edge.to] = self.level[current] + 1;
                    queue.push_back(edge.to);
                }
            }
        }
        self.level[sink] != Self::INF_LEVEL
    }

    fn send_flow(&mut self, current: usize, sink: usize, incoming_flow: i64) -> i64 {
        if current == sink {
            return incoming_flow;
        }
        while self.iter[current] < self.graph[current].len() {
            let i = self.iter[current];
            let edge = self.graph[current][i];
            if edge.capacity > 0 && self.level[edge.to] == self.level[current] + 1 {
                let current_flow = incoming_flow.min(edge.capacity);
                let sent_flow = self.send_flow(edge.to, sink, current_flow);
                if sent_flow > 0 {
                    self.graph[current][i].capacity -= sent_flow;
                    self.graph[edge.to][edge.rev].capacity += sent_flow;
                    return sent_flow;
                }
            }
            self.iter[current] += 1;
        }
        0
    }
}

fn main() {
    input! {
        n: usize,
        w: i64,
        a: [i64; n],
    }

    let mut house_rewards: Vec<i64> = vec![0; n];
    for i in 0..n {
        house_rewards[i] = a[i] - w;
    }

    // key_holders[v]: list of houses that contain a key to house v
    let mut key_holders: Vec<Vec<usize>> = vec![vec![]; n];
    for v in 0..n {
        input! {
            k: usize,
            locked_houses: [Usize1; k],
        }
        for &target in locked_houses.iter() {
            key_holders[target].push(v);
        }
    }

    let result: i64 = solve(&key_holders, &house_rewards);
    println!("{}", result);
}

fn solve(key_holders: &Vec<Vec<usize>>, house_rewards: &Vec<i64>) -> i64 {
    let n: usize = house_rewards.len();
    let source: usize = n;     // Side S: Enter the house
    let sink: usize = n + 1;   // Side T: Do NOT enter the house
    let mut dinic = Dinic::new(n + 2);

    let mut offset: i64 = 0;

    for v in 0..n {
        if house_rewards[v] >= 0 {
            // Positive net reward: we gain house_rewards[v] up front,
            // and pay a penalty of house_rewards[v] if we do NOT enter v (v in T).
            offset += house_rewards[v];
            dinic.add_edge(source, v, house_rewards[v]);
        } else {
            // Negative net reward: we pay a penalty of |house_rewards[v]| if we DO enter v (v in S).
            dinic.add_edge(v, sink, -house_rewards[v]);
        }

        // To enter house v (v in S), we MUST also enter every house `holder` that has v's key (holder in S).
        // Equivalently, (v in S and holder in T) is forbidden -> capacity INF on edge v -> holder.
        for &holder in key_holders[v].iter() {
            dinic.add_edge(v, holder, INF);
        }
    }

    offset - dinic.get_max_flow(source, sink)
}
