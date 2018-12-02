use std::collections::HashMap;

pub fn parse_input(input: &str) -> Vec<&str> {
  input.split_whitespace().collect()
}

pub fn calculate_checksum(ids: &[&str]) -> i32 {
  let (two_duplicate_count, three_duplicate_count) = ids.iter().fold((0, 0), |acc, id| {
    let character_counts = id.chars().fold(HashMap::new(), |mut counts, c| {
      *counts.entry(c).or_insert(0) += 1;
      counts
    });
    let has_two_duplicate = character_counts.values().any(|count| *count == 2);
    let has_three_duplicate = character_counts.values().any(|count| *count == 3);
    match (has_two_duplicate, has_three_duplicate) {
      (true, true) => (acc.0 + 1, acc.1 + 1),
      (true, false) => (acc.0 + 1, acc.1),
      (false, true) => (acc.0, acc.1 + 1),
      (false, false) => acc,
    }
  });
  two_duplicate_count * three_duplicate_count
}

pub fn find_similar_id_match(ids: &[&str]) -> String {
  let mut ids = ids.to_owned().clone();
  ids.sort_unstable();
  let (id_a, id_b) = ids
    .iter()
    .zip(ids.iter().skip(1))
    .find(|(id, next_id)| {
      id.chars()
        .zip(next_id.chars())
        .filter(|(a, b)| a != b)
        .count()
        == 1
    }).expect("No solution found");
  id_a
    .chars()
    .zip(id_b.chars())
    .filter(|(a, b)| a == b)
    .map(|(a, _)| a)
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_calculates_correct_checksum() {
    let sample_input = [
      "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
    ];
    assert_eq!(calculate_checksum(&sample_input), 12);
    let real_input = include_str!("../input");
    let ids = parse_input(real_input);
    assert_eq!(calculate_checksum(&ids), 6723);
  }

  #[test]
  fn it_finds_correct_similar_id_match() {
    let sample_input = [
      "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
    ];
    assert_eq!(find_similar_id_match(&sample_input), "fgij");
    let real_input = include_str!("../input");
    let ids = parse_input(real_input);
    assert_eq!(find_similar_id_match(&ids), "prtkqyluiusocwvaezjmhmfgx");
  }
}
