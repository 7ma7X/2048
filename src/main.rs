extern crate colored;
extern crate rand;

mod board;

use board::{Board};

fn main() {
  let mut current_board = Board::initialize();
  current_board.fill_in_board();
  current_board.fill_in_board();
  current_board.display();

  // let num = Board::generate_next_number();
  // println!("{}", num);
}
