use std::collections::VecDeque;
use std::error::Error;

pub fn parse_input(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
  let split_input: Vec<_> = input.trim().split_whitespace().collect();
  let players = match split_input.get(0) {
    Some(c) => c.parse()?,
    None => return Err("No player count found in input".into()),
  };
  let last_marble = match split_input.get(6) {
    Some(c) => c.parse()?,
    None => return Err("No last marble value found in input".into()),
  };
  Ok((players, last_marble))
}

pub fn solve((player_count, last_marble): &(usize, usize)) -> usize {
  let mut player_scores = vec![0; *player_count];
  let mut marbles = VecDeque::new();
  marbles.push_back(0);
  for marble in 1..=*last_marble {
    if marble % 23 == 0 {
      for _ in 0..7 {
        let back = marbles.pop_back().unwrap();
        marbles.push_front(back);
      }
      player_scores[marble % player_count] += marble + marbles.pop_front().unwrap();
      continue;
    }
    for _ in 0..2 {
      let front = marbles.pop_front().unwrap();
      marbles.push_back(front);
    }
    marbles.push_front(marble);
  }
  player_scores.into_iter().max().expect("No solution found")
}

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE_INPUTS: [&str; 6] = [
    "9 players; last marble is worth 25 points",
    "10 players; last marble is worth 1618 points",
    "13 players; last marble is worth 7999 points",
    "17 players; last marble is worth 1104 points",
    "21 players; last marble is worth 6111 points",
    "30 players; last marble is worth 5807 points",
  ];
  const PARSED_SAMPLE_INPUTS: [(usize, usize); 6] = [
    (9, 25),
    (10, 1618),
    (13, 7999),
    (17, 1104),
    (21, 6111),
    (30, 5807),
  ];
  const PART_ONE_SAMPLE_SOLUTIONS: [usize; 6] = [32, 8317, 146373, 2764, 54718, 37305];

  #[test]
  fn it_parses_input_correctly() {
    for (input, &parsed_input) in SAMPLE_INPUTS.iter().zip(PARSED_SAMPLE_INPUTS.iter()) {
      assert_eq!(
        parse_input(input).unwrap(),
        parsed_input,
        "failed for input: '{}'",
        input
      );
    }
  }

  #[test]
  fn it_solves_part_one_correctly() {
    for (input, &solution) in PARSED_SAMPLE_INPUTS
      .iter()
      .zip(PART_ONE_SAMPLE_SOLUTIONS.iter())
    {
      assert_eq!(solve(input), solution, "failed for input: '{:?}'", input);
    }
  }
}
