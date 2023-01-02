use std::cmp::Ordering;

use day24::get_smallest_combinations;

fn main() {
  let weights: Vec<u64> = global::read_lines().iter().map(|l| l.parse::<u64>().unwrap()).collect();
  let objective = weights.iter().sum::<u64>() / 3;
  let mut smallest = get_smallest_combinations(&weights, objective).unwrap();

  smallest.sort_by(|a, b| -> Ordering {
    a.iter().product::<u64>().cmp(&b.iter().product())
  });

  for selected in smallest {
    let remaining = weights.iter().filter(|w| !selected.contains(w)).cloned().collect::<Vec<u64>>();

    if get_smallest_combinations(&remaining, objective).is_some() {
      println!("{}", selected.iter().product::<u64>());
      break;
    }
  }
}