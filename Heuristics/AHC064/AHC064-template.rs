use pronio::input;
// AHC064
#[derive(Debug, Copy, Clone)]
struct Move {
    move_type: usize,
    departure_idx: usize,
    siding_idx: usize,
    k_cars: usize,
}

#[derive(Debug, Clone)]
struct State {
    r_tracks: usize,
    departures: Vec<Vec<usize>>,
    sidings: Vec<Vec<usize>>,
}

impl State {
}

#[derive(Debug, Clone)]
struct Scheduler {
    ready_departures: Vec<usize>,
    ready_sidings: Vec<usize>,
    moves_record: Vec<Vec<Move>>
}

impl Scheduler {
    fn schedule(&mut self, one_move: &Move) -> usize {
        
    }

    fn simulate(&self, one_move: &Move) -> usize {
        
    }
}

#[derive(Debug, Clone)]
struct SearchNode {
    moves: Vec<Move>,
    state: State,
    scheduler: Scheduler,
    score: i64,
}

impl SearchNode {
    fn calc_score(&self) -> i64 {
        let mut total_score: i64 = 0;
        return total_score;
    }
}

#[derive(Debug, Clone)]
struct BeamSearch {
    beams: Vec<Vec<SearchNode>>,
    width: usize,
}

impl BeamSearch {
    fn search(&mut self) {

    }

    fn get_best_moves(&self) -> Vec<Vec<Move>> {
        self.policy_moves.clone()
    }
}
const ITERATIONS: usize = 100;

fn main() {
    input{r_tracks: usize, rail_cars: [[usize; 10]; r_tracks]}
    let mut beam_searcher: BeamSearch = BeamSearch::new();
    for _iter in (0..ITERATIONS) {
        beam_searcher.search();
    }
    let best_moves: Vec<Vec<Move>> = beam_searcher.get_best_moves();
    print_moves(&best_moves);
}

fn print_moves(moves: &Vec<Vec<Move>>) {
    let turns: usize = moves.len();
    println!("{}", turns);
    for turn in 0..turns {
        let k: usize = moves[turn].len();
        println!("{}", k);
        for i in 0..k {
            let each_move: Move = moves[turn][i];
            println!("{} {} {} {}", each_move.move_type, each_move.departure_idx, each_move.siding_idx, each_move.k_cars);
        }
    }
}
