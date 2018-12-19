extern crate day_3;

pub const INPUT: &'static str = include_str!("../input");

use day_3::{count_overlapping_fabric_claim_units, find_fabric_claim_with_no_overlap, parse_input};

fn main() {
    let parsed_input = parse_input(INPUT);
    println!("{}", count_overlapping_fabric_claim_units(&parsed_input));
    println!("{}", find_fabric_claim_with_no_overlap(&parsed_input).id);
}
