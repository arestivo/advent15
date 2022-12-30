use std::collections::HashMap;

fn main() {
  let numbers: Vec<u32> = global::read_lines().iter().map(|s| s.parse().unwrap()).collect();
  let mut stats: HashMap<usize, u32> = HashMap::new();

  dfs(&numbers, Vec::new(), 0, &mut stats);

  let min = stats.keys().min().unwrap();
  println!("{:?}", stats.get(min).unwrap());
}

fn dfs(numbers: &Vec<u32>, visited: Vec<usize>, eggnog: u32, stats: &mut HashMap<usize, u32>) {
  let limit = if numbers.len() > 5 { 150 } else { 25 };

  if eggnog > limit { return }
  if eggnog == limit { 
    let count = stats.get(&visited.len()).unwrap_or(&0);
    stats.insert(visited.len(), count + 1);
  }

  if visited.len() == numbers.len() { return }

  let last = visited.last().unwrap_or(&0);

  for i in *last..numbers.len() {
    if visited.contains(&i) { continue; }
    let mut visited = visited.clone();
    visited.push(i);
    dfs(numbers, visited, eggnog + numbers[i], stats);
  }
}