use regex::Regex;
use std::collections::HashMap;
use std::fmt;

const INPUT: &str = include_str!("../input");

#[derive(Debug, PartialEq, Eq, Clone)]
enum Tile {
    Clay,
    Water,
    Flow,
    Spring,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Tile::Clay => '#',
            Tile::Water => '~',
            Tile::Spring => '+',
            Tile::Flow => '|',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone)]
struct World {
    filled_tiles: HashMap<(u32, u32), Tile>,
}

impl World {
    fn from_input(input: &str) -> Self {
        let regex = Regex::new(r"(x|y)=(\d+), (x|y)=(\d+)..(\d+)").unwrap();
        let mut filled_tiles = HashMap::new();
        filled_tiles.insert((500, 0), Tile::Spring);
        for line in input.trim().lines() {
            let caps = regex.captures(line).unwrap();
            let first_value: u32 = caps[2].parse().unwrap();
            let range_start: u32 = caps[4].parse().unwrap();
            let range_end: u32 = caps[5].parse().unwrap();
            let new_clay_tile_coords: Vec<_> = match (&caps[1], &caps[3]) {
                ("x", "y") => {
                    let x = first_value;
                    (range_start..=range_end).map(|y| (x, y)).collect()
                }
                ("y", "x") => {
                    let y = first_value;
                    (range_start..=range_end).map(|x| (x, y)).collect()
                }
                _ => panic!("Doesn't match x,y or y,x: {}", line),
            };
            filled_tiles.extend(new_clay_tile_coords.into_iter().map(|c| (c, Tile::Clay)));
        }
        Self { filled_tiles }
    }

    fn get_clay_bounds(&self) -> (u32, u32) {
        let clay_tile_y_coordinates: Vec<_> = self
            .filled_tiles
            .iter()
            .filter(|(_, t)| **t == Tile::Clay)
            .map(|((_x, y), _)| y)
            .collect();
        let min_bound = clay_tile_y_coordinates.iter().min().unwrap();
        let max_bound = clay_tile_y_coordinates.iter().max().unwrap();
        (**min_bound, **max_bound)
    }

    fn simulate_water(&mut self) {
        let (spring_x, spring_y) = *self
            .filled_tiles
            .iter()
            .find(|(_, t)| **t == Tile::Spring)
            .map(|(c, _)| c)
            .unwrap();
        let (_, max_y) = self.get_clay_bounds();
        let start = (spring_x, spring_y + 1);
        let mut downward_stack = vec![start];
        let mut downward_history = vec![start];
        while !downward_stack.is_empty() {
            let current_position = downward_stack.pop().unwrap();
            let (current_x, current_y) = current_position;
            self.filled_tiles.insert(current_position, Tile::Flow);
            if current_y == max_y {
                downward_history.clear();
                continue;
            }
            let downward_position = (current_x, current_y + 1);
            if self.filled_tiles.get(&downward_position).is_none() {
                downward_stack.push(downward_position);
                downward_history.push(current_position);
                continue;
            }
            let mut horizontal_stack = vec![current_position];
            while !horizontal_stack.is_empty() {
                let current_position = horizontal_stack.pop().unwrap();
                let (current_x, current_y) = current_position;
                self.filled_tiles.insert(current_position, Tile::Flow);
                let downward_position = (current_x, current_y + 1);
                if self.filled_tiles.get(&downward_position).is_none() {
                    downward_stack.push(downward_position);
                    downward_history.push(current_position);
                    continue;
                }
                let valid_horizontal: Vec<_> =
                    vec![(current_x - 1, current_y), (current_x + 1, current_y)]
                        .into_iter()
                        .filter(|p| {
                            self.filled_tiles.get(p).is_none() && self.is_valid_position(*p)
                        })
                        .collect();
                if !valid_horizontal.is_empty() {
                    horizontal_stack.extend(valid_horizontal.iter());
                }
            }
            let has_settled = self.settle_around(current_position);
            if has_settled && !downward_history.is_empty() {
                downward_stack.push(downward_history.pop().unwrap());
            }
        }
    }

    fn is_valid_position(&self, p: (u32, u32)) -> bool {
        let below_position = (p.0, p.1 + 1);
        let below_tile = self.filled_tiles.get(&below_position);
        match below_tile {
            Some(Tile::Flow) => false,
            _ => true,
        }
    }

    fn settle_around(&mut self, position: (u32, u32)) -> bool {
        fn find_flow_to_clay<I>(world: &World, x_values: I, y: u32) -> Option<Vec<(u32, u32)>>
        where
            I: IntoIterator<Item = u32>,
        {
            let mut flows = vec![];
            for x in x_values {
                let current_position = (x, y);
                let current_tile = world.filled_tiles.get(&current_position);
                if current_tile.is_none() {
                    return None;
                }
                let current_tile = current_tile.unwrap();
                match current_tile {
                    Tile::Flow => flows.push(current_position),
                    Tile::Water | Tile::Spring => continue,
                    Tile::Clay => break,
                };
            }
            Some(flows)
        }
        let left_flows = find_flow_to_clay(self, (0..position.0).rev(), position.1);
        let right_flows = find_flow_to_clay(self, position.0.., position.1);
        if left_flows.is_none() || right_flows.is_none() {
            return false;
        }
        let left_flows = left_flows.unwrap();
        let right_flows = right_flows.unwrap();
        for flow in left_flows.iter().chain(right_flows.iter()) {
            self.filled_tiles.insert(*flow, Tile::Water);
        }
        true
    }

    fn count_water_and_flow(&self) -> u32 {
        let (min_clay_bound, max_clay_bound) = self.get_clay_bounds();
        self.filled_tiles
            .iter()
            .filter(|((_x, y), t)| {
                *y >= min_clay_bound
                    && *y <= max_clay_bound
                    && (**t == Tile::Water || **t == Tile::Flow)
            })
            .count() as u32
    }

    fn count_water(&self) -> u32 {
        let (min_clay_bound, max_clay_bound) = self.get_clay_bounds();
        self.filled_tiles
            .iter()
            .filter(|((_x, y), t)| {
                *y >= min_clay_bound && *y <= max_clay_bound && (**t == Tile::Water)
            })
            .count() as u32
    }

    fn get_display_bounds(&self) -> ((u32, u32), (u32, u32)) {
        let tile_x_coordinates: Vec<_> = self.filled_tiles.iter().map(|((x, _y), _)| x).collect();
        let min_x = tile_x_coordinates.iter().min().unwrap();
        let max_x = tile_x_coordinates.iter().max().unwrap();
        let tile_y_coordinates: Vec<_> = self.filled_tiles.iter().map(|((_x, y), _)| y).collect();
        let min_y = tile_y_coordinates.iter().min().unwrap();
        let max_y = tile_y_coordinates.iter().max().unwrap();
        ((**min_x, **max_x), (**min_y, **max_y))
    }

    fn area_to_string(&self, x_bounds: (u32, u32), y_bounds: (u32, u32)) -> String {
        let (min_x, max_x) = x_bounds;
        let (min_y, max_y) = y_bounds;
        let mut s = String::new();
        // used to show an extra column of tiles except at the edge of possible coordinates
        // this shows there is no water or anything in those tiles
        let display_min_x = min_x.saturating_sub(1);
        let display_max_x = max_x.saturating_add(1);
        for y in min_y..=max_y {
            for x in display_min_x..=display_max_x {
                let tile = self.filled_tiles.get(&(x, y));
                if tile.is_some() {
                    s.push_str(&format!("{}", tile.unwrap()));
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }
        s
    }

    #[allow(dead_code)]
    fn print_area_around(&self, position: (u32, u32)) {
        let (x, y) = position;
        println!(
            "{}",
            self.area_to_string(
                (x.saturating_sub(10), x.saturating_add(10)),
                (y.saturating_sub(10), y.saturating_add(10)),
            )
        )
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x_bounds, y_bounds) = self.get_display_bounds();
        write!(f, "{}", self.area_to_string(x_bounds, y_bounds))?;
        Ok(())
    }
}

fn main() {
    let mut world = World::from_input(INPUT);
    world.simulate_water();
    println!("{}", world);
    println!("{}", world.count_water_and_flow());
    println!("{}", world.count_water());
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample-input");
    const SAMPLE_WORLD: &str = include_str!("../sample-world");

    #[test]
    fn it_parses_sample_correctly() {
        let world = World::from_input(SAMPLE_INPUT);
        println!("{}", world);
        assert_eq!(format!("{}", world), SAMPLE_WORLD);
    }

    #[test]
    fn it_solves_part_one_sample_correctly() {
        let mut world = World::from_input(SAMPLE_INPUT);
        world.simulate_water();
        println!("{}", world);
        assert_eq!(world.count_water_and_flow(), 57);
    }

    #[test]
    fn it_solves_part_two_sample_correctly() {
        let mut world = World::from_input(SAMPLE_INPUT);
        world.simulate_water();
        println!("{}", world);
        assert_eq!(world.count_water(), 29);
    }

    #[test]
    fn it_solves_part_one_real_correctly() {
        let mut world = World::from_input(INPUT);
        world.simulate_water();
        println!("{}", world);
        assert_eq!(world.count_water_and_flow(), 31412);
    }

    #[test]
    fn it_solves_part_two_real_correctly() {
        let mut world = World::from_input(INPUT);
        world.simulate_water();
        println!("{}", world);
        assert_eq!(world.count_water(), 25857);
    }
}
