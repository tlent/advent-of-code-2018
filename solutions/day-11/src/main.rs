use day_11::solve;

const INPUT: u32 = 5034;
const FUEL_CELL_GRID_SIZE: usize = 300;
const PART_ONE_SQUARE_SIZE: usize = 3;

fn main() {
  let part_one_solution = solve_part_one(INPUT);
  println!("{},{}", part_one_solution.0, part_one_solution.1);
  let (c, size) = solve_part_two(INPUT);
  println!("{},{},{}", c.0, c.1, size);
}

fn solve_part_one(serial_number: u32) -> (usize, usize) {
  solve(
    FUEL_CELL_GRID_SIZE,
    serial_number,
    PART_ONE_SQUARE_SIZE,
    PART_ONE_SQUARE_SIZE,
  )
  .0
}

fn solve_part_two(serial_number: u32) -> ((usize, usize), usize) {
  solve(
    FUEL_CELL_GRID_SIZE,
    serial_number,
    1,
    FUEL_CELL_GRID_SIZE - 1,
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  const PART_ONE_SAMPLE_SERIAL_NUMBER: u32 = 42;
  const PART_TWO_SAMPLE_SERIAL_NUMBERS: [u32; 2] = [18, 42];

  #[test]
  fn it_solves_part_one_correctly() {
    assert_eq!(solve_part_one(PART_ONE_SAMPLE_SERIAL_NUMBER), (21, 61));
  }

  #[test]
  #[ignore]
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
}
