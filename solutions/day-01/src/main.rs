use std::collections::HashSet;

const INPUT: &str = include_str!("../input");

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let numbers = parse_input(INPUT)?;
    println!("{}", sum(&numbers));
    println!("{}", find_repeat_result(&numbers));
    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<i32>> {
    input
        .split_whitespace()
        .map(|x| x.parse().map_err(Box::from))
        .collect()
}

fn sum(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

fn find_repeat_result(numbers: &[i32]) -> i32 {
    let mut past_results = HashSet::new();
    let mut result = 0;
    past_results.insert(result);
    loop {
        for number in numbers {
            result += number;
            let is_repeat = !past_results.insert(result);
            if is_repeat {
                return result;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_returns_correct_result() {
        assert_eq!(sum(&[2, 2]), 4);
        assert_eq!(sum(&[2, 2, -4, 0, 1]), 1);
    }

    #[test]
    fn find_repeat_result_returns_correct_result() {
        assert_eq!(find_repeat_result(&[1, -1]), 0);
        assert_eq!(find_repeat_result(&[3, 3, 4, -2, -4]), 10);
        assert_eq!(find_repeat_result(&[-6, 3, 8, 5, -6]), 5);
        assert_eq!(find_repeat_result(&[7, 7, -2, -7, -4]), 14);
    }
}
