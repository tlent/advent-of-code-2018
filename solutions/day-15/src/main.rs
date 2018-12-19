use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;
use std::usize;

const INPUT: &str = include_str!("../input");
#[allow(dead_code)]
const MOVEMENT_SAMPLE_INPUT: &str = include_str!("../sample-input-movement");
#[allow(dead_code)]
const COMBAT_SAMPLE_INPUT: &str = include_str!("../sample-input-combat");

const START_HP: u32 = 200;
const START_ATTACK_POWER: u32 = 3;
const PART_TWO_MIN_ATTACK_POWER: u32 = 4;

// TODO:
// EASY PART TWO IMPROVEMENT: End combat when first elf dies
// Opportunity for big clean up: Find a way to actually remove dead units
// Possibly speed up everything by improving the movement checks. Currently there are a lot of
// repeated checks that may not all be necessary

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
            attack_power: START_ATTACK_POWER,
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
            .filter_map(|p| world.get_live_unit_at_position(*p))
            .filter(|u| u.team != self.team && u.hit_points > 0)
            .collect();
        if adjacent_live_enemies.is_empty() {
            self.move_(world);
            adjacent_live_enemies = self
                .position
                .neighbors()
                .iter()
                .filter_map(|p| world.get_live_unit_at_position(*p))
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
        let mut in_range_positions: Vec<_> = live_enemies
            .iter()
            .map(|u| world.get_open_neighbors(u.position))
            .flatten()
            .collect();
        in_range_positions.sort_by_key(|p| self.position.distance_to(&p));
        let mut min_result = None;
        for p in in_range_positions {
            let shortest_path_len = min_result.map(|(shortest_path_len, _)| shortest_path_len);
            if shortest_path_len.is_some()
                && self.position.distance_to(&p) > shortest_path_len.unwrap()
            {
                continue;
            }
            let shortest_path = world.find_shortest_path(self.position, p);
            if shortest_path.is_none() {
                continue;
            }
            let shortest_path = shortest_path.unwrap();
            let current_result = (shortest_path.len() as u32, p);
            if min_result.is_none() || current_result < min_result.unwrap() {
                min_result = Some(current_result)
            }
        }
        if min_result.is_none() {
            return;
        }
        let (shortest_path_len, target) = min_result.unwrap();
        let next_position = world
            .get_open_neighbors(self.position)
            .iter()
            .filter_map(|p| world.find_shortest_path(*p, target))
            .filter(|path| (path.len() as u32) == shortest_path_len - 1)
            .map(|path| path[0])
            .min()
            .unwrap();
        self.position = next_position;
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}({})", self.team, self.hit_points)
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
            println!("{}", self);
            result = self.step();
            rounds += 1;
        }
        let (winning_team, remaining_hp) = result.unwrap();
        (rounds - 1, winning_team, remaining_hp)
    }

    fn has_wall_at_position(&self, position: Point) -> bool {
        self.walls.contains(&position)
    }

    fn get_live_unit_at_position(&self, position: Point) -> Option<&Unit> {
        self.units
            .iter()
            .find(|u| u.hit_points > 0 && u.position == position)
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

    fn set_elves_attack_power(&mut self, attack_power: u32) {
        for elf in self.units.iter_mut().filter(|u| u.team == Team::Elf) {
            elf.attack_power = attack_power;
        }
    }

    fn find_minimum_no_loss_elf_win_attack_power(&mut self) -> ((u32, Team, u32), u32) {
        for attack_power in PART_TWO_MIN_ATTACK_POWER.. {
            let mut world = self.clone();
            println!("Trying attack power {}", attack_power);
            world.set_elves_attack_power(attack_power);
            let before_combat_elf_count =
                world.units.iter().filter(|u| u.team == Team::Elf).count();
            let result = world.simulate_combat();
            let after_combat_elf_count = world.units.iter().filter(|u| u.team == Team::Elf).count();
            if result.1 == Team::Elf && before_combat_elf_count == after_combat_elf_count {
                return (result, attack_power);
            }
        }
        unreachable!()
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let (max_x, max_y) = self.bounds;
        for i in 0..max_y {
            let mut row_units = vec![];
            for j in 0..max_x {
                let current_position = Point { x: j, y: i };
                let has_wall = self.has_wall_at_position(current_position);
                let unit = self.get_live_unit_at_position(current_position);
                if unit.is_some() {
                    let u = unit.unwrap();
                    row_units.push(u);
                    write!(f, "{}", u.team)?;
                } else if has_wall {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, " ")?;
            for u in row_units.iter() {
                write!(f, "{} ", u)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct MinHeapWrapper<T> {
    data: T,
    f_score: u32,
}

impl<T> Eq for MinHeapWrapper<T> {}

impl<T> PartialEq for MinHeapWrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        self.f_score == other.f_score
    }
}

impl<T> Ord for MinHeapWrapper<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        Reverse(self.f_score).cmp(&Reverse(other.f_score))
    }
}

impl<T> PartialOrd for MinHeapWrapper<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn a_star(start: Point, goal: Point, world: &World) -> Option<Vec<Point>> {
    let mut seen_nodes = HashSet::new();
    let mut came_from = HashMap::new();
    let mut g_scores = HashMap::new();
    g_scores.insert(start, 0);
    let mut f_scores = HashMap::new();
    let start_f_score = start.distance_to(&goal);
    f_scores.insert(start, start_f_score);
    let mut new_nodes = BinaryHeap::new();
    new_nodes.push(MinHeapWrapper {
        f_score: start_f_score,
        data: start,
    });
    while !new_nodes.is_empty() {
        let wrapper = new_nodes.pop().unwrap();
        if wrapper.f_score != *f_scores.get(&wrapper.data).unwrap() {
            // wrapper with an outdated f_score
            continue;
        }
        let current = wrapper.data;
        if current == goal {
            return Some(reconstruct_path(&came_from, current));
        }
        seen_nodes.insert(current);
        for neighbor in world.get_open_neighbors(current).iter() {
            if seen_nodes.contains(neighbor) {
                continue;
            }
            let new_g_score = g_scores.get(&current).unwrap() + 1;
            if new_nodes.iter().any(|w| w.data == *neighbor)
                && new_g_score >= *g_scores.get(neighbor).unwrap()
            {
                continue;
            }
            came_from.insert(*neighbor, current);
            g_scores.insert(*neighbor, new_g_score);
            let new_f_score = new_g_score + neighbor.distance_to(&goal);
            f_scores.insert(*neighbor, new_f_score);
            new_nodes.push(MinHeapWrapper {
                f_score: new_f_score,
                data: *neighbor,
            });
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
    let initial_world = World::from_input(INPUT);
    let mut world = initial_world.clone();
    let (rounds, winning_team, remaining_hp) = world.simulate_combat();
    println!("{}", world);
    println!("Combat ends after {} rounds", rounds);
    println!(
        "{} win with {} hitpoints left",
        winning_team.full_name(),
        remaining_hp
    );
    println!("Outcome: {}", remaining_hp * rounds);
    let mut world = initial_world.clone();
    let ((rounds, winning_team, remaining_hp), min_attack_power) =
        world.find_minimum_no_loss_elf_win_attack_power();
    println!("{}", world);
    println!("Combat ends after {} rounds", rounds);
    println!(
        "{} win with {} hitpoints left",
        winning_team.full_name(),
        remaining_hp
    );
    println!("Outcome: {}", remaining_hp * rounds);
    println!("Min attack power: {}", min_attack_power);
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
    const PART_ONE_SAMPLE_RESULTS: [(u32, Team, u32); 5] = [
        (37, Team::Elf, 982),
        (46, Team::Elf, 859),
        (35, Team::Goblin, 793),
        (54, Team::Goblin, 536),
        (20, Team::Goblin, 937),
    ];
    const PART_TWO_SAMPLE_RESULTS: [u32; 4] = [4, 15, 12, 34];

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
            let expected = PART_ONE_SAMPLE_RESULTS[sample_number];
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

    #[test]
    fn it_solves_part_two_samples_correctly() {
        let input = COMBAT_SAMPLE_INPUT;
        let result = World::from_input(input).find_minimum_no_loss_elf_win_attack_power();
        assert_eq!(result.1, 15, "failed for combat sample");

        for sample_number in 1..4 {
            let input = SAMPLE_INPUTS[sample_number];
            let result = World::from_input(input).find_minimum_no_loss_elf_win_attack_power();
            let expected = PART_TWO_SAMPLE_RESULTS[sample_number - 1];
            assert_eq!(result.1, expected, "failed for sample #{}", sample_number);
        }
    }
}
