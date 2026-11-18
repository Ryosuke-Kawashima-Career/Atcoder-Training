use std::fmt::Display;
use std::io::{self, BufRead, Write};

struct InteractiveIO<R: BufRead, W: Write> {
    reader: R,
    writer: W,
}
impl<R: BufRead, W: Write> InteractiveIO<R, W> {
    fn new(reader: R, writer: W) -> Self {
        Self { reader, writer }
    }
    fn read_int<T>(&mut self) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.read_string().parse().unwrap()
    }
    fn read_string(&mut self) -> String {
        let mut buf = String::new();
        if self.reader.read_line(&mut buf).unwrap() == 0 {
            std::process::exit(0);
        }
        buf.trim().to_string()
    }
    // Every protocol message must reach the judge before reading its reply.
    fn write_value(&mut self, value: impl Display) {
        writeln!(self.writer, "{}", value).unwrap();
        self.writer.flush().unwrap();
    }

    fn query(&mut self, index: usize) -> i64 {
        self.write_value(format_args!("? {}", index));
        let value = self.read_int();
        if value == -1 {
            std::process::exit(0);
        }
        value
    }

    fn answer(&mut self, value: i64) {
        self.write_value(format_args!("! {}", value));
    }
}
//
fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut io = InteractiveIO::new(stdin.lock(), io::BufWriter::new(stdout.lock()));

    let t: usize = io.read_int();
    for _ in 0..t {
        let n: usize = io.read_int();
        let answer = get_max_ai(n, |index| io.query(index));
        io.answer(answer);
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
