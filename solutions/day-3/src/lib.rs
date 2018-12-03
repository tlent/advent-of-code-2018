extern crate regex;

use regex::Regex;
use std::collections::HashMap;

const PARSE_REGEX: &'static str = r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)";

pub fn parse_input(input: &str) -> Vec<FabricClaim> {
  let re = Regex::new(PARSE_REGEX).unwrap();
  re.captures_iter(input)
    .map(|capture| FabricClaim {
      id: capture.get(1).unwrap().as_str().parse().unwrap(),
      coordinates: Point {
        x: capture.get(2).unwrap().as_str().parse().unwrap(),
        y: capture.get(3).unwrap().as_str().parse().unwrap(),
      },
      width: capture.get(4).unwrap().as_str().parse().unwrap(),
      height: capture.get(5).unwrap().as_str().parse().unwrap(),
    }).collect()
}

#[derive(PartialEq, Debug, Clone)]
pub struct FabricClaim {
  pub id: u32,
  pub coordinates: Point,
  pub width: u32,
  pub height: u32,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct Point {
  pub x: u32,
  pub y: u32,
}

pub fn count_overlapping_fabric_claim_units(claims: &Vec<FabricClaim>) -> usize {
  get_claim_count_by_coordinate_map(claims)
    .values()
    .filter(|value| **value > 1)
    .count()
}

pub fn find_fabric_claim_with_no_overlap(claims: &Vec<FabricClaim>) -> &FabricClaim {
  let claim_count_by_coordinate = get_claim_count_by_coordinate_map(claims);
  &claims
    .iter()
    .find(|claim| {
      for x in claim.coordinates.x..(claim.coordinates.x + claim.width) {
        for y in claim.coordinates.y..(claim.coordinates.y + claim.height) {
          if *claim_count_by_coordinate.get(&Point { x, y }).unwrap() > 1 {
            return false;
          }
        }
      }
      true
    }).expect("No solution found")
}

fn get_claim_count_by_coordinate_map(claims: &Vec<FabricClaim>) -> HashMap<Point, u32> {
  claims.iter().fold(HashMap::new(), |mut map, claim| {
    for x in claim.coordinates.x..(claim.coordinates.x + claim.width) {
      for y in claim.coordinates.y..(claim.coordinates.y + claim.height) {
        *map.entry(Point { x, y }).or_insert(0) += 1;
      }
    }
    map
  })
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_parses_input_correctly() {
    let sample_input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
    assert_eq!(parse_input(sample_input), get_sample_claims());
  }

  #[test]
  fn it_counts_overlapping_fabric_claim_units_correctly() {
    assert_eq!(
      count_overlapping_fabric_claim_units(&get_sample_claims().to_vec()),
      4
    );
    assert_eq!(
      count_overlapping_fabric_claim_units(&get_real_input_claims()),
      100261
    );
  }

  #[test]
  fn it_finds_correct_fabric_claim_with_no_overlap() {
    assert_eq!(
      find_fabric_claim_with_no_overlap(&get_sample_claims().to_vec()).id,
      3
    );
    assert_eq!(
      find_fabric_claim_with_no_overlap(&get_real_input_claims()).id,
      251
    );
  }

  fn get_sample_claims() -> [FabricClaim; 3] {
    [
      FabricClaim {
        id: 1,
        coordinates: Point { x: 1, y: 3 },
        width: 4,
        height: 4,
      },
      FabricClaim {
        id: 2,
        coordinates: Point { x: 3, y: 1 },
        width: 4,
        height: 4,
      },
      FabricClaim {
        id: 3,
        coordinates: Point { x: 5, y: 5 },
        width: 2,
        height: 2,
      },
    ]
  }

  // relies on correct parse_input
  fn get_real_input_claims() -> Vec<FabricClaim> {
    let real_input = include_str!("../input");
    parse_input(real_input)
  }
}
