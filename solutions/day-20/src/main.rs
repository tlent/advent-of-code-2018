use std::collections::HashMap;

const INPUT: &str = include_str!("../input");

fn solve_part_one(regex: &str) -> usize {
    let door_counts = find_door_counts(&regex[1..regex.len() - 1]);
    *door_counts.iter().map(|(_, v)| v).max().unwrap()
}

type Position = (isize, isize);

fn find_door_counts(s: &str) -> HashMap<Position, usize> {
    let mut door_counts = HashMap::new();
    let mut doors = 0;
    for c in s.chars() {
        if c == '(' {
            // Count subdoors in array, store subdoors + doors for each position, at | start new subdoors
            // at ) add max subdoors to doors and continue
            // but this will not work for multiple subregexes
        }
        doors += 1;
    }
    door_counts
}

fn main() {
    let input = INPUT.trim();
    println!("{}", solve_part_one(input));
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
    fn it_solves_samples_correctly() {
        for (i, &(input, expected)) in SAMPLES.iter().enumerate() {
            assert_eq!(
                solve_part_one(input),
                expected,
                "wrong answer for input #{}: {}",
                i,
                input
            );
        }
    }

    #[test]
    fn it_solves_many_option_branches_correctly() {
        let input = "^ABC(A|B|CCCC)D$";
        assert_eq!(solve_part_one(input), 8);
    }
}
