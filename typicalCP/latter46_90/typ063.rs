use proconio::input;

fn main() {
    input! {h: usize, w: usize, p: [[usize; w]; h]}
    let bits_h: usize = 1 << h;
    let mut ans: usize = 0;
    for bit_h in 0..bits_h {
        let ok_columns: usize = get_ok_columns(bit_h, &p);
        ans = ans.max(bit_h.count_ones() as usize * ok_columns);
    }
    println!("{}", ans);
}

fn get_ok_columns(bit_h: usize, p: &Vec<Vec<usize>>) -> usize {
    let h: usize = p.len();
    let w: usize = p[0].len();
    let mut result: Vec<usize> = vec![0; h * w + 1];
    for j in 0..w {
        let mut all_same: bool = true;
        let mut value_h: usize = 0;
        for i in 0..h {
            if (bit_h >> i) & 1 == 1 {
                if value_h == 0 {
                    value_h = p[i][j];
                } else {
                    if value_h != p[i][j] {
                        all_same = false;
                        break;
                    }
                }
            }
        }
        if all_same {
            result[value_h] += 1;
        }
    }
    *result.iter().max().unwrap()
}
