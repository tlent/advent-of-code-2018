use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;
use std::usize;

const INPUT: &str = include_str!("../input");
#[allow(dead_code)]
const MOVEMENT_SAMPLE_INPUT: &str = include_str!("../sample-input-movement");
#[allow(dead_code)]
const COMBAT_SAMPLE_INPUT: &str = include_str!("../sample-input-combat");

const START_HP: u32 = 200;
const ATTACK_POWER: u32 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Point {
  y: usize,
  x: usize,
}

impl Point {
  fn distance_to(&self, other: &Self) -> u32 {
    let dx = (self.x as i32 - other.x as i32).abs() as u32;
    let dy = (self.y as i32 - other.y as i32).abs() as u32;
    dx + dy
  }

  fn neighbors(&self) -> Vec<Self> {
    let mut result = vec![];
    if self.y > 0 {
      result.push(Point {
        y: self.y - 1,
        x: self.x,
      });
    }
    if self.x > 0 {
      result.push(Point {
        y: self.y,
        x: self.x - 1,
      });
    }
    if self.x < usize::MAX {
      result.push(Point {
        y: self.y,
        x: self.x + 1,
      });
    }
    if self.y < usize::MAX {
      result.push(Point {
        y: self.y + 1,
        x: self.x,
      });
    }
    result
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Team {
  Goblin,
  Elf,
}

impl Team {
  fn from_char(c: char) -> Option<Self> {
    match c {
      'G' => Some(Team::Goblin),
      'E' => Some(Team::Elf),
      _ => None,
    }
  }

  fn to_char(&self) -> char {
    match &self {
      Team::Goblin => 'G',
      Team::Elf => 'E',
    }
  }

  fn full_name(&self) -> String {
    match &self {
      Team::Goblin => String::from("Goblins"),
      Team::Elf => String::from("Elves"),
    }
  }
}

impl fmt::Display for Team {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(f, "{}", self.to_char())
  }
}

#[derive(Debug, Clone)]
struct Unit {
  id: usize,
  team: Team,
  hit_points: u32,
  attack_power: u32,
  position: Point,
}

impl Unit {
  fn new(id: usize, team: Team, position: Point) -> Self {
    Self {
      id,
      team,
      hit_points: START_HP,
      attack_power: ATTACK_POWER,
      position,
    }
  }

  fn step(&mut self, world: &mut World) -> bool {
    if self.hit_points == 0 {
      return false;
    }
    let living_enemy_count = world
      .units
      .iter()
      .filter(|u| u.team != self.team && u.hit_points > 0)
      .count();
    if living_enemy_count == 0 {
      return true;
    }
    let mut adjacent_live_enemies: Vec<_> = self
      .position
      .neighbors()
      .iter()
      .filter_map(|p| world.get_unit_at_position(*p))
      .filter(|u| u.team != self.team && u.hit_points > 0)
      .collect();
    if adjacent_live_enemies.is_empty() {
      self.move_(world);
      adjacent_live_enemies = self
        .position
        .neighbors()
        .iter()
        .filter_map(|p| world.get_unit_at_position(*p))
        .filter(|u| u.team != self.team && u.hit_points > 0)
        .collect();
    }
    if adjacent_live_enemies.is_empty() {
      return false;
    }
    let target_id = adjacent_live_enemies
      .iter()
      .min_by_key(|e| (e.hit_points, e.position))
      .map(|t| t.id)
      .unwrap();
    let mut target = world.get_unit_mut(target_id).unwrap();
    target.hit_points = target.hit_points.saturating_sub(self.attack_power);
    false
  }

  fn move_(&mut self, world: &World) {
    let live_enemies: Vec<_> = world
      .units
      .iter()
      .filter(|u| u.team != self.team && u.hit_points > 0)
      .collect();
    let in_range_positions: Vec<_> = live_enemies
      .iter()
      .map(|u| world.get_open_neighbors(u.position))
      .flatten()
      .collect();
    let reachable_positions: Vec<_> = in_range_positions
      .iter()
      .filter_map(|p| {
        world
          .find_shortest_path(self.position, *p)
          .map(|path| (*p, path.len()))
      })
      .collect();
    if reachable_positions.is_empty() {
      return;
    }
    let (target, shortest_path_len) = reachable_positions
      .iter()
      .min_by_key(|(position, path_len)| (path_len, position))
      .unwrap();
    let next_position = world
      .get_open_neighbors(self.position)
      .iter()
      .filter_map(|p| world.find_shortest_path(*p, *target))
      .filter(|path| path.len() == shortest_path_len - 1)
      .map(|path| path[0])
      .min()
      .unwrap();
    self.position = next_position;
  }
}

impl fmt::Display for Unit {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(f, "{}", self.team)
  }
}

#[derive(Debug, Clone)]
struct World {
  walls: Vec<Point>,
  units: Vec<Unit>,
  bounds: (usize, usize),
}

impl World {
  fn from_input(input: &str) -> Self {
    let input = input.trim();
    let mut walls = vec![];
    let mut units = vec![];
    let max_y = input.split('\n').count();
    let max_x = input
      .split('\n')
      .nth(0)
      .map(|line| line.chars().count())
      .unwrap();
    let bounds = (max_x, max_y);
    for (i, line) in input.split('\n').enumerate() {
      for (j, c) in line.chars().enumerate() {
        let current_point = Point { y: i, x: j };
        match c {
          '#' => {
            walls.push(current_point);
          }
          'E' | 'G' => {
            units.push(Unit::new(
              units.len(),
              Team::from_char(c).unwrap(),
              current_point,
            ));
          }
          '.' => {}
          _ => panic!("Invalid input character: {}", c),
        }
      }
    }
    Self {
      walls,
      units,
      bounds,
    }
  }

  fn step(&mut self) -> Option<(Team, u32)> {
    self.units.sort_by_key(|u| u.position);
    // println!("{}", self);
    // println!(
    //   "{:?}",
    //   self
    //     .units
    //     .iter()
    //     .map(|u| format!("{}{}({})", u.team, u.id, u.hit_points))
    //     .collect::<Vec<_>>()
    // );
    for i in 0..self.units.len() {
      let mut unit = self.units[i].clone();
      let game_over = unit.step(self);
      self.units[i] = unit;
      if game_over {
        self.units.retain(|u| u.hit_points > 0);
        let winning_team = self.units[0].team;
        let remaining_hp: u32 = self.units.iter().map(|u| u.hit_points).sum();
        return Some((winning_team, remaining_hp));
      }
    }
    self.units.retain(|u| u.hit_points > 0);
    None
  }

  fn simulate_combat(&mut self) -> (u32, Team, u32) {
    let mut rounds = 0;
    let mut result = None;
    while result.is_none() {
      println!("{}", rounds);
      result = self.step();
      rounds += 1;
    }
    let (winning_team, remaining_hp) = result.unwrap();
    (rounds - 1, winning_team, remaining_hp)
  }

  fn has_wall_at_position(&self, position: Point) -> bool {
    self.walls.contains(&position)
  }

  fn get_unit_at_position(&self, position: Point) -> Option<&Unit> {
    self.units.iter().find(|u| u.position == position)
  }

  fn find_shortest_path(&self, start: Point, goal: Point) -> Option<Vec<Point>> {
    a_star(start, goal, self)
  }

  fn get_unit_mut(&mut self, id: usize) -> Option<&mut Unit> {
    self.units.iter_mut().find(|u| u.id == id)
  }

  fn get_open_neighbors(&self, point: Point) -> Vec<Point> {
    let occupied_points: HashSet<_> = self
      .units
      .iter()
      .filter(|u| u.hit_points > 0)
      .map(|u| u.position)
      .chain(self.walls.iter().cloned())
      .collect();
    let neighboring_points: HashSet<_> = point.neighbors().into_iter().collect();
    neighboring_points
      .difference(&occupied_points)
      .cloned()
      .collect()
  }
}

impl fmt::Display for World {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    let (max_x, max_y) = self.bounds;
    for i in 0..max_y {
      for j in 0..max_x {
        let current_position = Point { x: j, y: i };
        let has_wall = self.has_wall_at_position(current_position);
        let unit = self.get_unit_at_position(current_position);
        if unit.is_some() {
          unit.unwrap().fmt(f)?;
        } else if has_wall {
          write!(f, "#")?;
        } else {
          write!(f, ".")?;
        }
      }
      writeln!(f)?;
    }
    Ok(())
  }
}

fn a_star(start: Point, goal: Point, world: &World) -> Option<Vec<Point>> {
  let mut evaluated_nodes = HashSet::new();
  let mut came_from = HashMap::new();
  let mut g_scores = HashMap::new();
  g_scores.insert(start, 0);
  let mut f_scores = HashMap::new();
  let start_f_score = start.distance_to(&goal);
  f_scores.insert(start, start_f_score);
  let mut new_nodes = BinaryHeap::new();
  new_nodes.push((Reverse(start_f_score), start));
  while !new_nodes.is_empty() {
    let current = new_nodes.pop().map(|(_, node)| node).unwrap();
    if current == goal {
      return Some(reconstruct_path(&came_from, current));
    }
    evaluated_nodes.insert(current);
    for neighbor in world.get_open_neighbors(current).iter() {
      if evaluated_nodes.contains(neighbor) {
        continue;
      }
      let new_g_score = g_scores.get(&current).unwrap() + 1;
      let is_neighbor_new = !new_nodes.iter().any(|&(_, node)| node == *neighbor);
      if !is_neighbor_new && new_g_score >= *g_scores.get(neighbor).unwrap() {
        continue;
      }
      came_from.insert(*neighbor, current);
      g_scores.insert(*neighbor, new_g_score);
      let new_f_score = new_g_score + neighbor.distance_to(&goal);
      f_scores.insert(*neighbor, new_f_score);
      if is_neighbor_new {
        new_nodes.push((Reverse(new_f_score), *neighbor));
      }
    }
  }
  None
}

fn reconstruct_path(came_from: &HashMap<Point, Point>, current: Point) -> Vec<Point> {
  let mut path = vec![current];
  let mut current = current;
  while came_from.contains_key(&current) {
    current = *came_from.get(&current).unwrap();
    path.push(current);
  }
  path.reverse();
  path
}

fn main() {
  let world = World::from_input(INPUT);
  let mut part_one_world = world.clone();
  let (rounds, winning_team, remaining_hp) = part_one_world.simulate_combat();
  println!("{}", part_one_world);
  println!("Combat ends after {} rounds", rounds);
  println!(
    "{} win with {} hitpoints left",
    winning_team.full_name(),
    remaining_hp
  );
  println!("Outcome: {}", remaining_hp * rounds);
}

#[cfg(test)]
mod test {
  use super::*;

  const SAMPLE_INPUTS: [&str; 5] = [
    include_str!("../sample-input-1"),
    include_str!("../sample-input-2"),
    include_str!("../sample-input-3"),
    include_str!("../sample-input-4"),
    include_str!("../sample-input-5"),
  ];
  const SAMPLE_RESULTS: [(u32, Team, u32); 5] = [
    (37, Team::Elf, 982),
    (46, Team::Elf, 859),
    (35, Team::Goblin, 793),
    (54, Team::Goblin, 536),
    (20, Team::Goblin, 937),
  ];

  #[test]
  fn it_solves_combat_sample_correctly() {
    let (rounds, winning_team, remaining_hp) =
      World::from_input(COMBAT_SAMPLE_INPUT).simulate_combat();
    assert_eq!(rounds, 47);
    assert_eq!(winning_team, Team::Goblin);
    assert_eq!(remaining_hp, 590);
  }

  #[test]
  fn it_solves_movement_sample_correctly() {
    let (rounds, winning_team, remaining_hp) =
      World::from_input(MOVEMENT_SAMPLE_INPUT).simulate_combat();
    assert_eq!(rounds, 18);
    assert_eq!(winning_team, Team::Goblin);
    assert_eq!(remaining_hp, 1546);
  }

  #[test]
  fn it_solves_samples_correctly() {
    for sample_number in 0..5 {
      let input = SAMPLE_INPUTS[sample_number];
      let (rounds, winning_team, remaining_hp) = World::from_input(input).simulate_combat();
      let expected = SAMPLE_RESULTS[sample_number];
      assert_eq!(
        rounds, expected.0,
        "Wrong rounds for sample #{}",
        sample_number
      );
      assert_eq!(
        winning_team, expected.1,
        "Wrong team for sample #{}",
        sample_number
      );
      assert_eq!(
        remaining_hp, expected.2,
        "Wrong remaining hp for sample #{}",
        sample_number
      );
    }
  }
}
