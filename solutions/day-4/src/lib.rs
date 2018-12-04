extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct SleepRecord {
  pub guard_number: u32,
  pub start_minute: u8,
  pub end_minute: u8,
  pub date: String,
}

lazy_static! {
  static ref NEW_GUARD_REGEX: Regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();
  static ref SLEEP_START_REGEX: Regex =
    Regex::new(r"\[(\d+-\d+-\d+) 00:(\d+)\] falls asleep").unwrap();
  static ref SLEEP_END_REGEX: Regex = Regex::new(r"\[\d+-\d+-\d+ 00:(\d+)\] wakes up").unwrap();
}

// Will not panic as long as:
//  - the input is formatted as specified in the problem
//  - a guard begins shift message appears before the first falls asleep message when ordered by
//    date (a guard must be on shift for a guard to fall asleep)
//  - all falls asleep messages have a wakes up message as the following message when messages are
//    ordered by date (while a guard is asleep a new guard cannot begin shift and the guard cannot
//    fall asleep a second time without first waking up)
//
// out of order wakes up messages (when ordered by date) will be silently ignored
pub fn parse_input(input: &str) -> Vec<SleepRecord> {
  let mut input_lines: Vec<&str> = input.split('\n').collect();
  input_lines.sort();
  let lines = input_lines.iter().zip(input_lines.iter().skip(1));
  let mut result = vec![];
  let mut current_guard_number = None;
  for (&line, &next_line) in lines {
    if NEW_GUARD_REGEX.is_match(line) {
      let guard_number_match = NEW_GUARD_REGEX.captures(line).unwrap().get(1).unwrap();
      let guard_number = guard_number_match.as_str().parse().unwrap();
      current_guard_number = Some(guard_number);
    } else if SLEEP_START_REGEX.is_match(line) {
      let guard_number = current_guard_number.unwrap();
      let captures = SLEEP_START_REGEX.captures(line).unwrap();
      let date = captures.get(1).unwrap().as_str().to_owned();
      let start_minute = captures.get(2).unwrap().as_str().parse().unwrap();
      let end_minute_match = SLEEP_END_REGEX.captures(next_line).unwrap().get(1).unwrap();
      let end_minute = end_minute_match.as_str().parse().unwrap();
      result.push(SleepRecord {
        guard_number,
        start_minute,
        end_minute,
        date,
      });
    } else if !SLEEP_END_REGEX.is_match(line) && !line.is_empty() {
      panic!("invalid input line: {}", line);
    }
  }
  result
}

/// Returns guard number that had the most total slept minutes
fn find_guard_with_most_slept_minutes(records: &[SleepRecord]) -> u32 {
  let slept_minutes_by_guard_number = records.iter().fold(HashMap::new(), |mut map, record| {
    *map.entry(record.guard_number).or_insert(0) +=
      (record.end_minute - record.start_minute) as u32;
    map
  });
  *slept_minutes_by_guard_number
    .iter()
    .max_by(|a, b| a.1.cmp(&b.1)) // max by slept_minutes
    .unwrap()
    .0 // guard_number
}

/// Finds the minute that the guard specified by guard_number was most often asleep at
/// Returns a tuple with (minute most often asleep at, number of times the guard was asleep at that minute)
fn find_most_common_sleep_minute_for_guard(
  records: &[SleepRecord],
  guard_number: u32,
) -> (u8, u32) {
  let slept_count_by_minute = records
    .iter()
    .filter(|r| r.guard_number == guard_number)
    .fold([0; 60], |mut acc, r| {
      for minute in r.start_minute..r.end_minute {
        acc[minute as usize] += 1
      }
      acc
    });
  let result = slept_count_by_minute
    .iter()
    .enumerate()
    .max_by(|a, b| a.1.cmp(&b.1)) // max by slept_count
    .unwrap();
  (result.0 as u8, *result.1) // (minute, slept_count)
}

pub fn find_part_one_solution(records: &[SleepRecord]) -> u32 {
  let guard_with_most_slept_minutes = find_guard_with_most_slept_minutes(records);
  find_most_common_sleep_minute_for_guard(records, guard_with_most_slept_minutes).0 as u32
    * guard_with_most_slept_minutes
}

pub fn find_part_two_solution(records: &[SleepRecord]) -> u32 {
  let guard_numbers: HashSet<u32> = records.iter().map(|r| r.guard_number).collect();
  let (guard_number, most_common_minute, _) = guard_numbers
    .iter()
    .map(|guard_number| {
      let (most_common_minute, slept_count) =
        find_most_common_sleep_minute_for_guard(records, *guard_number);
      (guard_number, most_common_minute, slept_count)
    })
    .max_by(|a, b| (a.2).cmp(&(b.2))) // max by slept_count
    .unwrap();
  most_common_minute as u32 * guard_number
}

#[cfg(test)]
mod tests {
  use super::*;

  const REORDERED_SAMPLE_INPUT: &'static str = include_str!("../reordered_sample_input");

  #[test]
  fn it_parses_input_correctly() {
    assert_eq!(parse_input(REORDERED_SAMPLE_INPUT), get_sample_records());
  }

  #[test]
  fn it_finds_correct_guard_with_most_sleep() {
    assert_eq!(
      find_guard_with_most_slept_minutes(&get_sample_records()),
      10
    );
  }

  #[test]
  fn it_finds_correct_most_common_sleep_minute() {
    assert_eq!(
      find_most_common_sleep_minute_for_guard(&get_sample_records(), 10),
      (24, 2)
    );
  }

  #[test]
  fn it_finds_correct_part_one_solution() {
    assert_eq!(find_part_one_solution(&get_sample_records()), 240);
  }

  #[test]
  fn it_finds_correct_part_two_solution() {
    assert_eq!(find_part_two_solution(&get_sample_records()), 4455);
  }

  fn get_sample_records() -> Vec<SleepRecord> {
    vec![
      SleepRecord {
        guard_number: 10,
        start_minute: 5,
        end_minute: 25,
        date: "1518-11-01".to_owned(),
      },
      SleepRecord {
        guard_number: 10,
        start_minute: 30,
        end_minute: 55,
        date: "1518-11-01".to_owned(),
      },
      SleepRecord {
        guard_number: 99,
        start_minute: 40,
        end_minute: 50,
        date: "1518-11-02".to_owned(),
      },
      SleepRecord {
        guard_number: 10,
        start_minute: 24,
        end_minute: 29,
        date: "1518-11-03".to_owned(),
      },
      SleepRecord {
        guard_number: 99,
        start_minute: 36,
        end_minute: 46,
        date: "1518-11-04".to_owned(),
      },
      SleepRecord {
        guard_number: 99,
        start_minute: 45,
        end_minute: 55,
        date: "1518-11-05".to_owned(),
      },
    ]
  }
}
