pub fn get_smallest_combinations(weights: &Vec<u64>, objective: u64) -> Option<Vec<Vec<u64>>> {
  for size in 1..weights.len() {
    let possible = get_all_combinations(weights, objective, vec![], size);
    if !possible.is_empty() { return Some(possible); }
  };

  None
}

pub fn get_all_combinations(weights: &Vec<u64>, objective: u64, selected: Vec<u64>, size: usize) -> Vec<Vec<u64>> {
  let mut combinations = vec![];
  let weight = selected.iter().sum::<u64>();

  if selected.len() == size {
    if weight == objective { return vec![selected] }
  } else {
    for w in weights {
      if w > selected.last().unwrap_or(&0) && weight + w <= objective {
        let mut selected = selected.clone();
        selected.push(*w);
        combinations.extend(get_all_combinations(weights, objective, selected, size).iter().cloned());
      }
    }  
  }

  combinations
}