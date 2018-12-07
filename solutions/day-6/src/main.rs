use day_6::{find_part_one_solution, find_part_two_solution, parse_input};
use std::error::Error;

const INPUT: &str = include_str!("../input");

fn main() -> Result<(), Box<dyn Error>> {
    let parsed_input = parse_input(INPUT).expect("Failed parsing input");
    println!("{}", find_part_one_solution(&parsed_input));
    // println!("{}", find_part_two_solution(parsed_input));
    Ok(())
}
