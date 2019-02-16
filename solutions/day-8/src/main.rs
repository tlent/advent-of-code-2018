const INPUT: &str = include_str!("../input");

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let parsed_input = parse_input(INPUT)?;
    println!("{}", parsed_input.sum_metadata());
    println!("{}", parsed_input.find_value());
    Ok(())
}

#[derive(Debug, PartialEq)]
struct Node {
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
            .map(|&m| {
                self.children
                    .get(m as usize - 1)
                    .map(|c| c.find_value())
                    .unwrap_or(0)
            })
            .sum()
    }
}

fn parse_input(input: &str) -> Result<Node> {
    let data = input
        .trim()
        .split_whitespace()
        .map(|d| d.parse().map_err(Box::from))
        .collect::<Result<Vec<_>>>()?;
    Ok(Node::from_data(&data))
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
        assert_eq!(get_sample_input().sum_metadata(), 138);
        assert_eq!(parse_input(REAL_INPUT).unwrap().sum_metadata(), 49426);
    }

    #[test]
    fn it_solves_part_two_correctly() {
        assert_eq!(get_sample_input().find_value(), 66);
        assert_eq!(parse_input(REAL_INPUT).unwrap().find_value(), 40688);
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
