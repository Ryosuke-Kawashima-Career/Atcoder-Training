use std::io::{stdin, stdout, BufRead, BufReader, Write};
use proconio::{input, source::line::LineSource};
fn main() {
    input!{n: usize, k: usize, p: usize, r: usize, a: [i64; p-1]}
    let mut sheets: Vec<Vec<char>> = get_sheets(n, k, p, &a);
    print_sheets(&sheets);
    for _query in 0..r {
        stdout().flush().unwrap();
        input!{from source, ar: usize}
        let operations: Vec<(usize, usize)> = get_operations(ar, &mut sheets);
        print_operations(&operations);
    }
}

fn print_sheets(sheets: &Vec<Vec<char>>) {
    for sheet in sheets {
        println!("{}", sheet.iter().collect::<String>());
    }
}

fn print_operations(operations: &Vec<(usize, usize)>) {
    /* Args: Insertion sheet and position 
     */
    println!("{}", operations.len());
    for (i, j) in operations {
        println!("{} {}", i + 1, j + 1);
    }
}

fn get_sheets(n: usize, k: usize, p: usize, a: &Vec<i64>) -> Vec<Vec<char>> {
    let mut sheets: Vec<Vec<char>> = vec![vec!['.'; k]; n];
    for i in 0..p-1 {
        let mut x = a[i] as usize;
        let mut y = i + 1;
        while x > 0 {
            if sheets[y][x] == '.' {
                sheets[y][x] = '#';
                break;
            }
            x -= 1;
        }
    }
    sheets
}

fn get_operations(ar: usize, sheets: &mut Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut operations: Vec<(usize, usize)> = vec![];
    let mut x = ar;
    while x > 0 {
        let mut y = 0;
        while y < sheets.len() && sheets[y][x] == '#' {
            y += 1;
        }
        if y == sheets.len() {
            break;
        }
        operations.push((y, x));
        sheets[y][x] = '#';
        x -= 1;
    }
    operations
}

fn query<R: BufRead>(x: usize, source: &mut LineSource<R>) -> usize {
    println!("{}", x);
    stdout().flush().unwrap();
    input! {
        from source,
        y: usize,
    }
    y
}
