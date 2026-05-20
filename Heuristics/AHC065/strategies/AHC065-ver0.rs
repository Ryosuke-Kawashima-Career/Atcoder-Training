use proconio::input;
#[derive(Debug, Copy, Clone)]
struct Input {
    n: usize,
    box_ids: Vec<Vec<usize>>,
}
impl Input {
    fn new() -> Self {
        input! {n: usize, box_ids: Vec<Vec<usize>>};
        return Self { n, box_ids };
    }
}
#[derive(Debug, Clone)]
struct State {
    m: usize,
    n: usize,
    moved_boxes: usize,
    moved_boxed_ids: Vec<Vec<usize>>,
    conveyors: Vec<Vec<(usize, usize)>>,
    conveyor_lengths: Vec<usize>,
    operations: Vec<(usize, isize)>, // (conveyor_id, delta)
}
impl State {
    fn new(input: &Input) -> Self {
        return Self {
            m: 0,
            n: input.n,
            moved_boxes: 0,
            moved_boxed_ids: vec![],
            conveyors: vec![],
            conveyor_lengths: vec![],
            operations: vec![],
        };
    }
    fn set_conveyors(&mut self) {}
    fn simulate(&mut self) {}
    fn objective_function(&self) -> usize {}
    fn print_output(&self) {
        println!("{}", self.m);
        for i in 0..self.m {
            print!("{}", self.conveyor_lengths[i]);
            for j in 0..self.conveyor_lengths[i] {
                println!(" {} {}", self.conveyors[i][j].0, self.conveyors[i][j].1);
            }
        }
        let turns: usize = self.operations.len();
        println!("{}", turns);
        for i in 0..turns {
            println!("{} {}", self.operations[i].0, self.operations[i].1);
        }
    }
}

fn main() {
    let input = Input::new();
    let mut state = State::new(&input);
    state.set_conveyors();
    state.simulate();
    state.print_output();
}
