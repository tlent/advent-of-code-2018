use std::cmp::Ordering;
use std::collections::BinaryHeap;

const INPUT: &str = include_str!("../input");

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn manhattan_distance(&self, b: Self) -> usize {
        let dx = (self.x - b.x).abs() as usize;
        let dy = (self.y - b.y).abs() as usize;
        let dz = (self.z - b.z).abs() as usize;
        dx + dy + dz
    }
}

struct Bot {
    position: Point,
    range: usize,
}

impl Bot {
    fn from_input(line: &str) -> Self {
        let open = line.chars().position(|c| c == '<').unwrap();
        let close = line.chars().position(|c| c == '>').unwrap();
        let position: Vec<isize> = line[open + 1..close]
            .split(',')
            .map(|d| d.parse())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let position = Point {
            x: position[0],
            y: position[1],
            z: position[2],
        };
        let r_equal = line.rfind(|c| c == '=').unwrap();
        let range: usize = line[r_equal + 1..].parse().unwrap();
        Self { position, range }
    }
}

fn solve_part_one(bots: &[Bot]) -> usize {
    let strongest_bot = bots.iter().max_by_key(|b| b.range).unwrap();
    bots.iter()
        .filter(|b| strongest_bot.position.manhattan_distance(b.position) < strongest_bot.range)
        .count()
}

#[derive(Debug, PartialEq, Eq)]
struct Cube {
    position: Point,
    size: usize,
}

impl Cube {
    fn minimum_spanning_cube(bots: &[Bot]) -> Self {
        let flat_positions: Vec<_> = bots
            .iter()
            .map(|b| {
                let Point { x, y, z } = b.position;
                vec![x, y, z]
            })
            .flatten()
            .collect();
        let min = *flat_positions.iter().min().unwrap();
        let max = *flat_positions.iter().max().unwrap();
        Self {
            position: Point {
                x: min,
                y: min,
                z: min,
            },
            size: (max - min) as usize,
        }
    }

    fn contains_bot(&self, bot: &Bot) -> bool {
        let cube_pos = self.position;
        let bot_pos = bot.position;
        let size = self.size as isize;
        let contains_x = cube_pos.x < bot_pos.x && bot_pos.x < cube_pos.x + size;
        let contains_y = cube_pos.y < bot_pos.y && bot_pos.y < cube_pos.y + size;
        let contains_z = cube_pos.z < bot_pos.z && bot_pos.z < cube_pos.z + size;
        contains_x && contains_y && contains_z
    }

    fn intersects_bot(&self, bot: &Bot) -> bool {
        if self.contains_bot(bot) {
            return true;
        }
        let size = self.size as isize;
        let max_x = self.position.x + size;
        let max_y = self.position.y + size;
        let max_z = self.position.z + size;
        let mut distance = 0;
        distance += if bot.position.x > max_x {
            bot.position.x - max_x
        } else {
            self.position.x - bot.position.x
        };
        distance += if bot.position.y > max_y {
            bot.position.y - max_y
        } else {
            self.position.y - bot.position.y
        };
        distance += if bot.position.z > max_z {
            bot.position.z - max_z
        } else {
            self.position.z - bot.position.z
        };
        distance <= bot.range as isize
    }

    fn count_intersecting_bots(&self, bots: &[Bot]) -> usize {
        bots.iter().filter(|b| self.intersects_bot(b)).count()
    }

    fn subdivide(&self) -> Vec<Cube> {
        let size = self.size / 2;
        vec![
            Cube {
                position: Point {
                    x: self.position.x,
                    y: self.position.y,
                    z: self.position.z,
                },
                size,
            },
            Cube {
                position: Point {
                    x: self.position.x,
                    y: self.position.y,
                    z: self.position.z + size as isize,
                },
                size,
            },
            Cube {
                position: Point {
                    x: self.position.x,
                    y: self.position.y + size as isize,
                    z: self.position.z,
                },
                size,
            },
            Cube {
                position: Point {
                    x: self.position.x,
                    y: self.position.y + size as isize,
                    z: self.position.z + size as isize,
                },
                size,
            },
            Cube {
                position: Point {
                    x: self.position.x + size as isize,
                    y: self.position.y,
                    z: self.position.z,
                },
                size,
            },
            Cube {
                position: Point {
                    x: self.position.x + size as isize,
                    y: self.position.y,
                    z: self.position.z + size as isize,
                },
                size,
            },
            Cube {
                position: Point {
                    x: self.position.x + size as isize,
                    y: self.position.y + size as isize,
                    z: self.position.z,
                },
                size,
            },
            Cube {
                position: Point {
                    x: self.position.x + size as isize,
                    y: self.position.y + size as isize,
                    z: self.position.z + size as isize,
                },
                size,
            },
        ]
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HeapWrapper {
    cube: Cube,
    bot_count: usize,
}

impl HeapWrapper {
    fn from_cube(cube: Cube, bots: &[Bot]) -> Self {
        let bot_count = cube.count_intersecting_bots(bots);
        Self { cube, bot_count }
    }
}

impl Ord for HeapWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.bot_count.cmp(&other.bot_count) {
            Ordering::Equal => {
                let other_distance = other.cube.position.manhattan_distance(Point::default());
                let own_distance = self.cube.position.manhattan_distance(Point::default());
                other_distance.cmp(&own_distance) // note reversed order
            }
            order => order,
        }
    }
}

impl PartialOrd for HeapWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve_part_two(bots: &[Bot]) -> usize {
    let initial_cube = Cube::minimum_spanning_cube(bots);
    let mut cubes = BinaryHeap::new();
    cubes.push(HeapWrapper::from_cube(initial_cube, bots));
    while !cubes.is_empty() {
        let HeapWrapper { cube, .. } = cubes.pop().unwrap();
        if cube.size == 0 {
            return cube.position.manhattan_distance(Point::default());
        }
        cubes.extend(
            cube.subdivide()
                .into_iter()
                .map(|c| HeapWrapper::from_cube(c, bots)),
        );
    }
    panic!("No solution found")
}

fn main() {
    let bots: Vec<_> = INPUT.lines().map(Bot::from_input).collect();
    println!("{}", solve_part_one(&bots));
    println!("{}", solve_part_two(&bots));
}
