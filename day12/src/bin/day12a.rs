use parse_display::helpers::regex::Regex;

fn main () {
  let json = global::read_line();

  let re = Regex::new(r"-?\d+").unwrap();
  let total: i64 = re.captures_iter(&json).map(|c| c[0].parse::<i64>().unwrap()).sum();

  println!("{}", total);
}