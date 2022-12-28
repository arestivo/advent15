use parse_display::helpers::regex::Regex;

fn main () {
  let json = global::read_line();

  println!("{}", total(&json));
}

fn total(json: &str) -> i64 {
  let mut current = json.to_string();

  let re = Regex::new(r"\{[^{}]+}").unwrap();
  while re.is_match(&current) {
    let inner = re.captures_iter(&current).next().unwrap()[0].to_string();
    if inner.len() < current.len() {
      let total = total(&inner);
      current = re.replace(&current, total.to_string()).to_string();  
    } else { break; }
  }

  if current.contains(r#":"red""#) { return 0; }

  let re = Regex::new(r"-?\d+").unwrap();
  re.captures_iter(&current).map(|c| c[0].parse::<i64>().unwrap()).sum::<i64>()
}