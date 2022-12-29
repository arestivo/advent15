use std::collections::HashMap;

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug, Clone, Hash)]
#[display("{name}: capacity {capacity}, durability {durability}, flavor {flavor}, texture {texture}, calories {calories}")]
pub struct Ingredient {
  pub name: String,
  pub capacity: i32,
  pub durability: i32,
  pub flavor: i32,
  pub texture: i32,
  pub calories: i32,
}

pub fn lines_to_ingredients(lines: &[String]) -> HashMap<String, Ingredient> {
  lines
    .iter()
    .map(|l| l.parse::<Ingredient>())
    .map(Result::unwrap)
    .map(|rd| (rd.name.clone(), rd))
    .collect::<HashMap<_,_>>()
}