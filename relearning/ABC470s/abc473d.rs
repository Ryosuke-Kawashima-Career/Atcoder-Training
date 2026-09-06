use proconio::input;

fn main() {
    input! {n: usize, k: usize}
    let mut array: Vec<usize> = Vec::with_capacity(n);
    lexico_recursive(0, n, k, &mut array);
}

fn lexico_recursive(
    curr_index: usize,
    n: usize,
    remain_sum: usize,
    array: &mut Vec<usize>,
) -> bool {
    if curr_index == n - 1 {
        if remain_sum % n == 0 && remain_sum >= remain_sum / n {
            array.push(remain_sum / n);
            let result: String = array
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            println!("{result}");
            array.pop();
            return true;
        } else {
            return false;
        }
    }
    let mut exist_ok: bool = false;
    for num in 0..=(remain_sum) {
        array.push(num);
        if remain_sum - (curr_index + 1) * num >= 0
            && lexico_recursive(
                curr_index + 1,
                n,
                remain_sum - (curr_index + 1) * num,
                array,
            )
        {
            exist_ok = true;
        }
        array.pop();
    }
    return exist_ok;
}
