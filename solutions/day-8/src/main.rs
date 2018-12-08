use day_8::{parse_input, solve_part_one, solve_part_two};
use std::num::ParseIntError;

const INPUT: &str = include_str!("../input");

fn main() -> Result<(), ParseIntError> {
    let parsed_input = parse_input(INPUT)?;
    println!("{}", solve_part_one(&parsed_input));
    println!("{}", solve_part_two(&parsed_input));
    Ok(())
}
