use day21::{get_items_in_store, battle, Character, Configuration, BattleResult, Kind};
use priority_queue::PriorityQueue;

fn main() {
  let lines = global::read_lines();

  let hit_points = lines[0].split(' ').last().unwrap().parse::<i32>().unwrap();
  let damage = lines[1].split(' ').last().unwrap().parse::<i32>().unwrap();
  let armor = lines[2].split(' ').last().unwrap().parse::<i32>().unwrap();

  let items = get_items_in_store();

  let mut pq: PriorityQueue<Configuration, i32> = PriorityQueue::new();
  pq.push(Configuration { weapon: None, armor: None, ring1: None, ring2: None }, 0);

  let boss = Character { hit_points, damage, armor };

  while !pq.is_empty() {
    let (configuration, cost) = pq.pop().unwrap();
    let player = configuration.generate_character(100);

    match battle(&player, &boss) {
        BattleResult::Player(_, _) => { println!("{}", -cost); break; },
        BattleResult::Boss(_, _) => { },
    }

    if configuration.weapon.is_none() {
      let mut next = configuration.clone();
      for weapon in items.iter().filter(|i| i.kind == Kind::Weapon) {
        next.weapon = Some(weapon.clone());
        pq.push(next.clone(), cost - weapon.cost as i32);
      }
    }

    if configuration.ring1.is_none() {
      let mut next = configuration.clone();
      for ring in items.iter().filter(|i| i.kind == Kind::Ring) {
        next.ring1 = Some(ring.clone());
        pq.push(next.clone(), cost - ring.cost as i32);
      }
    }

    if configuration.ring2.is_none() {
      let mut next = configuration.clone();
      for ring in items.iter().filter(|i| i.kind == Kind::Ring) {
        next.ring2 = Some(ring.clone());
        pq.push(next.clone(), cost - ring.cost as i32);
      }
    }

    if configuration.armor.is_none() {
      let mut next = configuration.clone();
      for armor in items.iter().filter(|i| i.kind == Kind::Armor) {
        next.armor = Some(armor.clone());
        pq.push(next.clone(), cost - armor.cost as i32);
      }
    }
  }

}