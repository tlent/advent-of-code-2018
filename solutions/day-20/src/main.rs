const INPUT: &str = include_str!("../input");

const VERBOSE: bool = false;

fn solve_part_one(regex: &str) -> usize {
    if VERBOSE {
        println!("{}", regex);
    }
    find_door_count(&regex[1..regex.len() - 1])
}

fn find_door_count(s: &str) -> usize {
    if !s.contains('(') {
        if VERBOSE {
            println!("{}: {}", s, s.len());
        }
        return s.len();
    }
    let mut doors = 0;
    let mut i = 0;
    while i < s.len() {
        let c = s.chars().nth(i).unwrap();
        if c == '(' {
            let matching_paren_position = i + find_matching_paren_position(&s[i..]);
            let branch_group = &s[i..=matching_paren_position];
            if VERBOSE {
                println!("branching {}", branch_group);
            }
            let branch_door_count = find_branch_door_count(branch_group);
            if VERBOSE {
                println!("{} branch doors {}", branch_group, branch_door_count);
            }
            doors += branch_door_count;
            i = matching_paren_position + 1;
            continue;
        }
        doors += 1;
        i += 1;
    }
    doors
}

fn find_branch_door_count(s: &str) -> usize {
    assert!(s.starts_with('('));
    assert!(s.ends_with(')'));
    let branch_door_counts: Vec<_> = split_branch_group(s)
        .iter()
        .map(|g| find_door_count(g))
        .collect();
    if branch_door_counts.iter().any(|c| *c == 0) {
        0
    } else {
        *branch_door_counts.iter().max().unwrap()
    }
}

fn split_branch_group(s: &str) -> Vec<&str> {
    assert!(s.starts_with('('));
    assert!(s.ends_with(')'));
    let mut depth = 0;
    let mut start = 1;
    let mut parts = vec![];
    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            depth += 1;
        }
        if c == ')' {
            depth -= 1;
        }
        if c == '|' && depth == 1 {
            parts.push(&s[start..i]);
            start = i + 1;
        }
        if depth == 0 {
            parts.push(&s[start..i]);
            break;
        }
    }
    parts
}

fn find_matching_paren_position(s: &str) -> usize {
    assert!(s.starts_with('('));
    let mut depth = 0;
    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            depth += 1;
        }
        if c == ')' {
            depth -= 1;
        }
        if depth == 0 {
            return i;
        }
    }
    panic!("No matching paren found in {}", s);
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
