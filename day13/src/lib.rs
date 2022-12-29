use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug, Clone)]
pub enum Pair {
  #[display("{name1} would lose {ammount} happiness units by sitting next to {name2}.")]
  Lose {name1: String, name2: String, ammount: u64},
  #[display("{name1} would gain {ammount} happiness units by sitting next to {name2}.")]
  Gain {name1: String, name2: String, ammount: u64},
}

pub fn lines_to_pairs(lines: &[String]) -> Vec<Pair> {
  lines
    .iter()
    .map(|l| l.parse())
    .map(Result::unwrap)
    .collect()
}