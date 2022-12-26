use day6::{lines_to_instructions, Instruction};
use global::read_lines;

fn main() {
  let lines = read_lines();
  let instructions = lines_to_instructions(&lines);

  let mut count = 0_u64;

  for row in 0..1000 {
    for col in 0..1000 {
      if is_on(row, col, &instructions) { count += 1 };
    }
  }

  println!("{}", count)
}

fn is_on(row: u64, col: u64, instructions: &[Instruction]) -> bool {
  let mut is_on = false;

  for i in instructions {
    if i.range.includes(row, col) {
      match i.op {
        day6::Operation::TurnOn => is_on = true,
        day6::Operation::TurnOff => is_on = false,
        day6::Operation::Toggle => is_on = !is_on,
      }
    }
  }

  is_on
}

#[cfg(test)]
mod tests {
  use day6::{Range, Light};
  use super::*;

  #[test]
  fn is_on_works() {
    let instructions = vec![
      Instruction {op: day6::Operation::TurnOn, range: Range { from: Light { row: 0, col: 0 }, to: Light { row: 4, col: 4} } },
      Instruction {op: day6::Operation::TurnOff, range: Range { from: Light { row: 0, col: 0 }, to: Light { row: 2, col: 2} } },
      Instruction {op: day6::Operation::Toggle, range: Range { from: Light { row: 2, col: 2 }, to: Light { row: 4, col: 4} } }
    ];

    assert!(!is_on(0, 0, &instructions));
    assert!(is_on(2, 2, &instructions));
    assert!(!is_on(0, 2, &instructions));
    assert!(!is_on(2, 0, &instructions));
    assert!(!is_on(4, 4, &instructions));
    assert!(is_on(0, 4, &instructions));
    assert!(is_on(4, 0, &instructions));
  }
}