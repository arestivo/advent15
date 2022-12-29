use std::collections::HashMap;

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug, Clone)]
#[display("{name} can fly {speed} km/s for {time_fly} seconds, but then must rest for {time_rest} seconds.")]
#[from_str(default)]
pub struct RainDeer {
  pub name: String,
  pub speed: u32,
  pub time_fly: u32,
  pub time_rest: u32,
  pub state: State,
  pub distance: u32,
  pub points: u32
}

#[derive(Debug, Clone)]
pub enum State {
  Flying(u32),
  Resting(u32)
}

impl Default for RainDeer {
  fn default() -> Self {
    Self { name: "".to_string(), speed: 0, time_fly: 0, time_rest: 0, state: State::Flying(0), distance: 0, points: 0 }
  }
}

#[derive(Clone)]
pub enum Event {
  End,
  Start
}

pub fn lines_to_raindeers(lines: &[String]) -> HashMap<String, RainDeer> {
  lines
    .iter()
    .map(|l| l.parse::<RainDeer>())
    .map(Result::unwrap)
    .map(|rd| (rd.name.clone(), rd))
    .collect::<HashMap<_,_>>()
}