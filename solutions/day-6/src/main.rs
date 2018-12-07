use day_6::{find_part_one_solution, find_part_two_solution, parse_input};
use std::error::Error;

const INPUT: &str = include_str!("../input");
const MAX_DISTANCE: usize = 10_000;

fn main() -> Result<(), Box<dyn Error>> {
    let parsed_input = parse_input(INPUT)?;
    println!("{}", find_part_one_solution(&parsed_input));
    println!("{}", find_part_two_solution(&parsed_input, MAX_DISTANCE));
    Ok(())
}
