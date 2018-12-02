extern crate day_1;
extern crate failure;

use day_1::{find_repeat_result, parse_input, sum};
use failure::Error;

pub const INPUT: &'static str = include_str!("../input");

fn main() -> Result<(), Error> {
    let numbers = parse_input(INPUT)?;
    let result = sum(&numbers);
    println!("{}", result);
    let result = find_repeat_result(&numbers);
    println!("{}", result);
    Ok(())
}
