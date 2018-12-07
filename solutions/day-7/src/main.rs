use day_7::{parse_input, solve_part_one, solve_part_two};
// use std::error::Error;

const INPUT: &str = include_str!("../input");
const WORKERS: usize = 5;
const BASE_TIME: u32 = 60;

fn main() {
    let parsed_input = parse_input(INPUT);
    println!("{}", solve_part_one(&parsed_input));
    println!("{}", solve_part_two(&parsed_input, WORKERS, BASE_TIME));
}
