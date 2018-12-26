const INPUT: &str = include_str!("../input");

#[derive(Debug, Clone, Copy)]
struct FourDimensionalPoint {
    x: isize,
    y: isize,
    z: isize,
    t: isize,
}

impl FourDimensionalPoint {
    fn from_input(input: &str) -> Self {
        let values: Vec<_> = input
            .split(',')
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        FourDimensionalPoint {
            x: values[0],
            y: values[1],
            z: values[2],
            t: values[3],
        }
    }

    fn from_tuple((x, y, z, t): (isize, isize, isize, isize)) -> Self {
        FourDimensionalPoint { x, y, z, t }
    }

    fn manhattan_distance(&self, other: &Self) -> usize {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        let dz = (self.z - other.z).abs();
        let dt = (self.t - other.t).abs();
        (dx + dy + dz + dt) as usize
    }
}

fn find_constellations(points: &[FourDimensionalPoint]) -> Vec<Vec<FourDimensionalPoint>> {
    let mut points = points.to_owned();
    let min_point = *points.iter().min_by_key(|p| p.x + p.y + p.z + p.t).unwrap();
    points.sort_by_key(|p| p.manhattan_distance(&min_point));
    let mut constellations: Vec<Vec<FourDimensionalPoint>> = vec![];
    for &point in points.iter() {
        let constellation = constellations
            .iter_mut()
            .find(|c| c.iter().any(|p| point.manhattan_distance(p) <= 3));
        if let Some(c) = constellation {
            c.push(point);
        } else {
            constellations.push(vec![point]);
        }
    }
    constellations
}

fn parse_input(input: &str) -> Vec<FourDimensionalPoint> {
    input
        .lines()
        .map(FourDimensionalPoint::from_input)
        .collect()
}

fn solve_part_one(points: &[FourDimensionalPoint]) -> usize {
    let constellations = find_constellations(points);
    constellations.len()
}

fn main() {
    let points = parse_input(INPUT);
    println!("{}", solve_part_one(&points));
}

#[cfg(test)]
mod test {
    use super::*;

    type Sample = (&'static str, &'static str, usize);
    const SAMPLE_A: Sample = ("A", include_str!("../sample-a"), 2);
    const SAMPLE_B: Sample = ("B", include_str!("../sample-b"), 4);
    const SAMPLE_C: Sample = ("C", include_str!("../sample-c"), 3);
    const SAMPLE_D: Sample = ("D", include_str!("../sample-d"), 8);

    #[test]
    fn it_solves_part_one_samples_correctly() {
        let samples = [SAMPLE_A, SAMPLE_B, SAMPLE_C, SAMPLE_D];
        for &(name, input, answer) in samples.iter() {
            let points = parse_input(input);
            assert_eq!(
                solve_part_one(&points),
                answer,
                "failed for sample {}",
                name
            );
        }
    }
}
