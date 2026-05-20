use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {n: usize, ab: [(Usize1, Usize1); n], m: usize, s: [Chars; m]}
    let mut answer: Vec<String> = Vec::new();
    // length, characters
    let mut map: HashMap<usize, HashSet<(usize, char)>> = HashMap::new();
    for string in 0..m {
        let length = s[string].len();
        let mut set: HashSet<(usize, char)> = HashSet::new();
        for (i, character) in s[string].iter().enumerate() {
            set.insert((i, *character));
        }
        if map.contains_key(&length) {
            map.get_mut(&length).unwrap().extend(set);
        } else {
            map.insert(length, set);
        }
    }
    for spine in 0..m {
        let mut is_ok = true;
        for bone in 0..n {
            let (length, character_idx) = ab[bone];
            let character = s[spine][character_idx];
            if let Some(set) = map.get(&length) {
                if !set.contains(&(character_idx, character)) {
                    is_ok = false;
                    break;
                }
            } else {
                is_ok = false;
                break;
            }
        }
        if is_ok {
            answer.push(String::from("Yes"));
        } else {
            answer.push(String::from("No"));
        }
    }
    for string in answer {
        println!("{}", string);
    }
}
