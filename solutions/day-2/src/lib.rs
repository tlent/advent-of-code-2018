use std::collections::HashMap;

pub fn parse_input(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

pub fn calculate_checksum(ids: &[&str]) -> i32 {
    let (two_duplicate_count, three_duplicate_count) = ids.iter().fold((0, 0), |acc, id| {
        let character_counts = id.chars().fold(HashMap::new(), |mut counts, c| {
            *counts.entry(c).or_insert(0) += 1;
            counts
        });
        let has_two_duplicate = character_counts.values().any(|count| *count == 2);
        let has_three_duplicate = character_counts.values().any(|count| *count == 3);
        match (has_two_duplicate, has_three_duplicate) {
            (true, true) => (acc.0 + 1, acc.1 + 1),
            (true, false) => (acc.0 + 1, acc.1),
            (false, true) => (acc.0, acc.1 + 1),
            (false, false) => acc,
        }
    });
    two_duplicate_count * three_duplicate_count
}

pub fn find_similar_id_match(ids: &[&str]) -> String {
    for (index, id_a) in ids.iter().enumerate() {
        for id_b in ids.iter().skip(index) {
            if has_one_char_difference(id_a, id_b) {
                return to_string_of_matching_chars(id_a, id_b);
            }
        }
    }
    panic!("No solution found");
}

pub fn find_similar_id_match_with_cartesian(ids: &[&str]) -> String {
    let mut ids_cartesian_product = cartesian_product(ids.iter(), ids.iter());
    let (id_a, id_b) = ids_cartesian_product
        .find(|(id_a, id_b)| has_one_char_difference(id_a, id_b))
        .expect("No solution found");
    to_string_of_matching_chars(id_a, id_b)
}

fn has_one_char_difference(a: &str, b: &str) -> bool {
    a.chars().zip(b.chars()).filter(|(a, b)| a != b).count() == 1
}

fn to_string_of_matching_chars(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect()
}

// This was taken from rust-itertools and cut down to the important parts for this solution
// Included here to show its implementation and refactored to help my understanding

#[derive(Debug, Clone)]
struct Product<I, J>
where
    I: Iterator,
{
    a: I,
    a_cursor: Option<I::Item>,
    b: J,
    b_original: J,
}

/// Create a new cartesian product iterator
///
/// Iterator element type is `(I::Item, J::Item)`.
fn cartesian_product<I, J>(mut i: I, j: J) -> Product<I, J>
// i is mut because we call next, j gets cloned so doesn't need mut
where
    I: Iterator,
    J: Clone + Iterator,
    I::Item: Clone,
{
    Product {
        // keeps track of the a_item (so next does not need to be called and we don't need to move forward)
        a_cursor: i.next(),
        a: i,
        b: j.clone(),
        b_original: j, // used to reset b to the beginning
    }
}

impl<I, J> Iterator for Product<I, J>
where
    I: Iterator,
    J: Clone + Iterator,
    I::Item: Clone,
{
    type Item = (I::Item, J::Item);
    fn next(&mut self) -> Option<(I::Item, J::Item)> {
        let b_item = match self.b.next() {
            Some(x) => x, // use next element from b as b_item
            None => {
                self.b = self.b_original.clone(); // reset b to the beginning
                match self.b.next() {
                    Some(x) => {
                        self.a_cursor = self.a.next(); // move a forward one item and continue with the reset b
                        x
                    }
                    None => return None, // b was empty?
                }
            }
        };
        match self.a_cursor {
            Some(ref a_item) => Some((a_item.clone(), b_item)),
            None => None, // a has run out of items so the iterator stops
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_correct_checksum() {
        let sample_input = [
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];
        assert_eq!(calculate_checksum(&sample_input), 12);
        let real_input = include_str!("../input");
        let ids = parse_input(real_input);
        assert_eq!(calculate_checksum(&ids), 6723);
    }

    #[test]
    fn it_finds_correct_similar_id_match() {
        let sample_input = [
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ];
        assert_eq!(find_similar_id_match(&sample_input), "fgij");
        let real_input = include_str!("../input");
        let ids = parse_input(real_input);
        assert_eq!(find_similar_id_match(&ids), "prtkqyluiusocwvaezjmhmfgx");

        // Test case that broke my original solution
        let sample_input = ["aaaa", "baaa", "abbb"];
        assert_eq!(find_similar_id_match(&sample_input), "aaa");
    }

    #[test]
    fn it_finds_correct_similar_id_match_with_cartesian() {
        let sample_input = [
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ];
        assert_eq!(find_similar_id_match_with_cartesian(&sample_input), "fgij");
        let real_input = include_str!("../input");
        let ids = parse_input(real_input);
        assert_eq!(
            find_similar_id_match_with_cartesian(&ids),
            "prtkqyluiusocwvaezjmhmfgx"
        );

        // Test case that broke my original solution
        let sample_input = ["aaaa", "baaa", "abbb"];
        assert_eq!(find_similar_id_match_with_cartesian(&sample_input), "aaa");
    }
}
