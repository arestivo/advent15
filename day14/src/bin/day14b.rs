use day14::{lines_to_raindeers, State};

fn main () {
  let lines = global::read_lines();
  let mut raindeers = lines_to_raindeers(&lines);
  let names = Vec::from_iter(raindeers.keys().clone().cloned());

  for _ in 1..=2503 {
    for name in &names {
      let mut rd = raindeers.get(name).unwrap().clone();
      match rd.state {
        day14::State::Flying(spent) => {
          rd.distance += rd.speed;
          if spent + 1 > rd.time_fly { unreachable!("Missed Event!!! {:?}", rd) }
          if spent + 1 == rd.time_fly { rd.state = State::Resting(0) }
          if spent + 1 < rd.time_fly { rd.state = State::Flying(spent + 1) }
        },
        day14::State::Resting(spent) => {
          if spent + 1 > rd.time_rest { unreachable!("Missed Event!!! {:?}", rd) }
          if spent + 1 == rd.time_rest { rd.state = State::Flying(0) }
          if spent + 1 < rd.time_rest { rd.state = State::Resting(spent + 1) }
        },
      }
      raindeers.insert(rd.name.clone(), rd);
    }

    let max = raindeers.values().map(|rd| rd.distance).max().unwrap();
    for name in &names {
      let mut rd = raindeers.get(name).unwrap().clone();
      if rd.distance == max { rd.points += 1; }
      raindeers.insert(rd.name.clone(), rd);
    }
  }

  println!("{:?}", raindeers.values().map(|rd| rd.points).max().unwrap());
}