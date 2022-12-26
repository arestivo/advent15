use day2::{Present, present_to_areas};

fn main() {
  let lines = global::read_lines();
  let papers = lines.iter().map(|l| line_to_paper(l));

  println!("{}", papers.sum::<u64>());
}

fn line_to_paper(l: &str) -> u64 {
  let p = l.parse::<Present>().unwrap();
  let sides = present_to_areas(&p);
  sides.iter().sum::<u64>() + sides.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn present_to_paper_works() {
    assert_eq!(58, line_to_paper("2x3x4"));
    assert_eq!(43, line_to_paper("1x1x10"));
  }
}