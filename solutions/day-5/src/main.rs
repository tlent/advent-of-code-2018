const INPUT: &str = include_str!("../input");

fn main() {
    let parsed_input = parse_input(INPUT);
    println!("{}", find_part_one_solution(parsed_input));
    println!("{}", find_part_two_solution(parsed_input));
}

pub fn parse_input(input: &str) -> &str {
    input.trim()
}

pub fn find_part_one_solution(polymer: &str) -> usize {
    let final_polymer = fully_react_polymer(polymer);
    final_polymer.len()
}

pub fn find_part_two_solution(polymer: &str) -> usize {
    (b'a'..b'z')
        .map(|removed_unit| {
            fully_react_polymer(
                &polymer.replace(|c: char| c.to_ascii_lowercase() == removed_unit as char, ""),
            )
            .len()
        })
        .min()
        .unwrap()
}

fn fully_react_polymer(polymer: &str) -> String {
    let mut current_polymer = String::from(polymer);
    let mut previous_polymer: Option<String> = None;
    while previous_polymer.is_none() || *previous_polymer.unwrap() != current_polymer {
        previous_polymer = Some(current_polymer.clone());
        current_polymer = current_polymer
            .chars()
            .fold(String::new(), |mut result, unit| {
                match result.chars().last() {
                    Some(previous_unit) if is_reacting_pair(unit, previous_unit) => {
                        result.pop();
                    }
                    _ => {
                        result.push(unit);
                    }
                };
                result
            });
    }
    current_polymer
}

fn is_reacting_pair(a: char, b: char) -> bool {
    a != b && a.to_ascii_lowercase() == b.to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_removes_newline() {
        assert_eq!(parse_input("aA\n").len(), 2);
    }

    #[test]
    fn it_finds_correct_part_one_solution() {
        let sample_input = "dabAcCaCBAcCcaDA";
        assert_eq!(find_part_one_solution(sample_input), 10);
    }

    #[test]
    fn it_finds_correct_part_two_solution() {
        let sample_input = "dabAcCaCBAcCcaDA";
        assert_eq!(find_part_two_solution(sample_input), 4);
    }
}
