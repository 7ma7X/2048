extern crate colored;

use colored::Colorize;

pub struct Board {
  score: i32,
  board_data: [[Option<i32>; 4]; 4]
}

impl Board {
  pub fn initialize() -> Board {
    Board { score: 0, board_data: [[None; 4]; 4]}
  }

  pub fn display(&self) {
    println!("{} {}", "score:".yellow(), self.score.to_string().yellow());

    for i in 0..4 {
      for j in 0..4 {
        match self.board_data[i][j] {
          Some(x) => print!("{:>width$}", x, width=6),
          None => print!("     -")
        }
      }
      println!("");
    }
  }

  fn is_continuable(&self) -> bool {
    let mut ans = false;

    for i in 0..3 {
      for j in 0..4 {
        match self.board_data[i][j] {
          Some(x) => {
            match self.board_data[i+1][j] {
              Some(y) => if x == y { ans = true; break; },
              None => { ans = true; break; }
            }
          }
          None => { ans = true; break; }
        }

        match self.board_data[j][i] {
          Some(x) => {
            match self.board_data[j][i+1] {
              Some(y) => if x == y { ans = true; break; },
              None => {}
            }
          }
          None => {}          
        }
      }
    }

    ans
  }
}


#[test]
fn check_continuability() {
  let mut test_board = Board::initialize();
  assert_eq!(test_board.is_continuable(), true);
  test_board.board_data = [
    [Some(4), Some(2), Some(4), Some(2)],
    [Some(8), Some(4), Some(8), Some(4)],
    [Some(4), Some(8), Some(4), Some(8)],
    [Some(2), Some(4), Some(2), Some(4)]
  ];
  assert_eq!(test_board.is_continuable(), false);
    test_board.board_data = [
    [Some(4), Some(2), Some(4), Some(2)],
    [Some(8), Some(4), Some(8), Some(4)],
    [Some(4), Some(8), Some(4), Some(8)],
    [Some(2), Some(4), Some(2), Some(2)]
  ];
  assert_eq!(test_board.is_continuable(), true);
    test_board.board_data = [
    [Some(4), Some(2), Some(4), Some(2)],
    [Some(8), Some(4), Some(8), Some(4)],
    [Some(4), Some(8), Some(4), Some(8)],
    [Some(2), Some(4), Some(2), None]
  ];
  assert_eq!(test_board.is_continuable(), true);
}
