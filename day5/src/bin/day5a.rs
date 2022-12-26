use fancy_regex::Regex;
use global::read_lines;

fn main() {
  let lines = read_lines();
  println!("{}", lines.iter().filter(|s| is_nice(s)).count());
}

fn is_nice(s: &str) -> bool {
  let re1 = Regex::new(r"(.*[aeiou]){3}.*").unwrap();
  let re2 = Regex::new(r".*(.)\1.*").unwrap();
  let re3 = Regex::new(r".*ab|cd|pq|xy.*").unwrap();

  re1.is_match(s).unwrap() && re2.is_match(s).unwrap() && !re3.is_match(s).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_nice_works() {
    assert!(is_nice("ugknbfddgicrmopn"));
    assert!(is_nice("aaa"));
    assert!(!is_nice("jchzalrnumimnmhp"));
    assert!(!is_nice("haegwjzuvuyypxyu"));
    assert!(!is_nice("dvszwmarrgswjxmb"));
  }
}