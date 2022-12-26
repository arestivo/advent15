use std::collections::HashSet;

use day3::Position;

fn main() {
  let line = global::read_line();

  println!("{}", visited(&line));
}

fn visited(line: &str) -> usize {
  let mut pos1 = Position { row: 0, col: 0 };
  let mut pos2 = Position { row: 0, col: 0 };

  let mut visited = HashSet::new();

  visited.insert(pos1.clone());

  for i in (0..line.len()).step_by(2) {
    pos1 = pos1.go(line.chars().nth(i).unwrap());
    pos2 = pos2.go(line.chars().nth(i + 1).unwrap());
    visited.insert(pos1.clone());
    visited.insert(pos2.clone());
  }

  visited.len()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn visited_works() {
    assert_eq!(3, visited("^>v<"));
    assert_eq!(11, visited("^v^v^v^v^v"));
  }
}