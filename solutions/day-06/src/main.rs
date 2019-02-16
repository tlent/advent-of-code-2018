use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;

const INPUT: &str = include_str!("../input");
const MAX_DISTANCE: usize = 10_000;
const UPPERCASE_LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE_LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let parsed_input = parse_input(INPUT)?;
    println!("{}", find_part_one_solution(&parsed_input));
    println!("{}", find_part_two_solution(&parsed_input, MAX_DISTANCE));
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn distance_to(&self, other: &Point) -> usize {
        let x_distance = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };
        let y_distance = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };
        x_distance + y_distance
    }
}

#[derive(Debug)]
struct GridCoordinate {
    id: usize,
    point: Point,
}

struct Grid {
    data: Vec<Vec<Option<usize>>>,
    x_offset: usize,
    y_offset: usize,
    coordinates: Vec<GridCoordinate>,
}

impl Grid {
    fn from_points(points: &[Point]) -> Self {
        let min_x = points.iter().map(|p| p.x).min().unwrap();
        let max_x = points.iter().map(|p| p.x).max().unwrap();
        let min_y = points.iter().map(|p| p.y).min().unwrap();
        let max_y = points.iter().map(|p| p.y).max().unwrap();
        let mut data = vec![vec![None; max_x - min_x + 1]; max_y - min_y + 1];
        for (index, p) in points.iter().enumerate() {
            let Point { x, y } = *p;
            data[y - min_y][x - min_x] = Some(index);
        }
        let coordinates = points
            .iter()
            .enumerate()
            .map(|(index, &point)| GridCoordinate { id: index, point })
            .collect();
        Grid {
            data,
            x_offset: min_x,
            y_offset: min_y,
            coordinates,
        }
    }

    fn fill_areas(&mut self) {
        for (i, row) in self.data.iter_mut().enumerate() {
            for (j, value) in row.iter_mut().enumerate() {
                if value.is_some() {
                    continue;
                }
                let current_point = Point {
                    x: j + self.x_offset,
                    y: i + self.y_offset,
                };
                let closest_coordinate = self.coordinates.iter().min_by_strict(|a, b| {
                    a.point
                        .distance_to(&current_point)
                        .cmp(&b.point.distance_to(&current_point))
                });
                *value = closest_coordinate.map(|c| c.id);
            }
        }
    }

    fn has_area_reaching_edge(&self, c: &GridCoordinate) -> bool {
        let first_column = self.data.iter().map(|row| &row[0]);
        let last_column = self.data.iter().map(|row| &row[row.len() - 1]);
        let first_row = self.data[0].iter();
        let last_row = self.data[self.data.len() - 1].iter();
        let edge_points: HashSet<_> = first_column
            .chain(last_column)
            .chain(first_row)
            .chain(last_row)
            .filter_map(|v| *v) // Filter out None and map Some(value) to value
            .collect();
        edge_points.contains(&c.id)
    }

    fn has_coordinate_at(&self, point: &Point) -> bool {
        self.coordinates
            .iter()
            .map(|c| c.point)
            .any(|p| *point == p)
    }

    fn count_points_with_id(&self, id: usize) -> usize {
        self.data
            .iter()
            .map(|row| row.iter().filter(|&&value| value == Some(id)).count())
            .sum()
    }

    fn count_points_with_max_total_coordinate_distance(&self, max_distance: usize) -> usize {
        let mut count = 0;
        for (i, row) in self.data.iter().enumerate() {
            for j in 0..row.len() {
                let current_point = Point {
                    x: j + self.x_offset,
                    y: i + self.y_offset,
                };
                let total_coordinate_distance: usize = self
                    .coordinates
                    .iter()
                    .map(|c| current_point.distance_to(&c.point))
                    .sum();
                if total_coordinate_distance < max_distance {
                    count += 1;
                }
            }
        }
        count
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Grid {{ x_offset: {:?}, y_offset: {:?}, coordinates: {:?}, data: \n\n  ",
            self.x_offset, self.y_offset, self.coordinates
        )?;
        for column_number in self.x_offset..self.x_offset + self.data[0].len() {
            write!(f, "{} ", column_number)?;
        }
        writeln!(f)?;
        for (i, row) in self.data.iter().enumerate() {
            let y = i + self.y_offset;
            write!(f, "{} ", y)?;
            for (j, value) in row.iter().enumerate() {
                let x = j + self.x_offset;
                write!(
                    f,
                    "{} ",
                    match value {
                        Some(i) => {
                            let letters = if self.has_coordinate_at(&Point { x, y }) {
                                UPPERCASE_LETTERS
                            } else {
                                LOWERCASE_LETTERS
                            };
                            letters.chars().nth(*i).unwrap_or('*')
                        }
                        None => '.',
                    }
                )?;
            }
            writeln!(f)?;
        }
        write!(f, "\n}}")
    }
}

trait MinByStrictExt: Iterator {
    /// Returns the element that has the minimum value.
    ///
    /// If several elements are equally minimum, None is returned.
    /// If the iterator is empty, None is returned.
    fn min_strict(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.min_by_strict(|a, b| a.cmp(b))
    }

    /// Returns the element that gives the minimum value with respect to the specified comparison function.
    ///
    /// If several elements are equally minimum, None is returned.
    /// If the iterator is empty, None is returned.
    fn min_by_strict<F>(self, mut compare: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
        Self: Sized,
    {
        let (min, count) = self.fold((None, 0), |(current_min, count), item| match current_min {
            None => (Some(item), 1),
            Some(min) => match compare(&item, &min) {
                Ordering::Less => (Some(item), 1),
                Ordering::Equal => (Some(min), count + 1),
                Ordering::Greater => (Some(min), count),
            },
        });
        if count == 1 {
            min
        } else {
            None
        }
    }
}

impl<I: Iterator> MinByStrictExt for I {}

fn parse_input(input: &str) -> Result<Vec<Point>> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let parsed_line: Result<Vec<_>> = line
                .split(", ")
                .map(|s| s.parse().map_err(Box::from))
                .collect();
            match parsed_line {
                Ok(values) => Ok(Point {
                    x: values[0],
                    y: values[1],
                }),
                Err(e) => Err(e),
            }
        })
        .collect()
}

fn find_part_one_solution(points: &[Point]) -> usize {
    let mut grid = Grid::from_points(points);
    grid.fill_areas();
    grid.coordinates
        .iter()
        .filter(|c| !grid.has_area_reaching_edge(&c))
        .map(|c| grid.count_points_with_id(c.id))
        .max()
        .expect("No solution found")
}

fn find_part_two_solution(points: &[Point], max_distance: usize) -> usize {
    let grid = Grid::from_points(points);
    grid.count_points_with_max_total_coordinate_distance(max_distance)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_input_correctly() {
        let sample_input = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9\n";
        assert_eq!(parse_input(sample_input).unwrap(), get_sample_input());
    }

    #[test]
    fn it_finds_correct_part_one_solution() {
        let sample_input = get_sample_input();
        assert_eq!(find_part_one_solution(&sample_input), 17);
    }

    #[test]
    fn it_finds_correct_part_two_solution() {
        let sample_input = get_sample_input();
        assert_eq!(find_part_two_solution(&sample_input, 32), 16);
    }

    #[test]
    fn point_distance_to_returns_correct_result() {
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 1, y: 1 };
        let c = Point { x: 5, y: 10 };
        assert_eq!(a.distance_to(&b), 2);
        assert_eq!(a.distance_to(&c), 15);
        assert_eq!(b.distance_to(&a), 2);
        assert_eq!(b.distance_to(&c), 13);
    }

    #[test]
    fn min_by_strict_returns_correct_result() {
        let a = [1, 2, 3];
        let b = [1, 1, 3];
        let c: Vec<i32> = vec![];
        let d = [3, 3, 1];
        assert_eq!(a.iter().min_by_strict(|a, b| a.cmp(&b)), Some(&1));
        assert_eq!(b.iter().min_by_strict(|a, b| a.cmp(&b)), None);
        assert_eq!(c.iter().min_by_strict(|a, b| a.cmp(&b)), None);
        assert_eq!(d.iter().min_by_strict(|a, b| a.cmp(&b)), Some(&1));
    }

    fn get_sample_input() -> [Point; 6] {
        [
            Point { x: 1, y: 1 },
            Point { x: 1, y: 6 },
            Point { x: 8, y: 3 },
            Point { x: 3, y: 4 },
            Point { x: 5, y: 5 },
            Point { x: 8, y: 9 },
        ]
    }
}
