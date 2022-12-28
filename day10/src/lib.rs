
#[derive(Clone, Debug)]
pub struct Part {
  pub digit: char,
  pub count: u64
}

pub fn to_string(original: &Vec<Part>) -> String {
  let mut result = "".to_string();  

  for p in original {
    result.push_str(&p.count.to_string());
    result.push_str(&p.digit.to_string());
  }

  result
}

pub fn expand(original: &Vec<Part>) -> Vec<Part> {
  let mut expanded = vec![];

  for p in original {
    let p1: Vec<Part> = p.count.to_string().chars().map(|c| Part { digit: c, count: 1 }).collect();
    let p2 = Part { digit: p.digit, count: 1 };
    expanded.extend(p1.iter().cloned());
    expanded.push(p2);
  }

  expanded
}

pub fn look_and_say(original: &Vec<Part>) -> Vec<Part> {
  let mut result = vec![];
  let mut i = 0;

  while i < original.len() {
    let mut size = 0;

    while i + size < original.len() && (size == 0 || original[i + size - 1].digit == original[i + size].digit) {
      size += 1;
    }

    let total = original[i..i + size].iter().map(|r| r.count).sum();
    result.push(Part { digit: original[i].digit , count: total });

    i += size;
  }

  result
}