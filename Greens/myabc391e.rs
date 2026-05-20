#[allow(unused)]
use proconio::input;

fn main() {
    input! {n: usize, a: String}
    let a: Vec<usize> = a.chars().map(|c| (c == '1') as usize).collect();
    let a1 = get_majority(&a);
    let ans = get_indices_to_change(&a, a1);
}

fn get_indices_to_change(a: &[usize], a1: usize) -> (usize, ) {
    /* Returns the minimum operations to flip the result  */
    if a.len() == 1 {
        return 0;
    }
    if a.len() == 3 {
        let sum: usize = a.iter().sum();
        if sum == 3 {
            return if a1 == 1 { vec![0, 1] } else { vec![] };
        } else if sum == 2 {
            return if a1 == 1 { 
                vec![a.iter().position(|&x| x == 0).unwrap()]
            } else {
                vec![i for i in 0..3 if a[i] == 1]
            };
        } else if sum == 1 {
            return if a1 == 0 { 0 } else { 1 };
        } else {
            return if a1 == 0 { 0 } else { 2 };
        }
    }
    let mut indices_to_change: Vec<usize> = Vec::new();
}

fn get_majority(a: &Vec<usize>) -> usize {
    /* Recursive majority vote algorithm
    Args:
        a: input array of size 3^k
    Returns:
        usize: the majority element (0 or 1)
     */
    if a.len() == 1 {
        return a[0];
    }
    if a.len() == 3 {
        let sum: usize = a.iter().sum();
        return (sum >= 2) as usize;
    }
    let n: usize = a.len();
    let chunk_size = n / 3;
    let mut sub_majorities: Vec<usize> = Vec::new();
    for chunk in 0..chunk_size {
        let start: usize = chunk * 3;
        let end: usize = start + 3;
        let sub_majority = get_majority(&a[start..end].to_vec());
        sub_majorities.push(sub_majority);
    }
    return get_majority(&sub_majorities);
}