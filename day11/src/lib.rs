pub fn next_password(pwd: &str, current: &str, pairs: u32, just_made_pair: bool, has_sequence: bool) -> Option<String> {
  if pwd == current { return None; }
  if current.len() == pwd.len() {
    if pairs == 2 && has_sequence { return Some(current.to_string()) }
    else { return None }
  }

  let last = if current.len() > 1 { current.chars().nth(current.len() - 1).unwrap()} else { 'X' };
  let before_last = if current.len() > 1 { current.chars().nth(current.len() - 2).unwrap()} else { 'X' };

  let first = if pwd.starts_with(current) { pwd.chars().nth(current.len()).unwrap() } else { 'a' };

  for c in first..='z' {
    if c == 'i' || c == 'o' || c == 'l' { continue }

    let (pairs, just_made_pair) = (pairs + u32::from(c == last && !just_made_pair), c == last && !just_made_pair) ;

    let has_sequence = has_sequence || (last as u8 + 1 == c as u8 && before_last as u8 + 2 == c as u8);

    let mut next = "".to_string();
    next.push_str(current);
    next.push(c);

    if let Some(pwd) = next_password(pwd, &next, pairs, just_made_pair, has_sequence) { return Some(pwd) }
  }

  None
}