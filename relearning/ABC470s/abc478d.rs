use proconio::input;

#[derive(Debug, Copy, Clone)]
struct Event {
    // Zero-based, half-open interval [l, r).
    l: usize,
    r: usize,
    x: usize,
}

fn main() {
    input! {n: usize, q: usize, lrx: [(usize, usize, usize); q]}
    let mut imos = vec![0i64; n + 1];
    let mut events = Vec::with_capacity(q);
    for (l, r, x) in lrx {
        events.push(Event { l: l - 1, r, x });
    }
    // Merge each value's intervals in increasing left-endpoint order.
    events.sort_unstable_by_key(|e| (e.x, e.l, e.r));

    // Q >= 1, so the first interval always exists.
    let mut current = events[0];
    for &next in &events[1..] {
        if next.x == current.x && next.l <= current.r {
            // Overlapping or adjacent intervals of the same value.
            current.r = current.r.max(next.r);
        } else {
            // Finish the previous union interval before starting the next one.
            imos[current.l] += 1;
            imos[current.r] -= 1;
            current = next;
        }
    }
    // No following interval exists to trigger finishing the final interval.
    imos[current.l] += 1;
    imos[current.r] -= 1;

    let mut count = 0;
    let mut answer = Vec::with_capacity(n);
    for &delta in &imos[..n] {
        count += delta;
        answer.push(count.to_string());
    }
    println!("{}", answer.join(" "));
}
