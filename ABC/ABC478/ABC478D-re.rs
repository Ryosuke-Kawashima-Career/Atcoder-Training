use proconio::input;
// ABC478 D
// Q. Insert the value x to sets [l, r). Find the numbers of elements of these sets
// A. Sweepline Algorithm + Event sort.
// event sort
#[derive(Copy, Clone)]
struct Event {
    pos: usize,
    x: usize,
    val: isize,
}
fn main() {
    input! {n: usize, q: usize, mut lrx: [(usize, usize, usize); q]}
    lrx.sort_unstable_by_key(|tup| tup.2);
    // event: entry and deletion
    let mut events: Vec<Event> = Vec::new();
    for i in 0..q {
        let (l, r, x) = lrx[i];
        // Entry Event
        events.push(Event {
            pos: l - 1,
            x: x - 1,
            val: 1,
        });
        // Deleting the event
        events.push(Event {
            pos: r,
            x: x - 1,
            val: -1,
        });
    }
    // sentinel event
    events.push(Event {
        pos: n,
        x: 0,
        val: 0,
    });
    // sort based on left and right
    events.sort_unstable_by(|a, b| a.pos.cmp(&b.pos));
    // start counting the number of elements | counter[x] := how many events of `x` there are.
    let mut counter: Vec<isize> = vec![0; q];
    let mut ans: isize = 0;
    // cur_pos := current target segment
    let mut cur_pos: usize = 0;
    for event in events {
        let (pos, x, val) = (event.pos, event.x, event.val);
        while cur_pos < n && pos != cur_pos {
            print!("{} ", ans);
            cur_pos += 1;
        }

        // entering event
        if counter[x] == 0 {
            ans += 1;
        }
        counter[x] += val;
        // delete the target
        if counter[x] == 0 {
            ans -= 1;
        }
    }
    println!("");
}
