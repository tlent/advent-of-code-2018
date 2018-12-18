use std::fmt;

const INPUT: &str = include_str!("../input");
const PART_ONE_MINUTES: usize = 10;
const PART_TWO_MINUTES: usize = 1_000_000_000;

#[derive(Debug, Clone, PartialEq)]
struct World {
  acres: Vec<Vec<Acre>>,
}

impl World {
  fn from_input(input: &str) -> Self {
    let mut acres = vec![];
    for line in input.trim().lines() {
      acres.push(line.chars().map(Acre::from_char).collect::<Vec<_>>())
    }
    Self { acres }
  }

  fn get_adjacent_acres(&self, (x, y): (usize, usize)) -> Vec<&Acre> {
    [
      (x.checked_sub(1), y.checked_sub(1)),
      (x.checked_sub(1), Some(y)),
      (x.checked_sub(1), y.checked_add(1)),
      (Some(x), y.checked_sub(1)),
      (Some(x), y.checked_add(1)),
      (x.checked_add(1), y.checked_sub(1)),
      (x.checked_add(1), Some(y)),
      (x.checked_add(1), y.checked_add(1)),
    ]
    .iter()
    .filter_map(|&p| match p {
      (Some(x), Some(y)) if x < self.acres[0].len() && y < self.acres.len() => {
        Some(&self.acres[y][x])
      }
      _ => None,
    })
    .collect()
  }

  fn tick(&mut self) {
    let initial_world = self.clone();
    for (i, row) in self.acres.iter_mut().enumerate() {
      for (j, acre) in row.iter_mut().enumerate() {
        let adjacent_acres = initial_world.get_adjacent_acres((j, i));
        acre.tick(&adjacent_acres);
      }
    }
  }

  fn simulate(&mut self, minutes: usize) {
    println!("Initial state:\n{}", self);
    let mut prev_states = vec![self.clone()];
    for current_minute in 1..=minutes {
      self.tick();
      println!(
        "After {} minute{}:\n{}",
        current_minute,
        if current_minute > 1 { "s" } else { "" },
        self
      );
      if prev_states.contains(&self) {
        let cycle_start_index = prev_states.iter().position(|v| v == self).unwrap();
        let cycle = &prev_states[cycle_start_index..];
        let remaining_minutes = minutes - current_minute;
        let final_state = &cycle[remaining_minutes % cycle.len()];
        *self = final_state.clone();
        println!("Cycle at minute {}", current_minute);
        println!(
          "After {} minute{}:\n{}",
          minutes,
          if minutes > 1 { "s" } else { "" },
          self
        );
        return;
      }
      prev_states.push(self.clone());
    }
  }

  fn get_resource_value(&self) -> usize {
    let acres: Vec<_> = self.acres.iter().flatten().collect();
    let wooded_count = count_wooded(&acres);
    let lumberyard_count = count_lumberyards(&acres);
    wooded_count * lumberyard_count
  }
}

impl fmt::Display for World {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for row in self.acres.iter() {
      for acre in row.iter() {
        acre.fmt(f)?;
      }
      writeln!(f)?;
    }
    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Acre {
  Open,
  Wooded,
  Lumberyard,
}

impl Acre {
  fn from_char(c: char) -> Self {
    match c {
      '.' => Acre::Open,
      '|' => Acre::Wooded,
      '#' => Acre::Lumberyard,
      _ => panic!("Invalid acre char: {}", c),
    }
  }

  fn to_char(&self) -> char {
    match self {
      Acre::Open => '.',
      Acre::Wooded => '|',
      Acre::Lumberyard => '#',
    }
  }

  fn tick(&mut self, adjacent_acres: &[&Acre]) {
    match self {
      Acre::Open => {
        let adjacent_wooded_count = count_wooded(adjacent_acres);
        if adjacent_wooded_count >= 3 {
          *self = Acre::Wooded;
        }
      }
      Acre::Wooded => {
        let adjacent_lumberyard_count = count_lumberyards(adjacent_acres);
        if adjacent_lumberyard_count >= 3 {
          *self = Acre::Lumberyard;
        }
      }
      Acre::Lumberyard => {
        let adjacent_lumberyard_count = count_lumberyards(adjacent_acres);
        let adjacent_wooded_count = count_wooded(adjacent_acres);
        if adjacent_lumberyard_count < 1 || adjacent_wooded_count < 1 {
          *self = Acre::Open;
        }
      }
    }
  }
}

impl fmt::Display for Acre {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_char())
  }
}

fn count_lumberyards(acres: &[&Acre]) -> usize {
  acres.iter().filter(|&a| **a == Acre::Lumberyard).count()
}

fn count_wooded(acres: &[&Acre]) -> usize {
  acres.iter().filter(|&a| **a == Acre::Wooded).count()
}

fn main() {
  let world = World::from_input(INPUT);
  let mut part_one_world = world.clone();
  part_one_world.simulate(PART_ONE_MINUTES);
  println!("{}", part_one_world.get_resource_value());
  let mut part_two_world = world.clone();
  part_two_world.simulate(PART_TWO_MINUTES);
  println!("{}", part_two_world.get_resource_value());
}

#[cfg(test)]
mod test {
  use super::*;

  const SAMPLE_INPUT: &str = include_str!("../sample-input");
  const SAMPLE_SOLUTION: usize = 1147;
  const PART_ONE_SOLUTION: usize = 480150;
  const PART_TWO_SOLUTION: usize = 233020;

  #[test]
  fn it_parses_input_correctly() {
    // relies on correct Display implementations
    let world = World::from_input(SAMPLE_INPUT);
    assert_eq!(format!("{}", world), SAMPLE_INPUT);

    let world = World::from_input(INPUT);
    assert_eq!(format!("{}", world), INPUT);
  }

  #[test]
  fn it_solves_part_one_correctly() {
    let mut world = World::from_input(SAMPLE_INPUT);
    world.simulate(PART_ONE_MINUTES);
    assert_eq!(world.get_resource_value(), SAMPLE_SOLUTION);

    let mut world = World::from_input(INPUT);
    world.simulate(PART_ONE_MINUTES);
    assert_eq!(world.get_resource_value(), PART_ONE_SOLUTION);
  }

  #[test]
  fn it_solves_part_two_correctly() {
    let mut world = World::from_input(INPUT);
    world.simulate(PART_TWO_MINUTES);
    assert_eq!(world.get_resource_value(), PART_TWO_SOLUTION);
  }
}
