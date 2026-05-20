use proconio::input;
const N1: usize = 1usize.wrapping_neg();
// U, D, L, R
const D4: [(usize, usize); 4] = [(N1, 0), (1, 0), (0, N1), (0, 1)];
#[derive(Debug, Copy, Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}
struct Input {
    n_board: usize,
    m_favorite_colors: usize,
    color_num: usize,
    d_favorite_colors: Vec<usize>,
    f_initial_colors: Vec<Vec<usize>>,
}
impl Input {
    fn new() -> Self {
        input! {n_board: usize, m_favorite_colors: usize, color_num: usize, d_favorite_colors: [usize; m_favorite_colors], f_initial_colors: [[usize; n_board]; n_board]}
        Input {
            n_board,
            m_favorite_colors,
            color_num,
            d_favorite_colors,
            f_initial_colors,
        }
    }
}
fn main() {
    let input = Input::new();
    let moves: Vec<Move> = get_moves(&input);
    display(&moves);
}

fn get_moves(input: &Input) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    for _ in 4..input.n_board - 1 {
        moves.push(Move::Down);
    }
    for col in 1..input.n_board {
        moves.push(Move::Right);
        if col % 2 == 1 {
            for _ in 0..input.n_board - 1 {
                moves.push(Move::Up);
            }
        } else {
            for _ in 0..input.n_board - 1 {
                moves.push(Move::Down);
            }
        }
    }
    return moves;
}

fn display(moves: &Vec<Move>) {
    let t: usize = moves.len();
    for i in 0..t {
        match moves[i] {
            Move::Up => println!("U"),
            Move::Down => println!("D"),
            Move::Left => println!("L"),
            Move::Right => println!("R"),
        }
    }
}
