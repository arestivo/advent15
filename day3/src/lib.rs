
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Position { pub row: i64, pub col: i64 }

impl Position {
  pub fn go(&self, m: char) -> Position {
    match m {
      '^' => Position { row: self.row - 1, col: self.col },
      'v' => Position { row: self.row + 1, col: self.col },
      '<' => Position { row: self.row, col: self.col - 1 },
      '>' => Position { row: self.row, col: self.col + 1 },
      _ => unreachable!()
    }
  }
}