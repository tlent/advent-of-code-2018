use std::collections::HashSet;

const INPUT: &str = include_str!("../input");
const GENERATIONS: u32 = 20;

fn parse_input(input: &str) -> (Vec<u32>, HashSet<Vec<bool>>) {
  let mut lines = input.trim().lines();
  let first_line = lines.nth(0).unwrap();
  let initial_state: Vec<_> = first_line[15..]
    .chars()
    .enumerate()
    .filter(|&(_i, c)| c == '#')
    .map(|(i, _c)| i as u32)
    .collect();
  assert!(lines.next().unwrap().is_empty());
  let rules = lines.fold(HashSet::new(), |mut rules, line| {
    if line.chars().nth(9).unwrap() == '#' {
      let rule: Vec<bool> = line.chars().take(5).map(|c| c == '#').collect();
      rules.insert(rule);
    }
    rules
  });
  (initial_state, rules)
}

fn solve_part_one(initial_state: &[u32], rules: &HashSet<Vec<bool>>) -> i32 {
  let mut current_state: Vec<_> = initial_state.iter().map(|&x| x as i32).collect();
  for _ in 1..=GENERATIONS {
    let mut next_state = vec![];
    let current_min_pot_number = current_state[0];
    let current_max_pot_number = current_state[current_state.len() - 1];
    for pot_number in (current_min_pot_number - 2)..=(current_max_pot_number + 2) {
      let sequence: Vec<_> = ((pot_number - 2)..=(pot_number + 2))
        .map(|i| {
          if i < current_min_pot_number || i > current_max_pot_number {
            false
          } else {
            current_state.binary_search(&i).is_ok()
          }
        })
        .collect();
      let has_plant = rules.contains(&sequence);
      if has_plant {
        next_state.push(pot_number);
      }
    }
    current_state = next_state;
  }
  current_state.iter().sum()
}

fn main() {
  let (initial_state, rules) = parse_input(INPUT);
  println!("{}", solve_part_one(&initial_state, &rules));
}

#[cfg(test)]
mod test {
  use super::*;

  const SAMPLE_INPUT: &str = include_str!("../sample-input");

  #[test]
  fn it_parses_input_correctly() {
    let (parsed_initial_state, parsed_rules) = parse_input(SAMPLE_INPUT);
    let (sample_initial_state, sample_rules) = get_sample_input();
    assert_eq!(parsed_initial_state, sample_initial_state);
    assert_eq!(parsed_rules, sample_rules);
  }

  #[test]
  fn it_solves_part_one_correctly() {
    let (initial_state, rules) = get_sample_input();
    assert_eq!(solve_part_one(&initial_state, &rules), 325);
  }

  fn get_sample_input() -> (Vec<u32>, HashSet<Vec<bool>>) {
    let mut rules = HashSet::new();
    rules.extend(
      vec![
        vec![false, false, false, true, true],
        vec![false, false, true, false, false],
        vec![false, true, false, false, false],
        vec![false, true, false, true, false],
        vec![false, true, false, true, true],
        vec![false, true, true, false, false],
        vec![false, true, true, true, true],
        vec![true, false, true, false, true],
        vec![true, false, true, true, true],
        vec![true, true, false, true, false],
        vec![true, true, false, true, true],
        vec![true, true, true, false, false],
        vec![true, true, true, false, true],
        vec![true, true, true, true, false],
      ]
      .into_iter(),
    );
    (vec![0, 3, 5, 8, 9, 16, 17, 18, 22, 23, 24], rules)
  }
}
