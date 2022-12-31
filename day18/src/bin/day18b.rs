fn main() {
  let mut map: Vec<Vec<char>> = global::read_lines().iter().map(|s| s.chars().collect()).collect();

  let height = map.len();
  let width = map[0].len();

  map[0][0] = '#';
  map[0][width - 1] = '#';
  map[height - 1][0] = '#';
  map[height - 1][width - 1] = '#';

  for _ in 0..100 {
    map = animate(map);

    map[0][0] = '#';
    map[0][width - 1] = '#';
    map[height - 1][0] = '#';
    map[height - 1][width - 1] = '#';
  }

  let mut count = 0;

  for row in map {
    for c in row {
      if c == '#' { count += 1 }
    }
  }

  println!("{:?}", count);
}

fn animate(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
  let mut next = map.clone();

  for row in 0..next.len() {
    for col in 0..next[row].len() {
      let count = count_neighbours_on(&map, row as i32, col as i32);

      next[row][col] = match map[row][col] {
        '#' => if count == 2 || count == 3 {'#'} else {'.'},
        '.' => if count == 3 {'#'} else {'.'},
        _ => unreachable!()
      };
    }
  }

  next
}

fn count_neighbours_on(map: &[Vec<char>], row: i32, col: i32) -> u32 {
  let mut count = 0;

  for dr in -1..=1 {
    for dc in -1..=1 {
      if row + dr < 0 || col + dc < 0 { continue }
      if row + dr >= map.len() as i32 || col + dc >= map[0].len() as i32 { continue }
      if (dr != 0 || dc != 0) && map[(row + dr) as usize][(col + dc) as usize] == '#' { count += 1 }
    }      
  }

  count
}