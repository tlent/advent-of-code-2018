use std::{collections::HashSet, fmt, mem};

const INPUT: &str = include_str!("../input");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Coordinate {
    y: usize,
    x: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn from_character(c: char) -> Option<Self> {
        match c {
            '^' => Some(Direction::Up),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            _ => None,
        }
    }

    fn to_character(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn curve(&self, curve_direction: &CurveDirection) -> Self {
        match curve_direction {
            CurveDirection::Right => {
                match self {
                    //  /- to  >-
                    //  ^      |
                    Direction::Up => self.turn_right(),
                    //  v  to  |
                    // -/     -<
                    Direction::Down => self.turn_right(),
                    //  |  to  |
                    // >/     -^
                    Direction::Right => self.turn_left(),
                    //  /< to  v-
                    //  |      |
                    Direction::Left => self.turn_left(),
                }
            }
            CurveDirection::Left => {
                match self {
                    // -\  to -<
                    //  ^      |
                    Direction::Up => self.turn_left(),
                    //  v  to  |
                    //  \-     >-
                    Direction::Down => self.turn_left(),
                    // >\  to -v
                    //  |      |
                    Direction::Right => self.turn_right(),
                    //  |  to  |
                    //  \<     ^-
                    Direction::Left => self.turn_right(),
                }
            }
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_character())
    }
}

#[derive(Debug, Clone)]
enum TrackOrientation {
    Vertical,
    Horizontal,
}

/// Curve directions are based on the direction a cart moving up or down would turn.
/// A cart moving left or right would turn the opposite way.
///
/// Examples:
/// - current direction up, turn right -> right
/// - current direction down, turn right -> left
/// - current direction right, turn right -> up
/// - current direction left, turn right -> down
#[derive(Debug, Clone)]
enum CurveDirection {
    Left,
    Right,
}

#[derive(Debug, Clone)]
enum Track {
    Intersection,
    Curve(CurveDirection),
    Straight(TrackOrientation),
}

impl Track {
    fn from_character(c: char) -> Option<Self> {
        match c {
            '|' => Some(Track::Straight(TrackOrientation::Vertical)),
            '-' => Some(Track::Straight(TrackOrientation::Horizontal)),
            '\\' => Some(Track::Curve(CurveDirection::Left)),
            '/' => Some(Track::Curve(CurveDirection::Right)),
            '+' => Some(Track::Intersection),
            '^' | 'v' => Some(Track::Straight(TrackOrientation::Vertical)),
            '<' | '>' => Some(Track::Straight(TrackOrientation::Horizontal)),
            _ => None,
        }
    }

    fn to_character(&self) -> char {
        match *self {
            Track::Straight(TrackOrientation::Vertical) => '|',
            Track::Straight(TrackOrientation::Horizontal) => '-',
            Track::Curve(CurveDirection::Left) => '\\',
            Track::Curve(CurveDirection::Right) => '/',
            Track::Intersection => '+',
        }
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_character())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cart {
    direction: Direction,
    position: Coordinate,
    intersections_hit: u32,
}

impl Cart {
    fn new(position: Coordinate, direction: Direction) -> Self {
        Self {
            position,
            direction,
            intersections_hit: 0,
        }
    }

    fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => self.position.y -= 1,
            Direction::Down => self.position.y += 1,
            Direction::Right => self.position.x += 1,
            Direction::Left => self.position.x -= 1,
        }
    }

    fn reorient(&mut self, track: &Track) {
        let current_direction = mem::replace(&mut self.direction, Direction::Up);
        self.direction = match track {
            Track::Curve(curve_direction) => current_direction.curve(curve_direction),
            Track::Intersection => {
                let new_direction = match self.intersections_hit % 3 {
                    0 => current_direction.turn_left(),
                    1 => current_direction,
                    2 => current_direction.turn_right(),
                    _ => unreachable!(),
                };
                self.intersections_hit += 1;
                new_direction
            }
            Track::Straight(_) => current_direction,
        }
    }
}

impl fmt::Display for Cart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.direction)
    }
}

#[derive(Debug, Clone)]
struct Map {
    tracks: Vec<Vec<Option<Track>>>,
    carts: Vec<Cart>,
}

impl Map {
    fn from_input(input: &str) -> Self {
        let mut carts = vec![];
        let mut tracks = vec![];
        for (i, line) in input.lines().enumerate() {
            let mut track_row = vec![];
            for (j, c) in line.chars().enumerate() {
                if let Some(d) = Direction::from_character(c) {
                    carts.push(Cart::new(Coordinate { x: j, y: i }, d));
                }
                track_row.push(Track::from_character(c));
            }
            tracks.push(track_row);
        }
        Self { carts, tracks }
    }

    fn find_first_crash(&mut self) -> Coordinate {
        loop {
            self.carts.sort_by_key(|c| c.position);
            let mut cart_positions: HashSet<_> = self.carts.iter().map(|c| c.position).collect();
            for cart in self.carts.iter_mut() {
                cart_positions.remove(&cart.position);
                cart.move_forward();
                let track = &self.tracks[cart.position.y][cart.position.x];
                let track = track.as_ref().expect("Cart off the rails!!!");
                cart.reorient(track);
                if !cart_positions.insert(cart.position) {
                    return cart.position;
                }
            }
        }
    }

    fn find_last_cart(&mut self) -> Coordinate {
        loop {
            self.carts.sort_by_key(|c| c.position);
            let mut cart_positions: HashSet<_> = self.carts.iter().map(|c| c.position).collect();
            let mut crash_positions = HashSet::new();
            for cart in self.carts.iter_mut() {
                if crash_positions.contains(&cart.position) {
                    continue;
                }
                cart_positions.remove(&cart.position);
                cart.move_forward();
                let track = &self.tracks[cart.position.y][cart.position.x];
                let track = track.as_ref().expect("Cart off the rails!!!");
                cart.reorient(track);
                if !cart_positions.insert(cart.position) {
                    crash_positions.insert(cart.position);
                }
            }
            self.carts
                .retain(|c| !crash_positions.contains(&c.position));
            if self.carts.len() == 1 {
                return self.carts[0].position;
            }
            if self.carts.is_empty() {
                panic!("All carts removed");
            }
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, row) in self.tracks.iter().enumerate() {
            for (j, track) in row.iter().enumerate() {
                let carts_at_current_position: Vec<_> = self
                    .carts
                    .iter()
                    .filter(|c| c.position.x == j && c.position.y == i)
                    .collect();
                if carts_at_current_position.len() == 1 {
                    write!(f, "{}", carts_at_current_position[0])?;
                } else if carts_at_current_position.len() > 1 {
                    write!(f, "{}", 'X')?;
                } else {
                    let out: Box<dyn fmt::Display> = match track {
                        Some(t) => Box::new(t),
                        None => Box::new(' '),
                    };
                    write!(f, "{}", out)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let m = Map::from_input(INPUT);
    let Coordinate { x, y } = m.clone().find_first_crash();
    println!("{},{}", x, y);
    let Coordinate { x, y } = m.clone().find_last_cart();
    println!("{},{}", x, y);
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample-input");
    const PART_TWO_SAMPLE_INPUT: &str = include_str!("../part-two-sample-input");

    #[test]
    fn it_parses_input_correctly() {
        let m = Map::from_input(SAMPLE_INPUT);
        // relies on display impls being correct as well
        assert_eq!(format!("{}", m), SAMPLE_INPUT);
    }

    #[test]
    fn it_finds_correct_first_crash_coordinate() {
        let mut m = Map::from_input(SAMPLE_INPUT);
        let coord = m.find_first_crash();
        assert_eq!((coord.x, coord.y), (7, 3));

        let mut m = Map::from_input(INPUT);
        let coord = m.find_first_crash();
        assert_eq!((coord.x, coord.y), (82, 104));
    }

    #[test]
    fn it_finds_correct_last_cart_coordinate() {
        let mut m = Map::from_input(PART_TWO_SAMPLE_INPUT);
        let coord = m.find_last_cart();
        assert_eq!((coord.x, coord.y), (6, 4));

        let mut m = Map::from_input(INPUT);
        let coord = m.find_last_cart();
        assert_eq!((coord.x, coord.y), (121, 22));
    }
}
