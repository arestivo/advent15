use std::{collections::{HashMap, HashSet}, cmp::max};

use day13::lines_to_pairs;

fn main () {
  let lines = global::read_lines();
  let pairs = lines_to_pairs(&lines);

  let mut map = HashMap::new();
  let mut names = HashSet::new();

  for pair in pairs {
    match pair {
        day13::Pair::Lose { name1, name2, ammount } => map.insert((name1, name2), 0 - ammount as i64),
        day13::Pair::Gain { name1, name2, ammount } => map.insert((name1, name2), ammount as i64),
    };
  }

  println!("{:?}", map);

  for pair in map.keys() { names.insert(pair.0.clone()); }

  println!("{:?}", dfs(&Vec::from_iter(names.iter().cloned()), &map, vec![]));
}

fn dfs(names: &Vec<String>, map: &HashMap<(String, String), i64>, table: Vec<String>) -> i64 {
  let mut best = i64::MIN;

  if table.len() == names.len() {
    let mut happiness = 0_i64;

    for i in 1..table.len() {
      happiness += map.get(&(table.get(i).unwrap().clone(), table.get(i - 1).unwrap().clone())).unwrap();
      happiness += map.get(&(table.get(i - 1).unwrap().clone(), table.get(i).unwrap().clone())).unwrap();
    }

    return 
      happiness +
      map.get(&(table.last().unwrap().to_string(), table.first().unwrap().to_string())).unwrap() +
      map.get(&(table.first().unwrap().to_string(), table.last().unwrap().to_string())).unwrap();
  }

  for name in names {
    if !table.contains(name) {
      let next = table.iter().chain([name]).cloned().collect();
      best = max(best, dfs(names, map, next));
    }
  }

  best
}