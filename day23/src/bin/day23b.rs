use day23::{lines_to_code, CPU};

fn main() {
  let lines = global::read_lines();
  let code = lines_to_code(&lines);
  let mut cpu = CPU { pc: 0, register_a: 1, register_b: 0, code };

  cpu.execute();

  println!("{:?}", cpu.register_b);
}