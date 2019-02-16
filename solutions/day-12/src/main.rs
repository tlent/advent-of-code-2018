const INPUT: &str = include_str!("../input");
const PART_ONE_GENERATIONS: usize = 20;
const PART_TWO_GENERATIONS: usize = 50_000_000_000;

fn parse_input(input: &str) -> (Vec<u32>, Vec<u8>) {
    let mut lines = input.trim().lines();
    let first_line = lines.nth(0).unwrap();
    let initial_state: Vec<_> = first_line[15..]
        .chars()
        .enumerate()
        .filter(|&(_i, c)| c == '#')
        .map(|(i, _c)| i as u32)
        .collect();
    assert!(lines.next().unwrap().is_empty());
    let rules = lines
        .filter(|line| line.chars().nth(9).unwrap() == '#')
        .map(|line| {
            line.chars()
                .take(5)
                .fold(0, |acc, c| acc * 2 + if c == '#' { 1 } else { 0 })
        })
        .collect();
    (initial_state, rules)
}

fn solve(initial_state: &[u32], rules: &[u8], generations: usize) -> i64 {
    let mut prev_state: Vec<_> = initial_state.iter().map(|&x| i64::from(x)).collect();
    let mut rules = rules.to_vec();
    rules.sort();
    for current_gen in 1..=generations {
        let mut new_state = vec![];
        let first_filled_pot = prev_state[0];
        let last_filled_pot = prev_state[prev_state.len() - 1];
        for pot_number in (first_filled_pot - 2)..=(last_filled_pot + 2) {
            let sequence = ((pot_number - 2)..=(pot_number + 2)).fold(0, |acc, i| {
                if i >= first_filled_pot
                    && i <= last_filled_pot
                    && prev_state.binary_search(&i).is_ok()
                {
                    return acc * 2 + 1;
                }
                acc * 2
            });
            if rules.binary_search(&sequence).is_ok() {
                new_state.push(pot_number);
            }
        }
        let prev_sum: i64 = prev_state.iter().sum();
        let new_sum: i64 = new_state.iter().sum();
        if prev_sum - (prev_state[0] * prev_state.len() as i64)
            == new_sum - (new_state[0] * new_state.len() as i64)
        {
            let generation_change = new_sum - prev_sum;
            return new_sum + generation_change * (generations - current_gen) as i64;
        }
        prev_state = new_state;
    }
    prev_state.iter().sum()
}

fn main() {
    let (initial_state, rules) = parse_input(INPUT);
    println!("{}", solve(&initial_state, &rules, PART_ONE_GENERATIONS));
    println!("{}", solve(&initial_state, &rules, PART_TWO_GENERATIONS));
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
        assert_eq!(solve(&initial_state, &rules, PART_ONE_GENERATIONS), 325);
    }

    #[test]
    fn it_solves_part_two_correctly() {
        let (initial_state, rules) = parse_input(INPUT);
        assert_eq!(
            solve(&initial_state, &rules, PART_TWO_GENERATIONS),
            4_900_000_001_793
        );
    }

    fn get_sample_input() -> (Vec<u32>, Vec<u8>) {
        (
            vec![0, 3, 5, 8, 9, 16, 17, 18, 22, 23, 24],
            vec![
                0b00011, 0b00100, 0b01000, 0b01010, 0b01011, 0b01100, 0b01111, 0b10101, 0b10111,
                0b11010, 0b11011, 0b11100, 0b11101, 0b11110,
            ],
        )
    }
}
