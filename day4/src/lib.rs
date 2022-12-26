pub fn hash(secret: &str, num: u64) -> String {
  let digest = md5::compute(format!("{}{}", secret, num));
  format!("{:x}", digest)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hash_works() {
    assert_eq!("000006136ef2ff3b291c85725f17325c", hash("pqrstuv", 1048970));
  }
}