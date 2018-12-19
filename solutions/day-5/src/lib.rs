// Spent hours not realizing the input must be trimmed or the \n at the end will add 1 to the result
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
        .expect("No solution found")
}

fn fully_react_polymer(polymer: &str) -> String {
    let mut current_polymer = polymer.to_owned();
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

// My original working solution before improving with fold and looking back instead of peeking
#[allow(dead_code)]
fn find_part_one_solution_peek(units: &str) -> usize {
    let mut current_units = units.to_owned();
    let mut previous_units: Option<String> = None;
    while previous_units.is_none() || *previous_units.unwrap() != current_units {
        previous_units = Some(current_units.clone());
        let mut result = String::new();
        {
            let mut iter = current_units.chars().peekable();
            while let Some(c) = iter.next() {
                if iter.peek().is_some() && is_reacting_pair(c, *iter.peek().unwrap()) {
                    iter.next(); // do nothing with c and skip next c
                } else {
                    result.push(c);
                }
            }
        }
        current_units = result;
    }
    current_units.len()
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
