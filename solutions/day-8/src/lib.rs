use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn from_data(data: &[u32]) -> Self {
        fn build_node(data: &[u32]) -> (Node, usize) {
            let child_count = data[0];
            let metadata_count = data[1];
            let mut children = vec![];
            let mut index = 2;
            for _ in 0..child_count {
                let (child, len) = build_node(&data[index..]);
                children.push(child);
                index += len;
            }
            let metadata = data[index..(index + metadata_count as usize)].to_vec();
            index += metadata_count as usize;
            (Node { children, metadata }, index)
        }

        build_node(data).0
    }

    fn sum_metadata(&self) -> u32 {
        self.metadata.iter().sum::<u32>()
            + self.children.iter().map(|c| c.sum_metadata()).sum::<u32>()
    }

    fn find_value(&self) -> u32 {
        if self.children.is_empty() {
            return self.metadata.iter().sum();
        }
        self.metadata
            .iter()
            .map(|&m| match self.children.get(m as usize - 1) {
                Some(c) => c.find_value(),
                None => 0,
            })
            .sum()
    }
}

pub fn parse_input(input: &str) -> Result<Node, ParseIntError> {
    let data = input
        .trim()
        .split_whitespace()
        .map(|d| d.parse())
        .collect::<Result<Vec<_>, ParseIntError>>()?;
    Ok(Node::from_data(&data))
}

pub fn solve_part_one(n: &Node) -> u32 {
    n.sum_metadata()
}

pub fn solve_part_two(n: &Node) -> u32 {
    n.find_value()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2\n";
    const REAL_INPUT: &str = include_str!("../input");

    #[test]
    fn it_parses_input_correctly() {
        assert_eq!(parse_input(SAMPLE_INPUT).unwrap(), get_sample_input());
    }

    #[test]
    fn it_solves_part_one_correctly() {
        assert_eq!(solve_part_one(&get_sample_input()), 138);
        assert_eq!(solve_part_one(&parse_input(REAL_INPUT).unwrap()), 49426);
    }

    #[test]
    fn it_solves_part_two_correctly() {
        assert_eq!(solve_part_two(&get_sample_input()), 66);
        assert_eq!(solve_part_two(&parse_input(REAL_INPUT).unwrap()), 40688);
    }

    fn get_sample_input() -> Node {
        Node {
            metadata: vec![1, 1, 2],
            children: vec![
                Node {
                    metadata: vec![10, 11, 12],
                    children: vec![],
                },
                Node {
                    metadata: vec![2],
                    children: vec![Node {
                        metadata: vec![99],
                        children: vec![],
                    }],
                },
            ],
        }
    }
}
