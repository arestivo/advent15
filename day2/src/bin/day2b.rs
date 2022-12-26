use day2::{Present, present_to_perimeters, present_to_volume};

fn main() {
  let lines = global::read_lines();
  let papers = lines.iter().map(|l| line_to_ribbon(l));

  println!("{}", papers.sum::<u64>());
}

fn line_to_ribbon(l: &str) -> u64 {
  let p = l.parse::<Present>().unwrap();
  *present_to_perimeters(&p).iter().min().unwrap() + present_to_volume(&p)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn present_to_paper_works() {
    assert_eq!(34, line_to_ribbon("2x3x4"));
    assert_eq!(14, line_to_ribbon("1x1x10"));
  }
}