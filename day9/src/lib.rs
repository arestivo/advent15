use std::{collections::{HashSet, HashMap}, cmp::max};

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug, Clone)]
#[display("{city1} to {city2} = {distance}")]
pub struct Distance {
  pub city1: String,
  pub city2: String,
  pub distance: i64
}

pub fn lines_to_distances(lines: &[String]) -> Vec<Distance> {
  lines
    .iter()
    .map(|s| s.parse())
    .map(Result::unwrap)
    .collect()
}

pub fn bfs(cities: &HashSet<String>, distances: &HashMap<String, Vec<Distance>>) -> i64 {
  let mut pq: Vec<(Vec<String>, i64)> = vec![];
  for initial in cities { pq.push((vec![initial.to_owned()], 0)); }

  let mut best = i64::MIN;

  while !pq.is_empty() {
    let (path, distance) = pq.pop().unwrap();
    let city = path.last().unwrap();

    if path.len() == cities.len() { best = max(best, distance); }

    let edges = distances.get(path.last().unwrap()).unwrap();

    for e in edges {
      let next = if city == &e.city1 { e.city2.clone() } else { e.city1.clone() };
      if !path.contains(&next) {
        let path: Vec<String> = path.iter().chain([&next]).cloned().collect();
        pq.push((path, distance + e.distance));  
      }
    }
  }

  best
}