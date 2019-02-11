extern crate colored;
extern crate rand;

use colored::Colorize;
use rand::thread_rng;
use rand::seq::SliceRandom;

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

  fn fetch_no_number_area(&self) -> Vec<(usize, usize)> {
    let mut ans = Vec::new();

    for i in 0..4 {
      for j in 0..4 {
        if self.board_data[i][j] == None { ans.push((i, j)); }
      }
    }
    ans
  }

  fn generate_two_or_four() -> i32 {
    let choices: [i32; 4] = [2, 2, 2, 4];
    let mut rng = thread_rng();

    *choices.choose(&mut rng).unwrap()
  }

  pub fn fill_in(&mut self) {
    let no_number_vec = Board::fetch_no_number_area(self);
    let mut rng = thread_rng();

    match no_number_vec.choose(&mut rng) {
      Some(tuple) => {
        let (x, y) = *tuple;
        self.board_data[x][y] = Some(Board::generate_two_or_four());
      }
      None => {}
    }
  }

  pub fn is_continuable(&self) -> bool {
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

  pub fn move_up(&mut self) {
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

  pub fn move_down(&mut self) {
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
  
  pub fn move_left(&mut self) {
    for i in 0..4 {
      if self.board_data[i][0] == self.board_data[i][1] 
      && self.board_data[i][2] == self.board_data[i][3] 
      && self.board_data[i][0] != None
      && self.board_data[i][2] != None
      {
        self.board_data[i][0] = self.board_data[i][0].map(|v| v*2);
        self.board_data[i][1] = self.board_data[i][2].map(|v| v*2);
        self.board_data[i][2] = None;
        self.board_data[i][3] = None;

        self.score += self.board_data[i][0].unwrap() + self.board_data[i][1].unwrap();
      }
      else
      {
        let mut is_renewed = false;
        for j in 0..4 {
          for k in (0..j).rev() {
            match self.board_data[i][k+1] {
              None => {}
              Some(y) => {
                match self.board_data[i][k] {
                  None => {
                    self.board_data[i][k] = Some(y);
                    self.board_data[i][k+1] = None;
                  }
                  Some(x) => {
                    if x == y && !is_renewed {
                      self.board_data[i][k] = Some(2 * y);
                      self.board_data[i][k+1] = None;
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

  pub fn move_right(&mut self) {
    for i in 0..4 {
      if self.board_data[i][0] == self.board_data[i][1] 
      && self.board_data[i][2] == self.board_data[i][3] 
      && self.board_data[i][0] != None
      && self.board_data[i][2] != None
      {
        self.board_data[i][2] = self.board_data[i][1].map(|v| v*2);
        self.board_data[i][3] = self.board_data[i][3].map(|v| v*2);
        self.board_data[i][0] = None;
        self.board_data[i][1] = None;

        self.score += self.board_data[i][2].unwrap() + self.board_data[i][3].unwrap();
      }
      else
      {
        let mut is_renewed = false;
        for j in (0..4).rev() {
          for k in j..3 {
            match self.board_data[i][k] {
              None => {}
              Some(y) => {
                match self.board_data[i][k+1] {
                  None => {
                    self.board_data[i][k+1] = Some(y);
                    self.board_data[i][k] = None;
                  }
                  Some(x) => {
                    if x == y && !is_renewed {
                      self.board_data[i][k+1] = Some(2 * y);
                      self.board_data[i][k] = None;
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

  fn is_movable_row(row: [Option<i32>; 4]) -> bool {
    match row {
      [_      , None   , None,    None   ] => false,
      [Some(a), Some(b), None,    None   ] 
        if a != b                          => false,
      [Some(a), Some(b), Some(c), None   ] 
        if a != b && b != c                => false,
      [Some(a), Some(b), Some(c), Some(d)] 
        if a != b && b != c && c != d      => false,
      _                                    => true
    }
  }

  pub fn is_up_movable(&self) -> bool {
    let mut ans = false;

    for j in 0..4 {
      let row = [
        self.board_data[0][j],
        self.board_data[1][j],
        self.board_data[2][j],
        self.board_data[3][j]
      ];
      if Board::is_movable_row(row) {
        ans = true;
        break;
      }
    }
    ans
  }

  pub fn is_down_movable(&self) -> bool {
    let mut ans = false;

    for j in 0..4 {
      let row = [
        self.board_data[3][j],
        self.board_data[2][j],
        self.board_data[1][j],
        self.board_data[0][j]
      ];
      if Board::is_movable_row(row) {
        ans = true;
        break;
      }
    }
    ans
  }

  pub fn is_left_movable(&self) -> bool {
    let mut ans = false;

    for i in 0..4 {
      let row = [
        self.board_data[i][0],
        self.board_data[i][1],
        self.board_data[i][2],
        self.board_data[i][3]
      ];
      if Board::is_movable_row(row) {
        ans = true;
        break;
      }
    }
    ans
  }

  pub fn is_right_movable(&self) -> bool {
    let mut ans = false;

    for i in 0..4 {
      let row = [
        self.board_data[i][3],
        self.board_data[i][2],
        self.board_data[i][1],
        self.board_data[i][0]
      ];
      if Board::is_movable_row(row) {
        ans = true;
        break;
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

#[test]
fn check_move_left() {
  let mut test_board = Board::initialize();
  test_board.score = 8;
  test_board.board_data = [
    [None, None, None, None],
    [Some(2), Some(2), None, None],
    [Some(4), Some(4), None, None],
    [Some(2), Some(2), Some(4), Some(2)]
  ];
  test_board.move_left();
  assert_eq!(test_board.score, 24);
  assert_eq!(test_board.board_data, [
    [None, None, None, None], 
    [Some(4), None, None, None], 
    [Some(8), None, None, None], 
    [Some(4), Some(4), Some(2), None]
  ]);

  test_board.score = 60;
  test_board.board_data = [
    [Some(2), Some(2), None, None],
    [Some(2), Some(16), Some(2), None],
    [Some(4), Some(8), Some(4), Some(4)],
    [Some(2), Some(2), Some(2), Some(2)]
  ];
  test_board.move_left();
  assert_eq!(test_board.score, 80);
  assert_eq!(test_board.board_data, [
    [Some(4), None, None, None], 
    [Some(2), Some(16), Some(2), None], 
    [Some(4), Some(8), Some(8), None], 
    [Some(4), Some(4), None, None]
  ]);

  test_board.score = 276;
  test_board.board_data = [
    [Some(16), Some(4), Some(2), Some(4)],
    [Some(2), Some(32), Some(8), Some(8)],
    [Some(8), Some(8), Some(16), Some(2)],
    [None, Some(4), Some(2), None]
  ];
  test_board.move_left();
  assert_eq!(test_board.score, 308);
  assert_eq!(test_board.board_data, [
    [Some(16), Some(4), Some(2), Some(4)], 
    [Some(2), Some(32), Some(16), None], 
    [Some(16), Some(16), Some(2), None], 
    [Some(4), Some(2), None, None]
  ]);
}

#[test]
fn check_move_right() {
  let mut test_board = Board::initialize();
  test_board.score = 72;
  test_board.board_data = [
    [Some(8), Some(8), Some(2), Some(2)],
    [Some(4), Some(4), None, None],
    [Some(16), None, None, None],
    [None, None, Some(2), None]
  ];
  test_board.move_right();
  assert_eq!(test_board.score, 100);
  assert_eq!(test_board.board_data, [
    [None, None, Some(16), Some(4)], 
    [None, None, None, Some(8)], 
    [None, None, None, Some(16)], 
    [None, None, None, Some(2)]
  ]);

  test_board.score = 252;
  test_board.board_data = [
    [Some(2), Some(8), Some(4), Some(4)],
    [Some(8), Some(16), Some(16), Some(2)],
    [Some(32), Some(4), Some(4), Some(4)],
    [Some(2), Some(2), Some(2), Some(2)]
  ];
  test_board.move_right();
  assert_eq!(test_board.score, 308);
  assert_eq!(test_board.board_data, [
    [None, Some(2), Some(8), Some(8)], 
    [None, Some(8), Some(32), Some(2)], 
    [None, Some(32), Some(4), Some(8)], 
    [None, None, Some(4), Some(4)]
  ]);

  test_board.score = 640;
  test_board.board_data = [
    [Some(4), Some(4), Some(2), Some(2)],
    [Some(16), Some(16), Some(8), Some(8)],
    [Some(4), Some(64), Some(32), Some(2)],
    [Some(2), Some(16), Some(16), Some(4)]
  ];
  test_board.move_right();
  assert_eq!(test_board.score, 732);
  assert_eq!(test_board.board_data, [
    [None, None, Some(8), Some(4)], 
    [None, None, Some(32), Some(16)], 
    [Some(4), Some(64), Some(32), Some(2)],
    [None, Some(2), Some(32), Some(4)]
  ]);
}

#[test]
fn check_no_number_area() {
  let mut test_board = Board::initialize();

  test_board.board_data = [
    [None, None, None, None],
    [Some(2), Some(2), None, None],
    [None, Some(4), None, None],
    [Some(2), None, Some(4), Some(2)]
  ];
  assert_eq!(test_board.fetch_no_number_area(),
    vec![
      (0, 0), (0, 1), (0, 2), (0, 3),
      (1, 2), (1, 3), (2, 0), (2, 2),
      (2, 3), (3, 1)
    ]
  );
}

#[test]
fn check_movable_up() {
  let mut test_board = Board::initialize();

  test_board.board_data = [
    [Some(2), Some(8), Some(16), Some(4)],
    [None, Some(4), Some(8), Some(2)],
    [None, None, Some(2), Some(8)],
    [None, None, None, Some(2)]
  ];
  assert_eq!(test_board.is_up_movable(), false);

  test_board.board_data = [
    [Some(2), Some(8), Some(16), Some(2)],
    [None, Some(4), Some(8), Some(2)],
    [None, None, Some(2), Some(8)],
    [None, None, None, Some(2)]
  ];
  assert_eq!(test_board.is_up_movable(), true);

  test_board.board_data = [
    [Some(2), Some(8), Some(16), None],
    [None, Some(4), Some(8), Some(2)],
    [None, None, Some(2), Some(8)],
    [None, None, None, Some(2)]
  ];
  assert_eq!(test_board.is_up_movable(), true);
}

#[test]
fn check_movable_down() {
  let mut test_board = Board::initialize();

  test_board.board_data = [
    [None, None, None, Some(2)],
    [None, None, None, Some(4)],
    [None, None, Some(2), Some(16)],
    [None, Some(4), Some(16), Some(32)]
  ];
  assert_eq!(test_board.is_down_movable(), false);

  test_board.board_data = [
    [None, None, None, Some(2)],
    [None, None, None, Some(4)],
    [None, None, Some(2), Some(4)],
    [None, Some(4), Some(16), Some(32)]
  ];
  assert_eq!(test_board.is_down_movable(), true);

  test_board.board_data = [
    [None, None, None, Some(2)],
    [None, None, None, Some(4)],
    [None, None, Some(2), None],
    [None, Some(4), Some(16), Some(32)]
  ];
  assert_eq!(test_board.is_down_movable(), true);
}

#[test]
fn check_movable_left() {
  let mut test_board = Board::initialize();

  test_board.board_data = [
    [Some(8), None, None, None],
    [Some(4), Some(2), None, None],
    [Some(16), Some(4), Some(2), None],
    [Some(64), Some(16), Some(4), Some(2)]
  ];
  assert_eq!(test_board.is_left_movable(), false);

  test_board.board_data = [
    [Some(8), None, None, None],
    [Some(4), Some(2), None, None],
    [Some(16), Some(16), Some(2), None],
    [Some(64), Some(16), Some(4), Some(2)]
  ];
  assert_eq!(test_board.is_left_movable(), true);

  test_board.board_data = [
    [Some(8), None, None, None],
    [Some(4), Some(2), None, None],
    [Some(16), None, Some(2), None],
    [Some(64), Some(16), Some(4), Some(2)]
  ];
  assert_eq!(test_board.is_left_movable(), true);
}

#[test]
fn check_movable_right() {
  let mut test_board = Board::initialize();

  test_board.board_data = [
    [None, None, None, Some(4)],
    [None, None, Some(4), Some(8)],
    [None, Some(4), Some(8), Some(4)],
    [Some(2), Some(8), Some(16), Some(128)]
  ];
  assert_eq!(test_board.is_right_movable(), false);

  test_board.board_data = [
    [None, None, None, Some(4)],
    [None, None, Some(4), Some(8)],
    [None, Some(4), Some(8), Some(4)],
    [Some(2), Some(8), Some(16), Some(16)]
  ];
  assert_eq!(test_board.is_right_movable(), true);

  test_board.board_data = [
    [None, None, None, Some(4)],
    [None, None, Some(4), Some(8)],
    [None, Some(4), Some(8), Some(4)],
    [Some(2), Some(8), Some(16), None]
  ];
  assert_eq!(test_board.is_right_movable(), true);
}