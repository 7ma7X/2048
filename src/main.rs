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
      Ok(_n) => {
        if input.starts_with("i") {
          board.move_up();
          board.fill_in();
          board.display();
        } else if input.starts_with("m") {
          board.move_down();
          board.fill_in();
          board.display();
        } else if input.starts_with("j") {
          board.move_left();
          board.fill_in();
          board.display();
        } else if input.starts_with("k") {
          board.move_right();
          board.fill_in();
          board.display();
        } else {
          println!("error: {}", "invalid input");
        }
      }
      Err(error) => println!("error: {}", error),
    }
  }
                          
  println!("{}", "GAME OVER");
}
