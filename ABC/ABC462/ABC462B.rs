use proconio::input;

fn main() {
    input! {n: usize}
    let mut from_people: Vec<Vec<usize>> = vec![Vec::<usize>::new(); n];
    for from_person in 0..n {
        input! {k: usize, to_person_from: [usize; k]}
        for &to_person in to_person_from.iter() {
            from_people[to_person - 1].push(from_person);
        }
    }

    for to_person in 0..n {
        from_people[to_person].sort();
        let x: usize = from_people[to_person].len();
        print!("{}", x);
        for &from_person in from_people[to_person].iter() {
            print!(" {}", from_person + 1);
        }
        println!("");
    }
}
