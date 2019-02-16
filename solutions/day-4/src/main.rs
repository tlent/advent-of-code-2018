use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input");

lazy_static! {
    static ref NEW_GUARD_REGEX: Regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();
    static ref SLEEP_START_REGEX: Regex =
        Regex::new(r"\[(\d+-\d+-\d+) 00:(\d+)\] falls asleep").unwrap();
    static ref SLEEP_END_REGEX: Regex = Regex::new(r"\[\d+-\d+-\d+ 00:(\d+)\] wakes up").unwrap();
}

fn main() {
    let parsed_input = parse_input(INPUT);
    println!("{}", find_part_one_solution(&parsed_input));
    println!("{}", find_part_two_solution(&parsed_input));
}

#[derive(Debug, PartialEq)]
pub struct SleepRecord {
    pub guard_number: u32,
    pub start_minute: u8,
    pub end_minute: u8,
    pub date: String,
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
    let mut input_lines: Vec<_> = input.lines().collect();
    input_lines.sort();
    let line_pairs = input_lines.iter().zip(input_lines.iter().skip(1));
    let mut result = vec![];
    let mut current_guard_number = None;
    for (&line, &next_line) in line_pairs {
        if NEW_GUARD_REGEX.is_match(line) {
            let guard_number = NEW_GUARD_REGEX.captures(line).unwrap()[1].parse().unwrap();
            current_guard_number = Some(guard_number);
        } else if SLEEP_START_REGEX.is_match(line) {
            let guard_number = current_guard_number.unwrap();
            let captures = SLEEP_START_REGEX.captures(line).unwrap();
            let date = String::from(&captures[1]);
            let start_minute = captures[2].parse().unwrap();
            let end_minute = SLEEP_END_REGEX.captures(next_line).unwrap()[1]
                .parse()
                .unwrap();
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
        let &SleepRecord {
            guard_number,
            end_minute,
            start_minute,
            ..
        } = record;
        *map.entry(guard_number).or_insert(0) += u32::from(end_minute - start_minute);
        map
    });
    slept_minutes_by_guard_number
        .iter()
        .max_by_key(|(_, &slept_minutes)| slept_minutes)
        .map(|(&guard_number, _)| guard_number)
        .unwrap()
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
    slept_count_by_minute
        .iter()
        .enumerate()
        .max_by_key(|(_, &slept_count)| slept_count)
        .map(|(minute, &slept_count)| (minute as u8, slept_count))
        .unwrap()
}

pub fn find_part_one_solution(records: &[SleepRecord]) -> u32 {
    let guard_number = find_guard_with_most_slept_minutes(records);
    let (minute, _) = find_most_common_sleep_minute_for_guard(records, guard_number);
    u32::from(minute) * guard_number
}

pub fn find_part_two_solution(records: &[SleepRecord]) -> u32 {
    let guard_numbers: HashSet<_> = records.iter().map(|r| r.guard_number).collect();
    let (guard_number, most_common_minute, _) = guard_numbers
        .iter()
        .map(|guard_number| {
            let (most_common_minute, slept_count) =
                find_most_common_sleep_minute_for_guard(records, *guard_number);
            (guard_number, most_common_minute, slept_count)
        })
        .max_by_key(|&(_, _, slept_count)| slept_count)
        .unwrap();
    u32::from(most_common_minute) * guard_number
}

#[cfg(test)]
mod tests {
    use super::*;

    const REORDERED_SAMPLE_INPUT: &str = include_str!("../reordered_sample_input");

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
