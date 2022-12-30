fn main() {
  let numbers: Vec<u32> = global::read_lines().iter().map(|s| s.parse().unwrap()).collect();

  println!("{:?}", dfs(&numbers, Vec::new(), 0));
}

fn dfs(numbers: &Vec<u32>, visited: Vec<usize>, eggnog: u32) -> u32 {
  let mut count = 0_u32;
  let limit = if numbers.len() > 5 { 150 } else { 25 };

  if eggnog > limit { return 0 }
  if eggnog == limit { return 1 }

  if visited.len() == numbers.len() { return 0 }

  let last = visited.last().unwrap_or(&0);

  for i in *last..numbers.len() {
    if visited.contains(&i) { continue; }
    let mut visited = visited.clone();
    visited.push(i);
    count += dfs(numbers, visited, eggnog + numbers[i]);
  }

  count
}