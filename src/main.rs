extern crate colored;
extern crate rand;

mod board;

use board::Board;
use std::io::stdin;

fn main() {
  let mut board = Board::initialize();
  board.fill_in();
  board.fill_in();
  board.display();

  while board.is_continuable() {
    let mut input = String::new();
    match stdin().read_line(&mut input) {
      Ok(_) => {
        if input.starts_with("i") || input.starts_with("I") {
          if board.is_up_movable() {
            board.move_up();
            board.fill_in();
            board.display();
          }
        } else if input.starts_with("m") || input.starts_with("M") {
          if board.is_down_movable() {
            board.move_down();
            board.fill_in();
            board.display();
          }
        } else if input.starts_with("j") || input.starts_with("J") {
          if board.is_left_movable() {
            board.move_left();
            board.fill_in();
            board.display();
          }
        } else if input.starts_with("k") || input.starts_with("K") {
          if board.is_right_movable() {
            board.move_right();
            board.fill_in();
            board.display();
          }
        } else {
          println!("error: {}", "invalid input");
        }
      }
      Err(error) => println!("error: {}", error),
    }
  }
                          
  println!("{}", "GAME OVER");
}
