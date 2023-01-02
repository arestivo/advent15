#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Spell {
  pub name: String,
  pub mana: i32,

  pub damage_dealt: i32, 
  pub heals_player: i32
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Effect {
  pub name: String,
  pub mana: i32,
  pub turns: i32, 

  pub armor_increase: i32, 
  pub damage_dealt: i32, 
  pub mana_increase: i32
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Player {
  pub mana: i32,
  pub hit_points: i32,
  pub effects: Vec<Effect>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Boss {
  pub hit_points: i32,
  pub damage: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Turn {
  Player,
  Boss
}

pub fn get_spells_in_store () -> Vec<Spell> {
  vec![
    Spell { name: "Magic Missile".to_string(), mana: 53, damage_dealt: 4, heals_player: 0 },
    Spell { name: "Drain".to_string(), mana: 73, damage_dealt: 2, heals_player: 2 },
  ]
}

pub fn get_effects_in_store () -> Vec<Effect> {
  vec![
    Effect { name: "Shield".to_string(), mana: 113, turns: 6, armor_increase: 7, damage_dealt: 0, mana_increase: 0 },
    Effect { name: "Poison".to_string(), mana: 173, turns: 6, armor_increase: 0, damage_dealt: 3, mana_increase: 0 },
    Effect { name: "Recharge".to_string(), mana: 229, turns: 5, armor_increase: 0, damage_dealt: 0, mana_increase: 101 },
  ]
}