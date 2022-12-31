use day19::{lines_to_instruction, apply_all};

fn main() {
  let lines = global::read_lines();
  let instructions = lines_to_instruction(&lines[0..lines.len() - 2]);
  let molecule = lines[lines.len() - 1].to_owned();

  println!("{}", apply_all(&instructions, molecule).len());
}