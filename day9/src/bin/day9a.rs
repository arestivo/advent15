use std::collections::{HashSet, HashMap};

use day9::{lines_to_distances, Distance, bfs};

fn main() {
  let lines = global::read_lines();
  let distances = lines_to_distances(&lines);

  let distances: Vec<Distance> = distances.iter().map(|d| Distance { distance: -d.distance, ..d.clone() }).collect();

  let cities = HashSet::from_iter(
    distances.iter().map(|d| d.city1.clone()).chain(
    distances.iter().map(|d| d.city2.clone()))
  );

  let distances = cities.iter()
    .map(|c| (c.to_owned(), distances.iter().filter(|d| &d.city1 == c || &d.city2 == c).cloned().collect::<Vec<Distance>>()))
    .collect::<HashMap<_, _>>();

  println!("{:?}", -bfs(&cities, &distances));
}