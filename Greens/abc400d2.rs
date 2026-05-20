use proconio::{
    input,
    marker::{Chars, Usize1},
};
const INF: i64 = 1 << 30;
const N1: usize = 1usize.wrapping_neg();
const N2: usize = 2usize.wrapping_neg();
const D4: [(usize, usize); 4] = [(N1, 0), (0, N1), (0, 1), (1, 0)];
const D4_2: [(usize, usize); 4] = [(N2, 0), (0, N2), (0, 2), (2, 0)];
// ABC400D
// Q. the walker can turn walls into paths by kicking 1~2 blocks ahead.
// find the minimum number of kicks to reach the goal.
/* A. 0-1 BFS <- HW <= 10^6という制約に着目する！
各頂点は区画と一対一に対応する。特に、グラフ G は H × W 個の頂点を持つ。各区画 C について、次のように辺を張る。
上下左右に隣接する区画 C′ が道である場合、区画 C に対応する頂点から区画 C′ に対応する頂点へコスト 0 の辺を張る。
上下左右に隣接する区画 C′ が壁である場合、区画 C に対応する頂点から区画 C′ に対応する頂点へコスト 1 の辺を張る。
さらに、区画 C から上下左右のいずれかの方向を向いたとき、2 マス先にある区画 C′ について、C と C′ の間にある区画が壁である、または区画 C′ 自体が壁である場合には、区画 C に対応する頂点から区画 C′ に対応する頂点へコスト 1 の辺を張る。
このとき、グラフ G において、高橋君が最初にいる区画に対応する頂点から魚屋がある区画に対応する頂点への最短距離（経路上の辺の重みの総和の最小値）が問題の答えとなる。
*/
fn main() {
    // start: (a, b) and goal: (c, d)
    input! {h: usize, w: usize, s: [Chars; h], a: Usize1, b: Usize1, c: Usize1, d: Usize1}
    let mut graph: Vec<Vec<(usize, i64)>> = construct_graph(&s);
    let dist = bfs01(a * w + b, &graph);
    let ans = dist[c * w + d];
    println!("{}", ans);
}

fn construct_graph(s: &Vec<Vec<char>>) -> Vec<Vec<(usize, i64)>> {
    let h: usize = s.len();
    let w: usize = s[0].len();
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; h * w];
    for y in 0..h {
        for x in 0..w {
            let vertex: usize = y * w + x;
            for dir1 in 0..4 {
                let (dy1, dx1) = D4[dir1];
                let next_y1: usize = y.wrapping_add(dy1);
                let next_x1: usize = x.wrapping_add(dx1);
                if next_y1 < h && next_x1 < w {
                    let next_vertex1: usize = next_y1 * w + next_x1;
                    if s[next_y1][next_x1] == '#' {
                        graph[vertex].push((next_vertex1, 1));
                    } else {
                        graph[vertex].push((next_vertex1, 0));
                    }
                }
            }
            for dir2 in 0..4 {
                let (dy1, dx1) = D4[dir2];
                let (dy2, dx2) = D4_2[dir2];
                let next_y1: usize = y.wrapping_add(dy1);
                let next_x1: usize = x.wrapping_add(dx1);
                let next_y2: usize = y.wrapping_add(dy2);
                let next_x2: usize = x.wrapping_add(dx2);
                if next_y2 < h && next_x2 < w {
                    let next_vertex1: usize = next_y1 * w + next_x1;
                    let next_vertex2: usize = next_y2 * w + next_x2;
                    if s[next_y1][next_x1] == '#' || s[next_y2][next_x2] == '#' {
                        graph[vertex].push((next_vertex2, 1));
                    } else {
                        graph[vertex].push((next_vertex2, 0));
                    }
                }
            }
        }
    }
    return graph;
}

fn bfs01(start: usize, graph: &Vec<Vec<(usize, i64)>>) -> Vec<i64> {
    let n: usize = graph.len();
    let mut dist: Vec<i64> = vec![INF; n];
    dist[start] = 0;
    let mut que = std::collections::VecDeque::new();
    que.push_back(start);

    while let Some(vertex) = que.pop_front() {
        for &(next, cost) in graph[vertex].iter() {
            if cost == 0 {
                if dist[next] > dist[vertex] {
                    dist[next] = dist[vertex];
                    que.push_front(next);
                }
            } else {
                if dist[next] > dist[vertex] + 1 {
                    dist[next] = dist[vertex] + 1;
                    que.push_back(next);
                }
            }
        }
    }

    return dist;
}
