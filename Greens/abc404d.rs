use proconio::{input, marker::Usize1};
const INF: usize = 1 << 60;
// ABC404D: Brute Force Attack is the first thing to go
// Q. You must see every animal at least twice. You can visit the same zoo multiple times.
// Input
// n: the number of zoos (<= 10) | m: the number of animals (<= 100)
// c: the cost of visiting each zoo
// k: the number of animals in each zoo
// ak: the animals in each zoo
// A: まずは全探索の典型例
fn main() {
    input! {n: usize, m: usize, c: [usize; n]}
    let mut animal_to_zoo: Vec<Vec<usize>> = Vec::new();
    for _ in 0..m {
        input! {k: usize, ak: [Usize1; k]}
        animal_to_zoo.push(ak);
    }
    // Note that Vec::new() is not initialized with the empty zoo vector
    let mut zoo_to_animals: Vec<Vec<usize>> = vec![vec![]; n];
    for animal in 0..m {
        for &zoo in &animal_to_zoo[animal] {
            zoo_to_animals[zoo].push(animal);
        }
    }
    let mut possible_zoos: Vec<Vec<usize>> = vec![vec![]];
    enum_zoos(n, &mut possible_zoos);
    let mut ans: usize = INF;
    for zoos in possible_zoos {
        let mut cost: usize = 0;
        let mut count_animals: Vec<usize> = vec![0; m];
        for &zoo in &zoos {
            cost += c[zoo];
            for &animal in &zoo_to_animals[zoo] {
                count_animals[animal] += 1;
            }
        }
        let mut valid: bool = true;
        for &count in &count_animals {
            if count < 2 {
                valid = false;
                break;
            }
        }
        if valid {
            ans = ans.min(cost);
        }
    }
    println!("{}", ans);
}

fn enum_zoos(n: usize, possible_zoos: &mut Vec<Vec<usize>>) {
    if n == 0 {
        return;
    }
    let mut current_zoos: Vec<Vec<usize>> = possible_zoos.clone();
    for zoo in current_zoos {
        let mut next_zoos: Vec<usize> = zoo.clone();
        let mut next_zoos_twice: Vec<usize> = zoo.clone();
        next_zoos.push(n - 1);
        next_zoos_twice.push(n - 1);
        next_zoos_twice.push(n - 1);
        possible_zoos.push(next_zoos);
        possible_zoos.push(next_zoos_twice);
    }
    enum_zoos(n - 1, possible_zoos);
}
