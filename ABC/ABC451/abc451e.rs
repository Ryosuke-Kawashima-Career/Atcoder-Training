use proconio::input;

fn main() {
    input! {n: usize}
    let mut length_requirements: Vec<Vec<i64>> = vec![vec![0; n]; n];
    for v1 in 0..n - 1 {
        input! {a: [i64; n - v1 - 1]}
        for v2 in 0..n - v1 - 1 {
            length_requirements[v1][v1 + v2 + 1] = a[v2];
            length_requirements[v1 + v2 + 1][v1] = a[v2];
        }
    }
    if does_tree_exist(&length_requirements) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn does_tree_exist(length_requirements: &Vec<Vec<i64>>) -> bool {
    let n: usize = length_requirements.len();
    let mut edges: Vec<(usize, usize, i64)> = Vec::new();
    for v1 in 0..n {
        for v2 in v1 + 1..n {
            edges.push((v1, v2, length_requirements[v1][v2]));
        }
    }
    edges.sort_by_key(|&(_, _, l)| l);
    let mut graph: Vec<Vec<usize, i64>> = vec![vec![]; n];
    for i in 0..n - 1 {
        graph[i].push((i + 1, length_requirements[i][i + 1]));
        graph[i + 1].push((i, length_requirements[i][i + 1]));
    }
    does_satisfy_path_lengths(&graph, length_requirements)
}

fn does_satisfy_path_lengths(
    graph: &Vec<Vec<(usize, i64)>>,
    length_requirements: &Vec<Vec<i64>>,
) -> bool {
    let n: usize = graph.len();
    for v1 in 0..n {
        for v2 in v1 + 1..n {
            if get_path_length(graph, v1, v2) != length_requirements[v1][v2] {
                return false;
            }
        }
    }
    true
}

fn get_path_length(graph: &Vec<Vec<(usize, i64)>>, start: usize, end: usize) -> i64 {
    let n: usize = graph.len();
    let mut distances: Vec<i64> = vec![i64::MAX; n];
    distances[start] = 0;
    let mut priority_queue: BinaryHeap<(i64, usize)> = BinaryHeap::new();
    priority_queue.push((0, start));
    while let Some((distance, current_node)) = priority_queue.pop() {
        if distance > distances[current_node] {
            continue;
        }
        if current_node == end {
            return distance;
        }
        for &(neighbor, weight) in graph[current_node].iter() {
            let new_distance = distance + weight;
            if new_distance < distances[neighbor] {
                distances[neighbor] = new_distance;
                priority_queue.push((new_distance, neighbor));
            }
        }
    }
    distances[end]
}
