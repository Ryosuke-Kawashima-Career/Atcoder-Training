use proconio::input;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}
    let mut class_members: Vec<usize> = vec![0; k + 1];
    for i in 0..n {
        class_members[a[i]] += 1;
    }
    let mut number_to_classes: Vec<Vec<usize>> = vec![vec![]; k + 1];
    let mut max_member: usize = 0;
    let mut ans: usize = 0;
    for i in 0..=k {
        number_to_classes[class_members[i]].push(i);
        max_member = max_member.max(class_members[i]);
    }
    ans += number_to_classes[max_member].len();
    if max_member > 0 {
        ans += number_to_classes[max_member - 1].len();
    }
    println!("{}", ans);
}
