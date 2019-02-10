extern crate colored;

mod board;

use board::{Board};

fn main() {
  let current_board = Board::initialize();
  current_board.display();
}
