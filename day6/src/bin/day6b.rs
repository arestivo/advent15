use day6::{lines_to_instructions, Instruction};
use global::read_lines;

fn main() {
  let lines = read_lines();
  let instructions = lines_to_instructions(&lines);

  let mut total = 0_u64;

  for row in 0..1000 {
    for col in 0..1000 {
      total += brightness(row, col, &instructions);
    }
  }

  println!("{}", total)
}

fn brightness(row: u64, col: u64, instructions: &[Instruction]) -> u64 {
  let mut brightness = 0;
  
  for i in instructions {
    if i.range.includes(row, col) {
      match i.op {
        day6::Operation::TurnOn => brightness += 1,
        day6::Operation::TurnOff => if brightness > 0 { brightness -= 1 },
        day6::Operation::Toggle => brightness += 2,
      }
    }
  }

  brightness
}

#[cfg(test)]
mod tests {
  use day6::{Range, Light};
  use super::*;

  #[test]
  fn brightness_works() {
    let instructions = vec![
      Instruction {op: day6::Operation::TurnOn, range: Range { from: Light { row: 0, col: 0 }, to: Light { row: 4, col: 4} } },
      Instruction {op: day6::Operation::TurnOff, range: Range { from: Light { row: 0, col: 0 }, to: Light { row: 2, col: 2} } },
      Instruction {op: day6::Operation::Toggle, range: Range { from: Light { row: 2, col: 2 }, to: Light { row: 4, col: 4} } }
    ];

    assert_eq!(0, brightness(0, 0, &instructions));
    assert_eq!(2, brightness(2, 2, &instructions));
    assert_eq!(0, brightness(0, 2, &instructions));
    assert_eq!(0, brightness(2, 0, &instructions));
    assert_eq!(3, brightness(4, 4, &instructions));
    assert_eq!(1, brightness(0, 4, &instructions));
    assert_eq!(1, brightness(4, 0, &instructions));
  }
}