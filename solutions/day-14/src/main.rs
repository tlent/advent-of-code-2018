const INPUT: &str = "330121";

fn solve_part_one(input: usize) -> String {
    let mut recipes = vec![3, 7];
    let mut elf_recipe_indices = vec![0, 1];
    while (recipes.len()) < input + 10 {
        let sum: u8 = elf_recipe_indices.iter().map(|&i| recipes[i]).sum();
        let first_digit = (sum / 10) % 10;
        let second_digit = sum % 10;
        if first_digit > 0 {
            recipes.push(first_digit as u8);
            if (recipes.len()) == input + 10 {
                break;
            }
        }
        recipes.push(second_digit);
        for elf_recipe_index in elf_recipe_indices.iter_mut() {
            let current_recipe = recipes[*elf_recipe_index];
            *elf_recipe_index = (*elf_recipe_index + (current_recipe as usize + 1)) % recipes.len();
        }
    }
    digits_to_string(&recipes[input..])
}

fn solve_part_two(input: &[u8]) -> usize {
    let mut recipes = vec![3, 7];
    let mut elf_recipe_indices = vec![0, 1];
    while recipes.len() < input.len() || recipes[recipes.len() - input.len()..] != *input {
        let sum: u8 = elf_recipe_indices.iter().map(|&i| recipes[i]).sum();
        let first_digit = (sum / 10) % 10;
        let second_digit = sum % 10;
        if first_digit > 0 {
            recipes.push(first_digit as u8);
            if recipes.len() > input.len() && recipes[recipes.len() - input.len()..] == *input {
                break;
            }
        }
        recipes.push(second_digit);
        for elf_recipe_index in elf_recipe_indices.iter_mut() {
            let current_recipe = recipes[*elf_recipe_index];
            *elf_recipe_index = (*elf_recipe_index + (current_recipe as usize + 1)) % recipes.len();
        }
    }
    recipes.len() - input.len()
}

fn str_to_digits(s: &str) -> Vec<u8> {
    s.bytes().map(|b| b - b'0').collect()
}

fn digits_to_string(digits: &[u8]) -> String {
    digits.iter().map(|d| d.to_string()).collect()
}

fn main() {
    println!("{}", solve_part_one(INPUT.parse().unwrap()));
    println!("{}", solve_part_two(&str_to_digits(INPUT)));
}

#[cfg(test)]
mod test {
    use super::*;

    const PART_ONE_SAMPLES: [(usize, &str); 4] = [
        (9, "5158916779"),
        (5, "0124515891"),
        (18, "9251071085"),
        (2018, "5941429882"),
    ];

    const PART_ONE_SOLUTION: &str = "3410710325";

    const PART_TWO_SAMPLES: [(&str, usize); 4] =
        [("51589", 9), ("01245", 5), ("92510", 18), ("59414", 2018)];

    const PART_TWO_SOLUTION: usize = 20216138;

    #[test]
    fn it_solves_part_one_correctly() {
        assert_eq!(solve_part_one(PART_ONE_SAMPLES[0].0), PART_ONE_SAMPLES[0].1);
        assert_eq!(solve_part_one(PART_ONE_SAMPLES[1].0), PART_ONE_SAMPLES[1].1);
        assert_eq!(solve_part_one(PART_ONE_SAMPLES[2].0), PART_ONE_SAMPLES[2].1);
        assert_eq!(solve_part_one(PART_ONE_SAMPLES[3].0), PART_ONE_SAMPLES[3].1);

        assert_eq!(solve_part_one(INPUT.parse().unwrap()), PART_ONE_SOLUTION);
    }

    #[test]
    fn it_solves_part_two_correctly() {
        let inputs_as_digits: Vec<_> = PART_TWO_SAMPLES
            .iter()
            .map(|(input, _)| str_to_digits(input))
            .collect();
        assert_eq!(solve_part_two(&inputs_as_digits[0]), PART_TWO_SAMPLES[0].1);
        assert_eq!(solve_part_two(&inputs_as_digits[1]), PART_TWO_SAMPLES[1].1);
        assert_eq!(solve_part_two(&inputs_as_digits[2]), PART_TWO_SAMPLES[2].1);
        assert_eq!(solve_part_two(&inputs_as_digits[3]), PART_TWO_SAMPLES[3].1);

        assert_eq!(solve_part_two(&str_to_digits(INPUT)), PART_TWO_SOLUTION);
    }
}
