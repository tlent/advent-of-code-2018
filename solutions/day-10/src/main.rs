use day_10::{parse_input, solve};
use std::error::Error;

const INPUT: &str = include_str!("../input");

fn main() -> Result<(), Box<dyn Error>> {
    let points = parse_input(INPUT)?;
    let solution = solve(&points);
    println!("{}{}", solution.0, solution.1);
    Ok(())
}
