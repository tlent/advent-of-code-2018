use day_9::{parse_input, solve_with_deque as solve, Result};

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let parsed_input = parse_input(INPUT)?;
    println!("{}", solve(&parsed_input));
    let part_two_input = (parsed_input.0, parsed_input.1 * 100);
    println!("{}", solve(&part_two_input));
    Ok(())
}
