use std::collections::HashSet;

use day3::Position;

fn main() {
  let line = global::read_line();

  println!("{}", visited(&line));
}

fn visited(line: &str) -> usize {
  let mut pos = Position { row: 0, col: 0 };
  let mut visited = HashSet::new();

  visited.insert(pos.clone());

  for c in line.chars() {
    pos = pos.go(c);
    visited.insert(pos.clone());
  }

  visited.len()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn visited_works() {
    assert_eq!(4, visited("^>v<"));
    assert_eq!(2, visited("^v^v^v^v^v"));
  }
}