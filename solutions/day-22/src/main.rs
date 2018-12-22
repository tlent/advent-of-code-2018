use std::cmp::Ordering;
use std::collections::hash_map::Entry::*;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;
use std::iter;

const INPUT: &str = include_str!("../input");
const MOVE_COST: usize = 1;
const TOOL_CHANGE_COST: usize = 7;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from_tuple(t: (usize, usize)) -> Self {
        let (x, y) = t;
        Point { x, y }
    }

    fn manhattan_distance_to(&self, other: &Self) -> usize {
        let dx = (self.x as isize - other.x as isize).abs() as usize;
        let dy = (self.y as isize - other.y as isize).abs() as usize;
        dx + dy
    }

    fn as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RegionType {
    Rocky,
    Narrow,
    Wet,
}

impl RegionType {
    fn from_erosion_level(level: usize) -> Self {
        use crate::RegionType::*;
        match level % 3 {
            0 => Rocky,
            1 => Wet,
            2 => Narrow,
            _ => unreachable!(),
        }
    }

    fn find_risk_level(self) -> usize {
        use crate::RegionType::*;
        match self {
            Rocky => 0,
            Narrow => 2,
            Wet => 1,
        }
    }
}

impl fmt::Display for RegionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::RegionType::*;
        let c = match self {
            Rocky => '.',
            Wet => '=',
            Narrow => '|',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug)]
struct Cave {
    regions: Vec<Vec<RegionType>>,
    erosion_levels: Vec<Vec<usize>>,
    target: Point,
    depth: usize,
}

impl Cave {
    fn new(target: Point, depth: usize) -> Self {
        let mut cave = Self {
            regions: vec![],
            erosion_levels: vec![],
            target,
            depth,
        };
        cave.generate_regions(target.as_tuple());
        cave
    }

    fn from_input(input: &str) -> Self {
        let mut lines = input.trim().lines();
        let depth = lines
            .next()
            .and_then(|line| line.split(' ').nth(1))
            .unwrap();
        let depth = depth.parse().unwrap();
        let target = lines
            .next()
            .and_then(|line| line.split(' ').nth(1))
            .unwrap();
        let target = target
            .split(',')
            .map(|d| d.parse())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let target = Point {
            x: target[0],
            y: target[1],
        };
        Self::new(target, depth)
    }

    fn generate_regions(&mut self, new_bounds: (usize, usize)) {
        let (new_x_bound, new_y_bound) = new_bounds;
        let (current_x_bound, current_y_bound) = self.get_current_bounds();
        if new_x_bound < current_x_bound && new_y_bound < current_y_bound {
            return;
        }
        let x_bound = usize::max(new_x_bound, current_x_bound);
        let y_bound = usize::max(new_y_bound, current_y_bound);
        for y in 0..=y_bound {
            for x in 0..=x_bound {
                if y < current_y_bound && x < current_x_bound {
                    continue;
                }
                let coordinate = Point::from_tuple((x, y));
                let erosion_level = self.calculate_erosion_level(coordinate);
                let region_type = RegionType::from_erosion_level(erosion_level);
                if y < self.regions.len() {
                    self.regions[y].push(region_type);
                } else {
                    self.regions.push(vec![region_type])
                }
            }
        }
    }

    fn get_current_bounds(&self) -> (usize, usize) {
        let current_y_bound = self.regions.len();
        let current_x_bound = if current_y_bound == 0 {
            0
        } else {
            self.regions[0].len()
        };
        (current_x_bound, current_y_bound)
    }

    fn calculate_geologic_index(&self, coordinate: Point) -> usize {
        match coordinate.as_tuple() {
            (0, 0) => 0,
            c if c == self.target.as_tuple() => 0,
            (x, 0) => x * 16807,
            (0, y) => y * 48271,
            (x, y) => self.erosion_levels[y][x - 1] * self.erosion_levels[y - 1][x],
        }
    }

    fn calculate_erosion_level(&mut self, coordinate: Point) -> usize {
        let erosion_level = (self.calculate_geologic_index(coordinate) + self.depth) % 20183;
        if coordinate.y < self.erosion_levels.len() {
            self.erosion_levels[coordinate.y].push(erosion_level);
        } else {
            self.erosion_levels.push(vec![erosion_level]);
        }
        erosion_level
    }

    fn calculate_total_risk_level(&self) -> usize {
        self.regions
            .iter()
            .flatten()
            .map(|r| r.find_risk_level())
            .sum()
    }

    fn get_adjacent_regions(&mut self, coordinate: Point) -> Vec<(Point, RegionType)> {
        let (x, y) = coordinate.as_tuple();
        let (current_x_bound, current_y_bound) = self.get_current_bounds();
        if x + 1 >= current_x_bound || y + 1 >= current_y_bound {
            self.generate_regions((x + 1, y + 1));
        }
        [
            (Some(x), y.checked_sub(1)),
            (x.checked_sub(1), Some(y)),
            (Some(x + 1), Some(y)),
            (Some(x), Some(y + 1)),
        ]
        .into_iter()
        .filter_map(|c| match *c {
            (Some(x), Some(y)) => Some((Point::from_tuple((x, y)), self.regions[y][x])),
            _ => None,
        })
        .collect()
    }

    fn find_path_to_target(&mut self) -> (Vec<State>, usize) {
        let start_state = State {
            position: Point::from_tuple((0, 0)),
            equipped_tool: Tool::Torch,
        };
        let goal_state = State {
            position: self.target,
            equipped_tool: Tool::Torch,
        };
        a_star(start_state, goal_state, self).unwrap()
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (y, row) in self.regions.iter().enumerate() {
            for (x, r) in row.iter().enumerate() {
                let coordinate = (x, y);
                if coordinate == (0, 0) {
                    write!(f, "M")?;
                    continue;
                }
                if coordinate == self.target.as_tuple() {
                    write!(f, "T")?;
                    continue;
                }
                write!(f, "{}", r)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Tool {
    ClimbingGear,
    Torch,
    Neither,
}

impl Tool {
    fn is_allowed_in_region(self, region: RegionType) -> bool {
        use crate::RegionType::*;
        use crate::Tool::*;
        match region {
            Rocky => [ClimbingGear, Torch].contains(&self),
            Wet => [ClimbingGear, Neither].contains(&self),
            Narrow => [Torch, Neither].contains(&self),
        }
    }

    fn find_other_allowed_tool(self, current_region: RegionType) -> Self {
        use crate::Tool::*;
        *[ClimbingGear, Torch, Neither]
            .iter()
            .find(|t| **t != self && t.is_allowed_in_region(current_region))
            .unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
struct State {
    position: Point,
    equipped_tool: Tool,
}

impl State {
    fn next_states(&self, cave: &mut Cave) -> Vec<(State, usize)> {
        let (x, y) = self.position.as_tuple();
        let current_region = cave.regions[y][x];
        let tool_change_state = (
            State {
                position: self.position,
                equipped_tool: self.equipped_tool.find_other_allowed_tool(current_region),
            },
            TOOL_CHANGE_COST,
        );
        let adjacent_regions = cave.get_adjacent_regions(self.position);
        let move_states = adjacent_regions
            .iter()
            .filter(|(_, r)| self.equipped_tool.is_allowed_in_region(*r))
            .map(|&(c, _)| State {
                position: c,
                equipped_tool: self.equipped_tool,
            })
            .zip(iter::repeat(MOVE_COST));
        let mut result = vec![tool_change_state];
        result.extend(move_states);
        result
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Position: {:?} Tool: {:?}",
            self.position.as_tuple(),
            self.equipped_tool
        )
    }
}

#[derive(Debug)]
struct AStarWrapper {
    estimated_cost: usize,
    cost: usize,
    state: State,
}

impl Eq for AStarWrapper {}

impl PartialEq for AStarWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.estimated_cost.eq(&other.estimated_cost)
    }
}

impl Ord for AStarWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimated_cost.cmp(&self.estimated_cost) // Note this is reversed with other compared to self for min ordering
    }
}

impl PartialOrd for AStarWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn a_star(start: State, goal: State, cave: &mut Cave) -> Option<(Vec<State>, usize)> {
    let mut lowest_cost_to_state = HashMap::new();
    lowest_cost_to_state.insert(start, 0);
    let mut state_to_parent_state = HashMap::new();
    let mut unseen = BinaryHeap::new();
    unseen.push(AStarWrapper {
        estimated_cost: start.position.manhattan_distance_to(&goal.position),
        cost: 0,
        state: start,
    });
    while !unseen.is_empty() {
        let current = unseen.pop().unwrap();
        if current.state == goal {
            let states = reconstruct_states(&state_to_parent_state, current.state);
            return Some((states, current.cost));
        }
        if current.cost > lowest_cost_to_state[&current.state] {
            continue;
        }
        let next_states = current.state.next_states(cave);
        for (state, cost) in next_states {
            let full_cost = current.cost + cost;
            let prior_best_cost = lowest_cost_to_state.get(&state);
            if prior_best_cost.is_some() && *prior_best_cost.unwrap() <= full_cost {
                continue;
            }
            lowest_cost_to_state.insert(state, full_cost);
            state_to_parent_state.insert(state, current.state);
            unseen.push(AStarWrapper {
                estimated_cost: full_cost + state.position.manhattan_distance_to(&goal.position),
                cost: full_cost,
                state: state,
            })
        }
    }
    None
}

fn reconstruct_states(came_from: &HashMap<State, State>, current: State) -> Vec<State> {
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
    let mut cave = Cave::from_input(INPUT);
    println!("{}", cave.calculate_total_risk_level());
    let (states, time) = cave.find_path_to_target();
    // println!("{}", cave);
    // for state in states {
    //     println!("{}", state);
    // }
    println!("{}", time);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_solves_part_one_correctly() {
        let cave = Cave::new(Point::from_tuple((10, 10)), 510);
        assert_eq!(cave.calculate_total_risk_level(), 114);
    }

    #[test]
    fn it_solves_part_two_correctly() {
        let mut cave = Cave::new(Point::from_tuple((10, 10)), 510);
        assert_eq!(cave.find_path_to_target().1, 45);
    }
}
