extern crate day_5;

use day_5::{find_part_one_solution, find_part_two_solution, parse_input};

const INPUT: &'static str = include_str!("../input");

fn main() {
    let parsed_input = parse_input(INPUT);
    println!("{}", find_part_one_solution(parsed_input));
    println!("{}", find_part_two_solution(parsed_input));
}
