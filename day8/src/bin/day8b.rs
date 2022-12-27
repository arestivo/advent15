use parse_display::helpers::regex::Regex;

fn main() {
  let lines = global::read_lines();

  println!("{}", lines.iter().map(|s| encode(s).len() - s.len() + 2).sum::<usize>());
}

fn encode(s: &str) -> String {
  let mut n = s.to_owned();

  let re = Regex::new(r#"\\"#).unwrap();
  n = re.replace_all(&n, r#"\\"#).to_string();

  let re = Regex::new(r#"""#).unwrap();
  re.replace_all(&n, r#"\""#).to_string()
}