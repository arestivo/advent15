use std::collections::HashSet;

use day19::{lines_to_instruction, Instruction, apply_all};
use priority_queue::PriorityQueue;

fn main() {
  let lines = global::read_lines();
  let instructions = lines_to_instruction(&lines[0..lines.len() - 2]);
  let instructions = instructions.iter().map(|i| Instruction{ from: i.to.clone(), to: i.from.clone() }).collect();
  let molecule = lines[lines.len() - 1].to_owned();
  let mut visited = HashSet::new();

  println!("{}", bfs(instructions, molecule, "e".to_string(), &mut visited));
}

pub fn bfs(instructions: Vec<Instruction>, start: String, objective: String, visited: &mut HashSet<String>) -> u32 {
  let mut q: PriorityQueue<(String, u32), usize> = PriorityQueue::new();
  q.push((start, 0), 0);

  while !q.is_empty() {
    let ((current, steps), _) = q.pop().unwrap();
    
    if current == objective { return steps; }
    
    let results = apply_all(&instructions, current);
    for r in results { 
      if visited.contains(&r) { continue; }
      visited.insert(r.clone());  
      q.push((r.clone(), steps + 1), usize::MAX - r.len()); 
    }
  }

  unreachable!()
}