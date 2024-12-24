use libmancala::Board;

use libmancala::ID;

use libmancala::StartPit;

fn main() {
    let mut board = Board::new();
    println!("{}", board);

    board.turn_avalanche(&ID::One, &StartPit::Mine);
    println!("{}", board);
}
