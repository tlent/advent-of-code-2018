use std::collections::HashMap;

const INPUT: &str = include_str!("../input");

type Position = (isize, isize);

fn find_door_counts(s: &str) -> HashMap<Position, usize> {
    let mut door_counts = HashMap::new();
    let mut stack = vec![vec![(0, (0, 0))]];
    for c in s.chars() {
        match c {
            '(' => {
                let current_group = stack.last().unwrap();
                let &(_, start_position) = current_group.last().unwrap();
                let new_group = vec![(0, start_position)];
                stack.push(new_group);
            }
            '|' => {
                let previous_group = &stack[stack.len() - 2];
                let &(_, start_position) = previous_group.last().unwrap();
                let current_group = stack.last_mut().unwrap();
                current_group.push((0, start_position));
            }
            ')' => {
                let popped_group = stack.pop().unwrap();
                if popped_group.iter().any(|(count, _)| *count == 0) {
                    continue;
                }
                let (count, position) = popped_group.iter().max_by_key(|(count, _)| count).unwrap();
                let current_group = stack.last_mut().unwrap();
                let (ref mut current_count, ref mut current_position) =
                    current_group.last_mut().unwrap();
                *current_count += *count;
                *current_position = *position;
            }
            d if "NESW".contains(d) => {
                let current_group = stack.last_mut().unwrap();
                let (ref mut current_count, ref mut current_position) =
                    current_group.last_mut().unwrap();
                *current_count += 1;
                let (x, y) = *current_position;
                let position = match d {
                    'N' => (x, y - 1),
                    'E' => (x + 1, y),
                    'S' => (x, y + 1),
                    'W' => (x - 1, y),
                    _ => unreachable!(),
                };
                *current_position = position;
                let total_count = stack
                    .iter()
                    .map(|s| {
                        let (count, _) = s.last().unwrap();
                        count
                    })
                    .sum::<usize>();
                let previous_count = door_counts.get(&position);
                if previous_count.is_none() || total_count < *previous_count.unwrap() {
                    door_counts.insert(position, total_count);
                }
            }
            _ => panic!("invalid char"),
        }
    }
    door_counts
}

fn solve_part_one(regex: &str) -> usize {
    let door_counts = find_door_counts(&regex[1..regex.len() - 1]);
    *door_counts.values().max().unwrap()
}

fn solve_part_two(regex: &str) -> usize {
    let door_counts = find_door_counts(&regex[1..regex.len() - 1]);
    door_counts.values().filter(|v| **v >= 1000).count()
}

fn main() {
    let input = INPUT.trim();
    println!("{}", solve_part_one(input));
    println!("{}", solve_part_two(input));
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLES: [(&str, usize); 5] = [
        ("^WNE$", 3),
        ("^ENWWW(NEEE|SSE(EE|N))$", 10),
        ("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$", 18),
        ("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$", 23),
        (
            "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$",
            31,
        ),
    ];

    #[test]
    fn it_solves_first_sample_correctly() {
        let (input, expected) = SAMPLES[0];
        assert_eq!(solve_part_one(input), expected);
    }

    #[test]
    fn it_solves_second_sample_correctly() {
        let (input, expected) = SAMPLES[1];
        assert_eq!(solve_part_one(input), expected);
    }

    #[test]
    fn it_solves_third_sample_correctly() {
        let (input, expected) = SAMPLES[2];
        assert_eq!(solve_part_one(input), expected);
    }

    #[test]
    fn it_solves_fourth_sample_correctly() {
        let (input, expected) = SAMPLES[3];
        assert_eq!(solve_part_one(input), expected);
    }

    #[test]
    fn it_solves_fifth_sample_correctly() {
        let (input, expected) = SAMPLES[4];
        assert_eq!(solve_part_one(input), expected);
    }

    #[test]
    fn it_solves_many_option_branches_correctly() {
        let input = "^NNN(N|E|SSSS)W$";
        assert_eq!(solve_part_one(input), 8);
    }
}
