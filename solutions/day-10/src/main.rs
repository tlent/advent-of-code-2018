use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("../input");
const MAX_STEPS: u32 = 1_000_000;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

lazy_static! {
    static ref INPUT_REGEX: Regex =
        Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>").unwrap();
}

fn main() -> Result<()> {
    let points = parse_input(INPUT)?;
    let solution = solve(&points);
    println!("{}{}", solution.0, solution.1);
    Ok(())
}

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Point {
    fn step(&mut self) {
        let (x, y) = self.position;
        let (vx, vy) = self.velocity;
        self.position = (x + vx, y + vy);
    }

    fn has_neighbor_in(&self, other_points: &[Point]) -> bool {
        let (x, y) = self.position;
        let neighboring_positions = [
            (x + 1, y + 1),
            (x + 1, y),
            (x + 1, y - 1),
            (x, y + 1),
            (x, y - 1),
            (x - 1, y + 1),
            (x - 1, y),
            (x - 1, y - 1),
        ];
        other_points
            .iter()
            .any(|p| neighboring_positions.contains(&p.position))
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Point>> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let captures = INPUT_REGEX
                .captures(line)
                .ok_or_else(|| format!("Line did not match input regex: {}", line))?;
            let position = (captures[1].parse()?, captures[2].parse()?);
            let velocity = (captures[3].parse()?, captures[4].parse()?);
            Ok(Point { position, velocity })
        })
        .collect()
}

pub fn solve(points: &[Point]) -> (String, u32) {
    let mut points = points.to_owned();
    for step in 0..=MAX_STEPS {
        if points.iter().all(|p| p.has_neighbor_in(&points)) {
            return (points_to_str(&points), step);
        }
        points.iter_mut().for_each(|p| p.step());
    }
    panic!("hit max steps");
}

fn points_to_str(points: &[Point]) -> String {
    let x_values = points.iter().map(|p| p.position.0);
    let min_x = x_values.clone().min().unwrap();
    let max_x = x_values.clone().max().unwrap();
    let y_values = points.iter().map(|p| p.position.1);
    let min_y = y_values.clone().min().unwrap();
    let max_y = y_values.clone().max().unwrap();
    let mut result = String::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = if points.iter().any(|p| p.position == (x, y)) {
                '#'
            } else {
                '.'
            };
            result.push(c);
        }
        result.push('\n');
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample-input");
    const SAMPLE_OUTPUT: &str = include_str!("../sample-output");

    #[test]
    fn it_parses_input_correctly() {
        assert_eq!(parse_input(SAMPLE_INPUT).unwrap(), get_sample_input());
    }

    #[test]
    fn it_solves_part_one_correctly() {
        assert_eq!(solve(&get_sample_input()).0, SAMPLE_OUTPUT);
    }

    #[test]
    fn it_solves_part_two_correctly() {
        assert_eq!(solve(&get_sample_input()).1, 3);
    }

    fn get_sample_input() -> [Point; 31] {
        [
            Point {
                position: (9, 1),
                velocity: (0, 2),
            },
            Point {
                position: (7, 0),
                velocity: (-1, 0),
            },
            Point {
                position: (3, -2),
                velocity: (-1, 1),
            },
            Point {
                position: (6, 10),
                velocity: (-2, -1),
            },
            Point {
                position: (2, -4),
                velocity: (2, 2),
            },
            Point {
                position: (-6, 10),
                velocity: (2, -2),
            },
            Point {
                position: (1, 8),
                velocity: (1, -1),
            },
            Point {
                position: (1, 7),
                velocity: (1, 0),
            },
            Point {
                position: (-3, 11),
                velocity: (1, -2),
            },
            Point {
                position: (7, 6),
                velocity: (-1, -1),
            },
            Point {
                position: (-2, 3),
                velocity: (1, 0),
            },
            Point {
                position: (-4, 3),
                velocity: (2, 0),
            },
            Point {
                position: (10, -3),
                velocity: (-1, 1),
            },
            Point {
                position: (5, 11),
                velocity: (1, -2),
            },
            Point {
                position: (4, 7),
                velocity: (0, -1),
            },
            Point {
                position: (8, -2),
                velocity: (0, 1),
            },
            Point {
                position: (15, 0),
                velocity: (-2, 0),
            },
            Point {
                position: (1, 6),
                velocity: (1, 0),
            },
            Point {
                position: (8, 9),
                velocity: (0, -1),
            },
            Point {
                position: (3, 3),
                velocity: (-1, 1),
            },
            Point {
                position: (0, 5),
                velocity: (0, -1),
            },
            Point {
                position: (-2, 2),
                velocity: (2, 0),
            },
            Point {
                position: (5, -2),
                velocity: (1, 2),
            },
            Point {
                position: (1, 4),
                velocity: (2, 1),
            },
            Point {
                position: (-2, 7),
                velocity: (2, -2),
            },
            Point {
                position: (3, 6),
                velocity: (-1, -1),
            },
            Point {
                position: (5, 0),
                velocity: (1, 0),
            },
            Point {
                position: (-6, 0),
                velocity: (2, 0),
            },
            Point {
                position: (5, 9),
                velocity: (1, -2),
            },
            Point {
                position: (14, 7),
                velocity: (-2, 0),
            },
            Point {
                position: (-3, 6),
                velocity: (2, -1),
            },
        ]
    }
}
