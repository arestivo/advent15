use day14::{lines_to_raindeers, State};
use priority_queue::PriorityQueue;

fn main () {
  let lines = global::read_lines();
  let mut raindeers = lines_to_raindeers(&lines);
  let mut events = PriorityQueue::new();
  let names = Vec::from_iter(raindeers.keys().clone().cloned());

  events.push(2503, -2503);

  for rd in raindeers.values() { events.push(rd.time_fly, 0 - rd.time_fly as i32); }

  let mut last_time = 0;

  while !events.is_empty() {
    let (time, _) = events.pop().unwrap();
    let elapsed = time - last_time;

    if time > 2503 { break; }

    for name in &names {
      let mut rd = raindeers.get(name).unwrap().clone();
      match rd.state {
        day14::State::Flying(spent) => {
          rd.distance += elapsed * rd.speed;
          if spent + elapsed > rd.time_fly { unreachable!("Missed Event!!! {:?}", rd) }
          if spent + elapsed == rd.time_fly { rd.state = State::Resting(0); events.push(time + rd.time_rest, - (time as i32 + rd.time_rest as i32)); }
          if spent + elapsed < rd.time_fly { rd.state = State::Flying(spent + elapsed) }
        },
        day14::State::Resting(spent) => {
          if spent + elapsed > rd.time_rest { unreachable!("Missed Event!!! {:?}", rd) }
          if spent + elapsed == rd.time_rest { rd.state = State::Flying(0); events.push(time + rd.time_fly, - (time as i32 + rd.time_fly as i32)); }
          if spent + elapsed < rd.time_rest { rd.state = State::Resting(spent + elapsed) }
        },
      }
      raindeers.insert(rd.name.clone(), rd);
    }

    last_time = time;
  }

  println!("{:?}", raindeers.values().map(|rd| rd.distance).max().unwrap());
}