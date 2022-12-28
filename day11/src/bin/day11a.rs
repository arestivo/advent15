use day11::{next_password};

fn main () {
  let pwd = global::read_line();

  if let Some(pwd) = next_password(&pwd, "", 0, false, false) {
    println!("{}", pwd);
  }

}