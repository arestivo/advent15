fn main() {
  let line = global::read_line();
  println!("{:?}", calculate_floor(&line));
}

fn calculate_floor(line: &str) -> i64 {
  line.chars().fold(0_i64, |f, p| f + match p { '(' => 1, _ => -1 } )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn calculate_floor_works() {
    assert_eq!(0, calculate_floor("(())"));
    assert_eq!(0, calculate_floor("()()"));
    assert_eq!(3, calculate_floor("((("));
    assert_eq!(3, calculate_floor("(()(()("));
    assert_eq!(3, calculate_floor("))((((("));
    assert_eq!(-1, calculate_floor("())"));
    assert_eq!(-1, calculate_floor("))("));
    assert_eq!(-1, calculate_floor("))("));
    assert_eq!(-3, calculate_floor(")))"));
    assert_eq!(-3, calculate_floor(")())())"));  
  }
}