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

  fn move_up(&mut self) {
    for j in 0..4 {
      // pattern [a, a, b, b] => [2a, 2b, -, -]
      // only this pattern can board be renewed twice, otherwise once
      if self.board_data[0][j] == self.board_data[1][j] 
      && self.board_data[2][j] == self.board_data[3][j] 
      && self.board_data[0][j] != None
      && self.board_data[2][j] != None
      {
        self.board_data[0][j] = self.board_data[0][j].map(|v| v*2);
        self.board_data[1][j] = self.board_data[2][j].map(|v| v*2);
        self.board_data[2][j] = None;
        self.board_data[3][j] = None;

        self.score += self.board_data[0][j].unwrap() + self.board_data[1][j].unwrap();
      }
      else
      {
        let mut is_renewed = false;
        for i in 0..4 {
          for k in (0..i).rev() {
            match self.board_data[k+1][j] {
              None => {}
              Some(y) => {
                match self.board_data[k][j] {
                  None => {
                    self.board_data[k][j] = Some(y);
                    self.board_data[k+1][j] = None;
                  }
                  Some(x) => {
                    if x == y && !is_renewed {
                      self.board_data[k][j] = Some(2 * y);
                      self.board_data[k+1][j] = None;
                      self.score += 2 * y;
                      is_renewed = true;
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }

  fn move_down(&mut self) {
    for j in 0..4 {
      if self.board_data[0][j] == self.board_data[1][j] 
      && self.board_data[2][j] == self.board_data[3][j] 
      && self.board_data[0][j] != None
      && self.board_data[2][j] != None
      {
        self.board_data[2][j] = self.board_data[1][j].map(|v| v*2);
        self.board_data[3][j] = self.board_data[3][j].map(|v| v*2);
        self.board_data[0][j] = None;
        self.board_data[1][j] = None;

        self.score += self.board_data[2][j].unwrap() + self.board_data[3][j].unwrap();
      }
      else
      {
        let mut is_renewed = false;
        for i in (0..4).rev() {
          for k in i..3 {
            match self.board_data[k][j] {
              None => {}
              Some(y) => {
                match self.board_data[k+1][j] {
                  None => {
                    self.board_data[k+1][j] = Some(y);
                    self.board_data[k][j] = None;
                  }
                  Some(x) => {
                    if x == y && !is_renewed {
                      self.board_data[k+1][j] = Some(2 * y);
                      self.board_data[k][j] = None;
                      self.score += 2 * y;
                      is_renewed = true;
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
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

#[test]
fn check_move_up() {
  let mut test_board = Board::initialize();
  test_board.score = 92;
  test_board.board_data = [
    [Some(2), None, None, None],
    [Some(4), Some(2), None, Some(2)],
    [Some(2), Some(8), Some(4), Some(4)],
    [Some(4), Some(16), Some(8), Some(2)]
  ];
  test_board.move_up();
  assert_eq!(test_board.score, 92);
  assert_eq!(test_board.board_data, [
    [Some(2), Some(2), Some(4), Some(2)],
    [Some(4), Some(8), Some(8), Some(4)],
    [Some(2), Some(16), None, Some(2)],
    [Some(4), None, None, None]
  ]);

  test_board.score = 224;
  test_board.board_data = [
    [Some(4), Some(2), Some(4), Some(2)],
    [Some(16), Some(32), Some(8), Some(2)],
    [Some(4), Some(4), Some(8), None],
    [Some(8), Some(2), Some(4), None]
  ];
  test_board.move_up();
  assert_eq!(test_board.score, 244);
  assert_eq!(test_board.board_data, [
    [Some(4), Some(2), Some(4), Some(4)],
    [Some(16), Some(32), Some(16), None],
    [Some(4), Some(4), Some(4), None],
    [Some(8), Some(2), None, None]
  ]);

  test_board.score = 744;
  test_board.board_data = [
    [None, Some(4), Some(8), Some(4)],
    [Some(2), Some(4), Some(16), Some(2)],
    [Some(8), Some(64), Some(8), Some(2)],
    [Some(16), Some(64), Some(2), Some(4)]
  ];
  test_board.move_up();
  assert_eq!(test_board.score, 884);
  assert_eq!(test_board.board_data, [
    [Some(2), Some(8), Some(8), Some(4)],
    [Some(8), Some(128), Some(16), Some(4)],
    [Some(16), None, Some(8), Some(4)],
    [None, None, Some(2), None]
  ]);
}

#[test]
fn check_move_down() {
  let mut test_board = Board::initialize();
  test_board.score = 64;
  test_board.board_data = [
    [Some(2), Some(8), Some(4), Some(4)],
    [Some(16), Some(4), Some(2), Some(2)],
    [None, None, None, Some(4)],
    [None, Some(2), None, None]
  ];
  test_board.move_down();
  assert_eq!(test_board.score, 64);
  assert_eq!(test_board.board_data, [
    [None, None, None, None], 
    [None, Some(8), None, Some(4)], 
    [Some(2), Some(4), Some(4), Some(2)], 
    [Some(16), Some(2), Some(2), Some(4)]
  ]);

  test_board.score = 264;
  test_board.board_data = [
    [Some(4), Some(32), Some(4), None],
    [Some(2), Some(32), Some(4), Some(8)],
    [Some(2), Some(4), None, None],
    [Some(2), Some(4), None, Some(2)]
  ];
  test_board.move_down();
  assert_eq!(test_board.score, 348);
  assert_eq!(test_board.board_data, [
    [None, None, None, None], 
    [Some(4), None, None, None], 
    [Some(2), Some(64), None, Some(8)], 
    [Some(4), Some(8), Some(8), Some(2)]
  ]);

  test_board.score = 764;
  test_board.board_data = [
    [Some(2), Some(2), Some(16), Some(2)],
    [Some(2), Some(16), Some(4), Some(2)],
    [Some(64), Some(16), Some(64), Some(2)],
    [Some(2), Some(4), Some(8), Some(2)]
  ];
  test_board.move_down();
  assert_eq!(test_board.score, 808);
  assert_eq!(test_board.board_data, [
    [None, None, Some(16), None], 
    [Some(4), Some(2), Some(4), None], 
    [Some(64), Some(32), Some(64), Some(4)], 
    [Some(2), Some(4), Some(8), Some(4)]
  ]);
}