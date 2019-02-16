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

fn calculate_power_level(coordinate: (u32, u32), serial_number: u32) -> i32 {
    let (x, y) = coordinate;
    let rack_id = x + 10;
    (((rack_id * y + serial_number) * rack_id / 100) % 10) as i32 - 5
}

fn solve(
    grid_size: usize,
    serial_number: u32,
    min_square_size: usize,
    max_square_size: usize,
) -> ((usize, usize), usize) {
    if max_square_size >= grid_size {
        panic!("max square size must be less than grid size");
    }
    let power_levels: Vec<Vec<_>> = (1..=grid_size as u32)
        .map(|y| {
            (1..=grid_size as u32)
                .map(|x| calculate_power_level((x, y), serial_number))
                .collect()
        })
        .collect();
    let transposed_power_levels: Vec<Vec<_>> = (0..power_levels[0].len())
        .map(|x| {
            (0..power_levels.len())
                .map(|y| power_levels[y][x])
                .collect()
        })
        .collect();
    let mut results = Vec::with_capacity(max_square_size - min_square_size + 1);
    let mut square_power_levels = power_levels.clone();
    for square_size in 1..=max_square_size {
        let max_index = grid_size - square_size;
        let mut coordinates = Vec::with_capacity(max_index * max_index);
        (0..max_index).for_each(|i| (0..max_index).for_each(|j| coordinates.push((i, j))));
        if square_size >= min_square_size {
            let max_result = coordinates
                .iter()
                .map(|&(i, j)| {
                    let y = i + 1;
                    let x = j + 1;
                    (square_power_levels[i][j], (x, y), square_size)
                })
                .max_by_key(|&(power_level, ..)| power_level)
                .unwrap();
            results.push(max_result);
        }
        coordinates.iter().for_each(|&(i, j)| {
            let max_i = i + square_size;
            let max_j = j + square_size;
            // only the row range is inclusive to avoid counting the bottom right cell twice
            square_power_levels[i][j] += power_levels[max_i][j..=max_j].iter().sum::<i32>()
                + transposed_power_levels[max_j][i..max_i].iter().sum::<i32>();
        });
    }
    results
        .into_iter()
        .max_by_key(|&(power_level, ..)| power_level)
        .map(|(_power_level, coordinate, square_size)| (coordinate, square_size))
        .unwrap()
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

    #[test]
    fn it_finds_correct_cell_power_levels() {
        for (i, (cell, serial_number, expected_power_level)) in
            get_sample_cell_power_levels().iter().enumerate()
        {
            assert_eq!(
                calculate_power_level(*cell, *serial_number),
                *expected_power_level,
                "failed for input #{} with cell: {:?} serial number: {}",
                i + 1,
                cell,
                serial_number
            );
        }
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
