use std::collections::HashMap;

use day7::{Input, lines_to_wires, Wire};

fn main() {
  let lines = global::read_lines();
  let mut wires = lines_to_wires(&lines);

  let a = value("a", &mut wires);

  let mut wires = lines_to_wires(&lines);
  let b = wires.get("b").unwrap().clone();
  wires.insert("b".to_string(), Wire {value: Some(a), ..b});

  println!("{}", value("a", &mut wires));
}

fn value(s: &str, wires: &mut HashMap<String, Wire>) -> u16 {
  if s.chars().all(char::is_numeric) { return s.parse::<u16>().unwrap() }

  let wire = wires.get(s).unwrap().clone();

  if let Some(v) = wire.value { return v; }

  let v = match wire.input.clone() {
    Input::Value { value } => value,
    Input::Direct { input } => value(&input, wires),
    Input::And { input1, input2 } => value(&input1, wires) & value(&input2, wires),
    Input::Or { input1, input2 } => value(&input1, wires) | value(&input2, wires),
    Input::LShift { input, ammount } => value(&input, wires) << ammount,
    Input::RShift { input, ammount } => value(&input, wires) >> ammount,
    Input::Not { input } => !value(&input, wires),
  };

  wires.insert(s.to_string(), Wire {value: Some(v), ..wire});

  v
}