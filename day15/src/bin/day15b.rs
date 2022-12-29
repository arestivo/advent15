use std::{collections::HashMap, cmp::max};

use day15::{lines_to_ingredients, Ingredient};

fn main () {
  let lines = global::read_lines();
  let ingredients = lines_to_ingredients(&lines);

  let mut quantities = HashMap::new();
  let names = ingredients.keys();
  for name in names.clone() { quantities.insert(name.clone(), 0); };

  let mut mem = HashMap::new();

  println!("{:?}", best_cookie(&ingredients, &quantities, &mut mem));
}

fn best_cookie(ingredients: &HashMap<String, Ingredient>, quantities: &HashMap<String, i32>, mem: &mut HashMap<String, i32>) -> i32 {
  let hash = format!("{:?}", quantities);
  if let Some(t) = mem.get(&hash) {
    return *t;
  }

  let calories = calories(ingredients, quantities);
  
  if calories > 500 { return i32::MIN }

  let tablespoons = quantities.iter().fold(0, |count, (_, quantity)| count + quantity);
  if tablespoons == 100 { return if calories == 500 { total(ingredients, quantities) } else { 0 }; }

  let names = ingredients.keys();

  let mut best = i32::MIN;

  for name in names.clone() {
    let mut quantities = quantities.clone();
    let quantity = quantities.get(name).unwrap();
    quantities.insert(name.clone(), quantity + 1);

    best = max (best, best_cookie(ingredients, &quantities, mem));
  }

  mem.insert(hash, best);

  best
}

fn total(ingredients: &HashMap<String, Ingredient>, quantities: &HashMap<String, i32>) -> i32 {
  let total = quantities.iter().fold(
    Ingredient{ name: "Total".to_string(), capacity: 0, durability: 0, flavor: 0, texture: 0, calories: 0 }, 
    |total, (n, q )| {
      let i = ingredients.get(n).unwrap();
      Ingredient { name: "Total".to_string(), 
        capacity: total.capacity + q * i.capacity , 
        durability: total.durability + q * i.durability, 
        flavor: total.flavor + q * i.flavor, 
        texture: total.texture + q * i.texture, 
        ..i.clone()
      }
    });

  max(0, total.capacity) * max(0, total.durability) * max(0, total.flavor) * max(0, total.texture)
}

fn calories(ingredients: &HashMap<String, Ingredient>, quantities: &HashMap<String, i32>) -> i32 {
  let total = quantities.iter().fold(
    Ingredient{ name: "Total".to_string(), capacity: 0, durability: 0, flavor: 0, texture: 0, calories: 0 }, 
    |total, (n, q )| {
      let i = ingredients.get(n).unwrap();
      Ingredient { name: "Total".to_string(), 
        calories: total.calories + q * i.calories,
        ..i.clone()
      }
    });

  total.calories
}