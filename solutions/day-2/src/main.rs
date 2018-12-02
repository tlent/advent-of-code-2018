extern crate day_2;

pub const INPUT: &'static str = include_str!("../input");

use day_2::{calculate_checksum, find_similar_id_match, parse_input};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let ids = parse_input(INPUT);
    let result = calculate_checksum(&ids);
    println!("{}", result);
    let result = find_similar_id_match(&ids);
    println!("{}", result);
    Ok(())
}
