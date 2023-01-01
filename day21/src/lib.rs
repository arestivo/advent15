#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Kind {
  Weapon,
  Armor,
  Ring
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Item {
  pub kind: Kind,
  pub name: String,
  pub cost: u32,
  pub damage: u32,
  pub armor: u32  
}

#[derive(Debug, Clone)]
pub struct Character {
  pub hit_points: i32,
  pub damage: i32,
  pub armor: i32
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BattleResult {
  Player(i32, i32),
  Boss(i32, i32)
}

pub fn get_items_in_store () -> Vec<Item> {
  vec![
    Item { kind: Kind::Weapon, name: "Dagger".to_string(),     cost: 8,   damage: 4, armor: 0 },
    Item { kind: Kind::Weapon, name: "Shortsword".to_string(), cost: 10,  damage: 5, armor: 0 },
    Item { kind: Kind::Weapon, name: "Warhammer".to_string(),  cost: 25,  damage: 6, armor: 0 },
    Item { kind: Kind::Weapon, name: "Longsword".to_string(),  cost: 40,  damage: 7, armor: 0 },
    Item { kind: Kind::Weapon, name: "Greataxe".to_string(),   cost: 74,  damage: 8, armor: 0 },

    Item { kind: Kind::Armor, name: "Leather".to_string(),     cost: 13,  damage: 0, armor: 1 },
    Item { kind: Kind::Armor, name: "Chainmail".to_string(),   cost: 31,  damage: 0, armor: 2 },
    Item { kind: Kind::Armor, name: "Splintmail".to_string(),  cost: 53,  damage: 0, armor: 3 },
    Item { kind: Kind::Armor, name: "Bandedmail".to_string(),  cost: 75,  damage: 0, armor: 4 },
    Item { kind: Kind::Armor, name: "Platemail".to_string(),   cost: 102, damage: 0, armor: 5 },

    Item { kind: Kind::Ring, name: "Damage +1".to_string(),    cost: 25,  damage: 1, armor: 0 },
    Item { kind: Kind::Ring, name: "Damage +2".to_string(),    cost: 50,  damage: 2, armor: 0 },
    Item { kind: Kind::Ring, name: "Damage +3".to_string(),    cost: 100, damage: 3, armor: 0 },
    Item { kind: Kind::Ring, name: "Defense +1".to_string(),   cost: 20,  damage: 0, armor: 1 },
    Item { kind: Kind::Ring, name: "Defense +2".to_string(),   cost: 40,  damage: 0, armor: 2 },
    Item { kind: Kind::Ring, name: "Defense +3".to_string(),   cost: 80,  damage: 0, armor: 3 },
  ]
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Configuration {
  pub weapon: Option<Item>,
  pub armor: Option<Item>,
  pub ring1: Option<Item>,
  pub ring2: Option<Item>,
}

pub fn battle(player: &Character, boss: &Character) -> BattleResult {
  let mut player = player.clone();
  let mut boss = boss.clone();

  let boss_attack = boss.damage - player.armor;
  let boss_attack = if boss_attack < 1 { 1 } else { boss_attack };

  let player_attack = player.damage - boss.armor;
  let player_attack = if player_attack < 1 { 1 } else { player_attack };

  loop {
    boss.hit_points -= player_attack;
    if boss.hit_points <= 0 { return BattleResult::Player(player.hit_points, boss.hit_points) }

    player.hit_points -= boss_attack;
    if player.hit_points <= 0 { return BattleResult::Boss(player.hit_points, boss.hit_points) }
  }
}

impl Configuration {
  pub fn generate_character(&self, hit_points: i32) -> Character {
    let weapon_damage = if let Some(weapon) = &self.weapon { weapon.damage } else { 0 };
    let ring1_damage = if let Some(ring) = &self.ring1 { ring.damage } else { 0 };
    let ring2_damage = if let Some(ring) = &self.ring2 { ring.damage } else { 0 };

    let armor_armor = if let Some(armor) = &self.armor { armor.armor } else { 0 };
    let ring1_armor = if let Some(ring) = &self.ring1 { ring.armor } else { 0 };
    let ring2_armor = if let Some(ring) = &self.ring2 { ring.armor } else { 0 };

    let damage = weapon_damage + ring1_damage + ring2_damage;
    let armor = armor_armor + ring1_armor + ring2_armor;

    Character { hit_points, damage: damage.try_into().unwrap(), armor: armor.try_into().unwrap() }
  }
}