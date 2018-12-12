use std::collections::HashSet;

const INPUT: &str = include_str!("../input");
const GENERATIONS: u32 = 20;

fn parse_input(input: &str) -> (Vec<bool>, HashSet<Vec<bool>>) {
  let mut lines = input.trim().lines();
  let first_line = lines.nth(0).unwrap();
  let initial_state: Vec<_> = first_line[15..].chars().map(|c| c == '#').collect();
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

fn solve_part_one(
  initial_state: &[bool],
  rules: &HashSet<Vec<bool>>,
  start_pot_number: i32,
) -> i32 {
  let plant_count = initial_state.len();
  let mut current_gen = initial_state.to_vec();
  for _ in 0..GENERATIONS {
    display_plants(&current_gen);
    current_gen = (0..plant_count)
      .map(|current_plant_index| {
        let start = current_plant_index as i32 - 2;
        let end = current_plant_index as i32 + 2;
        let sequence: Vec<_> = (start..=end)
          .map(|i| {
            let index = if i < 0 {
              (plant_count as i32 + i) as usize
            } else if i >= plant_count as i32 {
              i as usize - plant_count
            } else {
              i as usize
            };
            current_gen[index]
          })
          .collect();
        rules.contains(&sequence)
      })
      .collect();

    println!(
      "{}",
      (start_pot_number..)
        .zip(current_gen.iter())
        .filter(|(_, b)| **b)
        .map(|(i, _)| i)
        .sum::<i32>()
    );
  }
  (start_pot_number..)
    .zip(current_gen.iter())
    .filter(|(_, b)| **b)
    .map(|(i, _)| i)
    .sum()
}

fn display_plants(plants: &[bool]) {
  println!(
    "{}",
    plants
      .iter()
      .map(|b| if *b { '#' } else { '.' })
      .collect::<String>()
  );
}

fn main() {
  let (initial_state, rules) = parse_input(INPUT);
  println!("{}", solve_part_one(&initial_state, &rules, 0));
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
    assert_eq!(solve_part_one(&initial_state, &rules, -3), 325);
  }

  fn get_sample_input() -> (Vec<bool>, HashSet<Vec<bool>>) {
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
    (
      vec![
        false, false, false, true, false, false, true, false, true, false, false, true, true,
        false, false, false, false, false, false, true, true, true, false, false, false, true,
        true, true, false, false, false, false, false, false, false, false, false, false, false,
      ],
      rules,
    )
  }
}
