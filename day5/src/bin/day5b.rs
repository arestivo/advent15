use fancy_regex::Regex;
use global::read_lines;

fn main() {
  let lines = read_lines();
  println!("{}", lines.iter().filter(|s| is_nice(s)).count());
}

fn is_nice(s: &str) -> bool {
  let re1 = Regex::new(r".*(..).*\1.*").unwrap();
  let re2 = Regex::new(r".*(.).\1.*").unwrap();

  re1.is_match(s).unwrap() && re2.is_match(s).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_nice_works() {
    assert!(is_nice("qjhvhtzxzqqjkmpb"));
    assert!(is_nice("xxyxx"));
    assert!(!is_nice("uurcxstgmygtbstg"));
    assert!(!is_nice("ieodomkazucvgmuy"));
  }
}