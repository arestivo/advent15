use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug, Clone)]
pub enum Operation {
  #[display("turn_on")]
  TurnOn,
  #[display("turn_off")]
  TurnOff,
  #[display("toggle")]
  Toggle
}

#[derive(Display, FromStr, Debug, Clone)]
#[display("{op} {range}")]
pub struct Instruction { 
  pub op: Operation,
  pub range: Range
}

#[derive(Display, FromStr, Debug, Clone)]
#[display("{row},{col}")]
pub struct Light {
  pub row: u64, pub col: u64
}

#[derive(Display, FromStr, Debug, Clone)]
#[display("{from} through {to}")]
pub struct Range {
  pub from: Light, pub to: Light
}

impl Range {
  pub fn includes(&self, row: u64, col: u64) -> bool {
    row >= self.from.row &&
    row <= self.to.row &&
    col >= self.from.col &&
    col <= self.to.col
  }
}

pub fn lines_to_instructions(lines: &[String]) -> Vec<Instruction> {
  lines
    .iter()
    .map(|s| s.replace("turn ", "turn_"))
    .map(|s| s.parse())
    .map(Result::unwrap)
    .collect()
}