use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug, Clone)]
pub enum Instruction {
  #[display("hlf {0}")]
  Hlf(char),

  #[display("tpl {0}")]
  Tpl(char),

  #[display("inc {0}")]
  Inc(char),

  #[display("jmp {0}")]
  Jmp(isize),

  #[display("jie {0}, {1}")]
  JIE(char, isize),

  #[display("jio {0}, {1}")]
  JIO(char, isize),
}

#[derive(Debug, Clone)]
pub struct CPU {
  pub pc: usize,
  pub register_a: u32,
  pub register_b: u32,
  pub code: Vec<Instruction>
}

pub fn lines_to_code(lines: &[String]) -> Vec<Instruction> {
  lines
    .iter()
    .map(|l| l.parse())
    .map(Result::unwrap)
    .collect()
}

impl CPU {
  pub fn execute(&mut self) {
    while self.pc < self.code.len() {
      let instruction = &self.code[self.pc];
      match instruction {
        Instruction::Hlf(r) => { if r == &'a' { self.register_a >>= 1 } else if r == &'b' { self.register_b >>= 1 }; self.pc += 1 },
        Instruction::Tpl(r) => { if r == &'a' { self.register_a *= 3  } else if r == &'b' { self.register_b *= 3  }; self.pc += 1 },
        Instruction::Inc(r) => { if r == &'a' { self.register_a += 1  } else if r == &'b' { self.register_b += 1  }; self.pc += 1 },
        Instruction::Jmp(d) => self.pc = (self.pc as isize + d) as usize,
        Instruction::JIE(r, d) => if (r == &'a' && self.register_a % 2 == 0) || (r == &'b' && self.register_b % 2 == 0) { self.pc = (self.pc as isize + d) as usize} else { self.pc += 1 } ,
        Instruction::JIO(r, d) => if (r == &'a' && self.register_a == 1) || (r == &'b' && self.register_b == 1) { self.pc = (self.pc as isize + d) as usize} else { self.pc += 1 } ,
      }
    }
  }
}