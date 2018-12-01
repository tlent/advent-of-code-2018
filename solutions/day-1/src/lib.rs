#![allow(dead_code)]

use std::cell::Cell;
use std::collections::HashSet;
use std::num::ParseIntError;

pub fn sum(numbers: &[i32]) -> i32 {
  numbers.iter().sum()
}

// My solution after learning from others (~20% faster and much more readable)
//
// The Cell is necessary because take_while has the value borrowed immutably while the loop tries
// to borrow it mutably. Values cannot be borrowed as mutable while they are borrowed as immutable.
// Cell is implemented using unsafe code that mutates a memory location even though the Cell is
// immutable. This bypasses Rust's normal borrowing rules which are checked at compile time. Rust
// still checks at runtime that the rules are not violated and panics if they are. My understanding is
// that in this case it is safe to bypass these rules because the instructions borrowing the
// frequency cell never use their reference at the same time due to the program being single-threaded.
// https://doc.rust-lang.org/book/second-edition/ch15-05-interior-mutability.html
//
// move on the closure does not work because it must have a reference to the current frequency, not
// a copy since the copy would not change after the closure was first made
//
// take_while replaces the if statement and the past_results.insert statement. It takes advantage of
// HashSet::insert returning true if the value was not already in the set or false otherwise
//
// using for_each here was slightly faster than using a for..in loop
// using nightly rust and the unstable feature cell_update had no performance benefit
pub fn find_repeat_result(numbers: &[i32]) -> i32 {
  let mut past_results = HashSet::new();
  let frequency = Cell::new(0);
  numbers
    .iter()
    .cycle()
    .take_while(|_| past_results.insert(frequency.get()))
    .for_each(|number| frequency.set(frequency.get() + number));
  frequency.get()
}

// My solution before looking at others
pub fn original_find_repeat_result(numbers: &[i32]) -> i32 {
  let mut past_results = HashSet::new();
  let mut result = 0;
  past_results.insert(result);
  for number in numbers.iter().cycle() {
    result += number;
    if past_results.contains(&result) {
      return result;
    }
    past_results.insert(result);
  }
  unreachable!()
}

pub fn parse_input(input: &str) -> Result<Vec<i32>, ParseIntError> {
  input.split_whitespace().map(|x| x.parse()).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sum_returns_correct_result() {
    assert_eq!(sum(&[2, 2]), 4);
    assert_eq!(sum(&[2, 2, -4, 0, 1]), 1);
  }

  #[test]
  fn find_repeat_result_returns_correct_result() {
    assert_eq!(find_repeat_result(&[1, -1]), 0);
    assert_eq!(find_repeat_result(&[3, 3, 4, -2, -4]), 10);
    assert_eq!(find_repeat_result(&[-6, 3, 8, 5, -6]), 5);
    assert_eq!(find_repeat_result(&[7, 7, -2, -7, -4]), 14);
  }
}
