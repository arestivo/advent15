use parse_display::helpers::regex::Regex;

fn main() {
  let lines = global::read_lines();

  println!("{}", lines.iter().map(|s| s.len() - decode(s).len()).sum::<usize>());
}

fn decode(s: &str) -> String {
  let mut n = s.to_owned();

  let re = Regex::new(r"\\x([0-9a-f][0-9a-f])").unwrap();
  n = re.replace_all(&n, "X").to_string();

  let re = Regex::new(r#"\\""#).unwrap();
  n = re.replace_all(&n, "\"").to_string();

  let re = Regex::new(r#"\\\\"#).unwrap();
  n = re.replace_all(&n, "\\").to_string();

  n[1..n.len()-1].to_string()
}