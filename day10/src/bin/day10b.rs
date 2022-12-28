use day10::{expand, Part, look_and_say, to_string};

fn main () {
  let s = global::read_line();

  let mut parts = s.chars().map(|c| Part { digit: c, count: 1 } ).collect();

  for i in 0..50 { 
    if i != 0 { parts = expand(&parts); }
    parts = look_and_say(&parts); 
  }

  println!("{:?}", to_string(&parts).len());
}