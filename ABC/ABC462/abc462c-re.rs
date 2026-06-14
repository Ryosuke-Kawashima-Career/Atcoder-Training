use proconio::input;
// ABC462C
// Q. there is a 2D coordinate(1<=x<=N, 1<=y<=N) and N points with the x and y positions the permutaion of 1..=N.
// Find the number of points (x,y) such that its rectangle((0,0)to(x,y)) includes no other points.
// A. Greedy Algorithm and Sweepline Algorithm
fn main() {
    input! {n: usize, mut xy: [(usize, usize); n]}
    let mut min_y: usize = usize::MAX;
    xy.sort();
    let mut not_inversed: usize = 0;
    for &(_, y) in xy.iter() {
        if min_y > y {
            not_inversed += 1;
            min_y = y;
        }
    }
    println!("{}", not_inversed);
}
