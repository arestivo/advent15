use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
pub struct AuntSue {
  pub number: u32,
  pub things: HashMap<String, u32>
}
 
pub fn lines_to_aunts(lines: &[String]) -> Vec<AuntSue> {
  let re = Regex::new(r"Sue (\d+): (.+)").unwrap();

  lines
    .iter()
    .map(|line| {
      let cap = re.captures_iter(line).next().unwrap();
      let number =  cap[1].parse::<u32>().unwrap();
      let rest =  cap[2].to_string();
      let parts: Vec<Vec<&str>> = rest.split(',').map(|p| p.split(": ").collect()).collect();
      let things = 
        parts
          .iter()
          .map(|p| (p.first().unwrap().to_string().trim().to_owned(), p.last().unwrap().parse().unwrap()))
          .collect::<HashMap<_,_>>();
      AuntSue { 
        number, 
        things
      }
    })
    .collect()
}