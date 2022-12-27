use std::collections::HashMap;

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug, Clone)]
pub enum Input {
  #[display("{value}")]
  Value{value: u16},
  #[display("{input1} AND {input2}")]
  And{input1: String, input2: String},
  #[display("{input1} OR {input2}")]
  Or{input1: String, input2: String},
  #[display("{input} LSHIFT {ammount}")]
  LShift{input: String, ammount: u16},
  #[display("{input} RSHIFT {ammount}")]
  RShift{input: String, ammount: u16},
  #[display("NOT {input}")]
  Not{input: String},
  #[display("{input}")]
  #[from_str(regex = "(?P<input>[a-z]+)")]
  Direct{input: String},
}

#[derive(Display, FromStr, Debug, Clone)]
#[display("{input} -> {output}")]
#[from_str(default)]
pub struct Wire {
  pub output: String,
  pub input: Input,
  pub value: Option<u16>
}

impl Default for Wire {
  fn default() -> Self {
    Self { value: None, output: "".to_string(), input: Input::Value{ value: 0 } }
  }
}

pub fn lines_to_wires(lines: &[String]) -> HashMap<String, Wire> {
  lines
    .iter()
    .map(|s| s.parse::<Wire>())
    .map(Result::unwrap)
    .map(|w| (w.clone().output, w))
    .collect::<HashMap<_,_>>()
}