use std::collections::HashSet;

use divisors::get_divisors;

fn main() {
  let objective = global::read_line().parse::<u64>().unwrap();

  for i in 1..=100000000 {
    let p = presents(i);
    if p >= objective { println!("{}", i); break; }
  }
}

fn presents(house: u64) -> u64 {
  let factors = get_divisors(house);

  let mut set: HashSet<u64> = HashSet::from_iter(factors.iter().cloned());
  set.insert(1);
  set.insert(house);

  set.iter().sum::<u64>() * 10
}