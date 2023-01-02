use day22::{get_spells_in_store, Player, Boss, Effect, get_effects_in_store, Turn};
use priority_queue::PriorityQueue;

fn main() {
  let lines = global::read_lines();

  let hit_points = lines[0].split(' ').last().unwrap().parse::<i32>().unwrap();
  let damage = lines[1].split(' ').last().unwrap().parse::<i32>().unwrap();

  let spells = get_spells_in_store();
  let effects = get_effects_in_store();

  let boss = Boss { hit_points, damage };
  let player = Player { hit_points: 50, mana: 500, effects: vec![] };

  let mut pq = PriorityQueue::new();
  pq.push((player, boss, Turn::Player), 0);

  while !pq.is_empty() {
    let ((mut player, mut boss, turn), total_mana) = pq.pop().unwrap(); 

    if turn == Turn::Boss {
      let mut player_armor = 0;

      for effect in &player.effects {
        player.mana += effect.mana_increase;
        boss.hit_points -= effect.damage_dealt;
        player_armor += effect.armor_increase;
      }

      if boss.hit_points <= 0 { println!("{}", -total_mana); break; }

      player.effects = player.effects.iter().map(|e| Effect { turns: e.turns - 1, ..e.clone()}).collect();
      player.effects = player.effects.iter().cloned().filter(|e| e.turns > 0).collect();

      let boss_damage = boss.damage - player_armor;
      player.hit_points -= if boss_damage < 1 { 1 } else { boss_damage };  

      if player.hit_points <= 0 { continue }

      pq.push((player.clone(), boss.clone(), Turn::Player), total_mana);
    } else {
      player.hit_points -= 1;
      if player.hit_points <= 0 { continue }

      for effect in &player.effects {
        player.mana += effect.mana_increase;
        boss.hit_points -= effect.damage_dealt;
      }

      if boss.hit_points <= 0 { println!("{}", -total_mana); break; }

      player.effects = player.effects.iter().map(|e| Effect { turns: e.turns - 1, ..e.clone()}).collect();
      player.effects = player.effects.iter().cloned().filter(|e| e.turns > 0).collect();

      for spell in &spells {
        if spell.mana <= player.mana {
          let mut next_player = player.clone();
          let mut next_boss = boss.clone();

          next_player.mana -= spell.mana;
          next_boss.hit_points -= spell.damage_dealt; 
          next_player.hit_points += spell.heals_player;
  
          pq.push((next_player, next_boss, Turn::Boss), total_mana - spell.mana);  
        }
      }
  
      for effect in &effects {
        if player.effects.iter().map(|e| e.name.clone()).any(|n| n == effect.name) { continue; }
  
        if effect.mana <= player.mana {
          let mut next_player = player.clone();
          let next_boss = boss.clone();
          next_player.mana -= effect.mana;
          
          next_player.effects.push(effect.clone());
          
          pq.push((next_player, next_boss, Turn::Boss), total_mana - effect.mana);  
        }
      }
    }
  }
}