use std::collections::VecDeque;

pub type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

pub fn parse_input(input: &str) -> Result<(usize, u32)> {
    let split_input: Vec<_> = input.trim().split_whitespace().collect();
    let players = match split_input.get(0) {
        Some(c) => c.parse()?,
        None => return Err(Box::from("No player count found in input")),
    };
    let last_marble = match split_input.get(6) {
        Some(c) => c.parse()?,
        None => return Err(Box::from("No last marble value found in input")),
    };
    Ok((players, last_marble))
}

// Original solution which worked for part one used a regular Vec and did normal insertion and removal.
// This solution was O(n^2) due to O(n) insertions and removals, so it took too long for part two.

// Two better solutions below: one with a deque and one with a linked list.

// The solution using deque is simpler and slightly faster (probably due to better use of CPU caching)
// It always accesses values sequentially at either the front or the back of the deque and it only stores u32s.
// The linked list jumps around to indices in the vec and stores a u32 and two usize per marble.

// Solution with deque O(n)
// The current marble is kept at index 0 so removing or inserting doesn't require O(n) shifting.
// The manual shifting to keep the current marble at index 0 always takes a constant number of shifts.
pub fn solve_with_deque(&(player_count, last_marble): &(usize, u32)) -> u32 {
    let mut player_scores = vec![0; player_count];
    let mut marbles = VecDeque::with_capacity(last_marble as usize + 1);
    marbles.push_back(0);
    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            for _ in 0..7 {
                let back = marbles.pop_back().unwrap();
                marbles.push_front(back);
            }
            player_scores[marble as usize % player_count] += marble + marbles.pop_front().unwrap();
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

// Solution with linked list O(n)
// Marbles are stored with references to the next marble and the previous marble.
// All references are actually array indices.
// Removing and inserting requires no shifting because nothing is actually removed from the array
// and all inserts are done at the end of the array.
// Implementing a doubly linked list that doesn't use a backing Vec is very complex in Rust
// and probably would not be faster.
pub fn solve_with_linked_list(&(player_count, last_marble): &(usize, u32)) -> u32 {
    let mut player_scores = vec![0; player_count];
    let mut marbles = Vec::with_capacity(last_marble as usize + 1);
    marbles.push(Marble {
        value: 0,
        next: 0,
        prev: 0,
    });
    let mut current_marble_index = 0;
    for new_marble_value in 1..=last_marble {
        if new_marble_value % 23 == 0 {
            let mut marble_to_remove_index = current_marble_index;
            for _ in 0..7 {
                marble_to_remove_index = marbles[marble_to_remove_index].prev;
            }
            let marble_to_remove = marbles[marble_to_remove_index];
            marbles[marble_to_remove.prev].next = marble_to_remove.next;
            marbles[marble_to_remove.next].prev = marble_to_remove.prev;
            player_scores[new_marble_value as usize % player_count] +=
                new_marble_value + marble_to_remove.value;
            current_marble_index = marble_to_remove.next;
            continue;
        }
        let one_clockwise_index = marbles[current_marble_index].next;
        let two_clockwise_index = marbles[one_clockwise_index].next;
        let new_marble_index = marbles.len();
        marbles.push(Marble {
            value: new_marble_value,
            prev: one_clockwise_index,
            next: two_clockwise_index,
        });
        marbles[one_clockwise_index].next = new_marble_index;
        marbles[two_clockwise_index].prev = new_marble_index;
        current_marble_index = new_marble_index;
    }
    player_scores.into_iter().max().expect("No solution found")
}

#[derive(Debug, Copy, Clone)]
struct Marble {
    value: u32,
    next: usize,
    prev: usize,
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
    const PARSED_SAMPLE_INPUTS: [(usize, u32); 6] = [
        (9, 25),
        (10, 1618),
        (13, 7999),
        (17, 1104),
        (21, 6111),
        (30, 5807),
    ];
    const PART_ONE_SAMPLE_SOLUTIONS: [u32; 6] = [32, 8317, 146_373, 2764, 54718, 37305];
    const PART_ONE_INPUT: (usize, u32) = (412, 71646);
    const PART_TWO_INPUT: (usize, u32) = (PART_ONE_INPUT.0, PART_ONE_INPUT.1 * 100);
    const PART_ONE_SOLUTION: u32 = 439_635;
    const PART_TWO_SOLUTION: u32 = 3_562_722_971;

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
            assert_eq!(
                solve_with_deque(input),
                solution,
                "failed for input: '{:?}'",
                input
            );
            assert_eq!(
                solve_with_linked_list(input),
                solution,
                "linked list solution failed for input: '{:?}'",
                input
            );
        }
        assert_eq!(solve_with_deque(&PART_ONE_INPUT), PART_ONE_SOLUTION);
        assert_eq!(solve_with_linked_list(&PART_ONE_INPUT), PART_ONE_SOLUTION);
    }

    #[test]
    fn it_solves_part_two_correctly() {
        assert_eq!(solve_with_deque(&PART_TWO_INPUT), PART_TWO_SOLUTION);
        assert_eq!(solve_with_linked_list(&PART_TWO_INPUT), PART_TWO_SOLUTION);
    }
}
