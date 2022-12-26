fn main() {
  let line = global::read_line();
  println!("{:?}", calculate_basement(&line));
}

fn calculate_basement(line: &str) -> usize {
  let mut floor = 0_i64;
  for (i, c) in line.chars().enumerate() {
    floor += match c { '(' => 1, _ => -1 };
    if floor == -1 { return i + 1 ;}
  }

  unreachable!()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn calculate_floor_works() {
    assert_eq!(1, calculate_basement(")"));
    assert_eq!(5, calculate_basement("()())"));
  }
}