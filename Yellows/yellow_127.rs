use proconio::input;
// GRL_7_A - Bipartite Matching
// Q. Bipartite Maximum Matching
// A. Network Maximum Flow (Ford-fulkerson or Dinic methods)
struct Dinic {
    data: Vec<i64>,
}
impl Dinic {
    fn new() -> Self {

    }
    fn add_edge(&mut self, from: usize, to: usize, capacity: i64) {
        
    }
    fn get_max_flow(&mut self, source: usize, sink: usize) -> i64 {
        
    }
}
fn main() {
    input!{x_num: usize, y_num: usize, edges_num: usize, edges: [(usize, usize); edges_num]}
    let mut network_flow = Dinic::new(x_num + y_num + 2);
    // start: 0, x's indexes: 1..=x_num, y's indexes: x_num+1..=x_num+y_num, end: x_num+y_num+1
    for x_index in 1..=x_num {
        network_flow.add_edge(0, x_index, 1);
    }
    for y_index in x_num+1..=x_num+y_num {
        network_flow.add_edge(y_index, x_num+y_num+1);
    }
    for &(u, v) in edges.iter() {
        let x_index: usize = u + 1;
        let y_index: usize = v + x_num + 1;
        network_flow.add_edge(x_index, y_index, 1);
    }
    let ans = network_flow.get_max_flow(0, x_num+y_num+1);
    println!("{}", ans);
}
