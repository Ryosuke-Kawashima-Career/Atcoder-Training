use proconio::input;
struct Fenwick {
    data: Vec<usize>,
}
impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick {
            data: vec![0; n + 1],
        }
    }

    fn update(&mut self, i: usize, delta: i64) {
        let mut idx = i;
        while idx <= self.data.len() - 1 {
            self.data[idx] += delta;
            idx += idx & (!idx + 1);
        }
    }

    fn query(&self, i: usize) -> usize {
        let mut sum = 0;
        let mut idx = i;
        while idx > 0 {
            sum += self.data[idx];
            idx -= idx & (!idx + 1);
        }
        sum
    }

    fn query_range(&self, l: usize, r: usize) -> usize {
        self.query(r) - self.query(l - 1)
    }
    fn get_at(&self, idx: usize) -> 
}

fn main() {
    input! {n: usize, q: usize, lrx: [(usize, usize, usize); q]}
    let x_to_range: Vec<Vec<(usize, usize)>> = vec![vec![]; q + 1];
    for &(l, r, x) in lrx.iter() {
        x_to_range[x].push((l, r));
    }

    for x in 1..=q {
        combine_range(&mut x_to_range[x]);
    }

    let mut fenwick = Fenwick::new(n);

    for x in 1..=q {
        for (l, r) in x_to_range[x].iter() {
            fenwick.update(*l, 1);
            fenwick.update(*r + 1, -1);
        }
    }
    for i in 0..n {
        print!("{} ", fenwick.get_at(i));
    }
    println!("");
}

fn combine_range(range: &mut Vec<(usize, usize)>) {
    range.sort();
    let mut combined: Vec<(usize, usize)> = vec![];
    let mut current_start = range[0].0;
    let mut current_end = range[0].1;
    for i in 1..range.len() {
        if range[i].0 <= current_end + 1 {
            current_end = current_end.max(range[i].1);
        } else {
            combined.push((current_start, current_end));
            current_start = range[i].0;
            current_end = range[i].1;
        }
    }
    combined.push((current_start, current_end));
    *range = combined;
}
