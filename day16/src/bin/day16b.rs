use day16::{lines_to_aunts, AuntSue};

fn main () {
  let lines = global::read_lines();
  let aunts = lines_to_aunts(&lines);

  let correct = vec![
    ("children", 3),
    ("cats", 7),
    ("samoyeds", 2),
    ("pomeranians", 3),
    ("akitas", 0),
    ("vizslas", 0),
    ("goldfish", 5),
    ("trees", 3),
    ("cars", 2),
    ("perfumes", 1),
  ];

  for aunt in aunts {
    if !{
      let mut is_correct = true;
      for (thing, quantity) in correct.clone() {
        if !is_correct_aunt(&aunt, thing, quantity) { is_correct = false; }
      }
      is_correct
    } { continue }
    println!("{}", aunt.number);
  }
}

fn is_correct_aunt(aunt: &AuntSue, thing: &str, quantity: u32) -> bool {
  if thing == "cats" || thing == "trees" {
    if let Some(thing) = aunt.things.get(thing) { if thing <= &quantity { return false; }}
  } else if thing == "pomeranians" || thing == "goldfish" {
    if let Some(thing) = aunt.things.get(thing) { if thing >= &quantity { return false; }}
  } else if let Some(thing) = aunt.things.get(thing) { if thing != &quantity { return false; }}

  true
}