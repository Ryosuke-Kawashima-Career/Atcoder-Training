use proconio::{
    input,
    marker::{Chars, Usize1},
};
const INF: usize = 1 << 30;
const N1: usize = 1usize.wrapping_neg();
const N2: usize = 2usize.wrapping_neg();
const D4: [(usize, usize); 4] = [(N1, 0), (0, N1), (0, 1), (1, 0)];
const D4_2: [(usize, usize); 4] = [(N2, 0), (0, N2), (0, 2), (2, 0)];
// Q. the walker can turn walls into paths by kicking 1~2 blocks ahead.
// find the minimum number of kicks to reach the goal.
/* A. 0-1 BFS
各頂点は区画と一対一に対応する。特に、グラフ G は H × W 個の頂点を持つ。各区画 C について、次のように辺を張る。
上下左右に隣接する区画 C′ が道である場合、区画 C に対応する頂点から区画 C′ に対応する頂点へコスト 0 の辺を張る。
上下左右に隣接する区画 C′ が壁である場合、区画 C に対応する頂点から区画 C′ に対応する頂点へコスト 1 の辺を張る。
さらに、区画 C から上下左右のいずれかの方向を向いたとき、2 マス先にある区画 C′ について、C と C′ の間にある区画が壁である、または区画 C′ 自体が壁である場合には、区画 C に対応する頂点から区画 C′ に対応する頂点へコスト 1 の辺を張る。
このとき、グラフ G において、高橋君が最初にいる区画に対応する頂点から魚屋がある区画に対応する頂点への最短距離（経路上の辺の重みの総和の最小値）が問題の答えとなる。
*/
fn main() {
    // start: (a, b) and goal: (c, d)
    input! {h: usize, w: usize, s: [Chars; h], a: Usize1, b: Usize1, c: Usize1, d: Usize1}
    let dist = bfs(a, b, &s);
    let ans = dist[c][d];
    println!("{}", ans);
}

fn bfs(h0: usize, w0: usize, graph: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let mut dist: Vec<Vec<usize>> = vec![vec![INF; w]; h];
    dist[h0][w0] = 0;
    let mut que = std::collections::VecDeque::new();
    que.push_back((h0, w0));

    while let Some((y, x)) = que.pop_front() {
        for &(dy, dx) in D4.iter() {
            let next_y: usize = y.wrapping_add(dy);
            let next_x: usize = x.wrapping_add(dx);
            if next_y < h && next_x < w {
                if graph[next_y][next_x] == '#' {
                    if dist[next_y][next_x] > dist[y][x] + 1 {
                        dist[next_y][next_x] = dist[y][x] + 1;
                        que.push_back((next_y, next_x));
                    }
                } else {
                    if dist[next_y][next_x] > dist[y][x] {
                        dist[next_y][next_x] = dist[y][x];
                        que.push_front((next_y, next_x));
                    }
                }
            }
        }
        for dir in 0..4 {
            let (dy1, dx1) = D4[dir];
            let (dy2, dx2) = D4_2[dir];
            let next_y1: usize = y.wrapping_add(dy1);
            let next_x1: usize = x.wrapping_add(dx1);
            let next_y2: usize = y.wrapping_add(dy2);
            let next_x2: usize = x.wrapping_add(dx2);
            if next_y2 < h && next_x2 < w {
                let mut count_walls: usize = 0;
                if graph[next_y1][next_x1] == '#' {
                    count_walls += 1;
                }
                if graph[next_y2][next_x2] == '#' {
                    count_walls += 1;
                }
                if count_walls > 0 {
                    if dist[next_y2][next_x2] > dist[y][x] + 1 {
                        dist[next_y2][next_x2] = dist[y][x] + 1;
                        que.push_back((next_y2, next_x2));
                    }
                } else {
                    if dist[next_y2][next_x2] > dist[y][x] {
                        dist[next_y2][next_x2] = dist[y][x];
                        que.push_front((next_y2, next_x2));
                    }
                }
            }
        }
    }

    return dist;
}
