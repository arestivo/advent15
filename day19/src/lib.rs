use std::collections::HashSet;

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug, Clone)]
#[display("{from} => {to}")]
pub struct Instruction {
  pub from: String,
  pub to: String
}

pub fn lines_to_instruction(lines: &[String]) -> Vec<Instruction> {
  lines
    .iter()
    .map(|l| l.parse::<Instruction>())
    .map(Result::unwrap)
    .collect()
}

pub fn apply_all(instructions: &[Instruction], molecule: String) -> HashSet<String> {
  let mut different = HashSet::new();

  for instruction in instructions {
    different.extend(apply_one(instruction, molecule.clone()).iter().cloned());
  }

  different
}

pub fn apply_one(instruction: &Instruction, molecule: String) -> HashSet<String> {
  let mut results = HashSet::new();
  let indices = molecule.match_indices(&instruction.from);
  
  for (i, _) in indices {
    let mut next = molecule.clone();
    next.replace_range(i..i+instruction.from.len(), &instruction.to);
    results.insert(next);
  }

  results
}