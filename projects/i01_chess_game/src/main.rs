mod board;

use crate::board::Board;

fn main() {
    let mut board = Board::new();
    board.init_figures();
    print!("{}", board);
}
