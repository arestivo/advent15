use day4::hash;

fn main() {
  let secret = global::read_line();
  let mut num = 0;
  
  while !hash(&secret, num).starts_with("00000") { num += 1 }

  println!("{}", num);
}