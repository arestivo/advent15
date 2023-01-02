fn main() {
  let numbers: Vec<u64> = global::read_line()
    .split(&[' ','.',','])
    .map(|s| s.parse::<u64>())
    .filter_map(|r| r.ok())
    .collect();

  let (code_row, code_col) = (numbers[0], numbers[1]);

  let mut code: u64 = 20151125;

  let mut row = 1; let mut col = 1;
  loop {
    if row == 1 {

    row = col + 1; col = 1;
   } else {
    row -= 1;
    col += 1
   }

   code = (code * 252533) % 33554393;
   
   if row == code_row && col == code_col { println!("{}", code); break }

  }

}