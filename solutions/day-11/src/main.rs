const INPUT: u32 = 5034;
const FUEL_CELL_GRID_SIZE: u32 = 300;
const PART_ONE_SQUARE_SIZE: u32 = 3;

fn find_power_level(coordinate: (u32, u32), serial_number: u32) -> i32 {
  let rack_id = coordinate.0 + 10;
  (((rack_id * coordinate.1 + serial_number) * rack_id / 100) % 10) as i32 - 5
}

fn find_square_power_level(
  top_left_coordinate: (u32, u32),
  serial_number: u32,
  square_size: u32,
) -> i32 {
  let mut power_level = 0;
  for y in top_left_coordinate.1..top_left_coordinate.1 + square_size {
    for x in top_left_coordinate.0..top_left_coordinate.0 + square_size {
      power_level += find_power_level((x, y), serial_number);
    }
  }
  power_level
}

fn solve(serial_number: u32, square_size: u32) -> ((u32, u32), i32) {
  let mut possible_coordinates = vec![];
  for start_y in 1..=(FUEL_CELL_GRID_SIZE - square_size) {
    for start_x in 1..=(FUEL_CELL_GRID_SIZE - square_size) {
      possible_coordinates.push((start_x, start_y));
    }
  }
  possible_coordinates
    .iter()
    .map(|c| (*c, find_square_power_level(*c, serial_number, square_size)))
    .max_by_key(|c| c.1)
    .unwrap()
}

fn solve_part_one(serial_number: u32) -> (u32, u32) {
  solve(serial_number, PART_ONE_SQUARE_SIZE).0
}

fn solve_part_two(serial_number: u32) -> ((u32, u32), u32) {
  (1..FUEL_CELL_GRID_SIZE)
    .map(|size| {
      let (c, power_level) = solve(serial_number, size);
      (power_level, c, size)
    })
    .max_by_key(|(power_level, ..)| *power_level)
    .map(|(_power_level, c, size)| (c, size))
    .unwrap()
}

fn main() {
  let part_one_solution = solve_part_one(INPUT);
  println!("{},{}", part_one_solution.0, part_one_solution.1);
  let (c, size) = solve_part_two(INPUT);
  println!("{},{},{}", c.0, c.1, size);
}

#[cfg(test)]
mod tests {
  use super::*;

  const PART_ONE_SAMPLE_SERIAL_NUMBER: u32 = 42;
  const PART_TWO_SAMPLE_SERIAL_NUMBERS: [u32; 2] = [18, 42];

  #[test]
  fn it_finds_correct_cell_power_levels() {
    for (i, (cell, serial_number, expected_power_level)) in
      get_sample_cell_power_levels().iter().enumerate()
    {
      assert_eq!(
        find_power_level(*cell, *serial_number),
        *expected_power_level,
        "failed for input #{} with cell: {:?} serial number: {}",
        i + 1,
        cell,
        serial_number
      );
    }
  }

  #[test]
  fn it_solves_part_one_correctly() {
    assert_eq!(solve_part_one(PART_ONE_SAMPLE_SERIAL_NUMBER), (21, 61));
  }

  #[test]
  fn it_solves_part_two_correctly() {
    assert_eq!(
      solve_part_two(PART_TWO_SAMPLE_SERIAL_NUMBERS[0]),
      ((90, 269), 16)
    );
    assert_eq!(
      solve_part_two(PART_TWO_SAMPLE_SERIAL_NUMBERS[1]),
      ((232, 251), 12)
    );
  }

  /// Returns ((x, y), serial_number, expected_power_level)
  fn get_sample_cell_power_levels() -> [((u32, u32), u32, i32); 4] {
    [
      ((3, 5), 8, 4),
      ((122, 79), 57, -5),
      ((217, 196), 39, 0),
      ((101, 153), 71, 4),
    ]
  }
}
