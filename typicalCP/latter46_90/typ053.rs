use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = stdin.lock();
    let mut writer = io::BufWriter::new(stdout.lock());

    let t = read_value(&mut reader) as usize;
    for _ in 0..t {
        let n = read_value(&mut reader) as usize;
        let answer = get_max_ai(n, |index| query(index, &mut reader, &mut writer));
        writeln!(writer, "! {}", answer).unwrap();
        writer.flush().unwrap();
    }
}

fn get_max_ai(n: usize, mut query: impl FnMut(usize) -> i64) -> i64 {
    let mut fib = vec![0, 1];
    while *fib.last().unwrap() < n + 1 {
        let len = fib.len();
        fib.push(fib[len - 1] + fib[len - 2]);
    }

    let mut cache = vec![None; n + 1];
    let mut value_at = |index: usize| {
        if index > n {
            // A strictly decreasing negative tail preserves unimodality.
            return -(index as i64);
        }
        *cache[index].get_or_insert_with(|| query(index))
    };

    // The maximum stays strictly inside (left, left + fib[order]).
    let mut left = 0;
    let mut order = fib.len() - 1;
    while order > 3 {
        let x = left + fib[order - 2];
        let y = left + fib[order - 1];
        if value_at(x) < value_at(y) {
            left = x;
        }
        // Either remaining interval has length fib[order - 1].
        // One comparison point is reused in the next iteration.
        order -= 1;
    }

    // fib[3] = 2, so only left + 1 remains inside the interval.
    value_at(left + 1)
}

fn query(index: usize, reader: &mut impl BufRead, writer: &mut impl Write) -> i64 {
    writeln!(writer, "? {}", index).unwrap();
    writer.flush().unwrap();
    read_value(reader)
}

fn read_value(reader: &mut impl BufRead) -> i64 {
    let mut line = String::new();
    if reader.read_line(&mut line).unwrap() == 0 {
        std::process::exit(0);
    }
    let value = line.trim().parse().unwrap();
    if value == -1 {
        std::process::exit(0);
    }
    value
}
