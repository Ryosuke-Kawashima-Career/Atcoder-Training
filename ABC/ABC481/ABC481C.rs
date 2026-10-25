use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {q: usize, query: [String; q]}
    let mut map = HashMap::new();
    let mut current_id: usize = 1;
    for i in 0..q {
        if let Some(id) = map.get(&query[i]) {
            println!("{}", id);
        } else {
            map.insert(query[i].clone(), current_id);
            println!("{}", current_id);
            current_id += 1;
        }
    }
}
