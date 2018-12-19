use std::collections::HashMap;

const INPUT: &str = include_str!("../input");

#[derive(Debug, PartialEq, Eq, Clone)]
struct Sample {
    before: [u32; 4],
    after: [u32; 4],
    instruction: [u32; 4],
}

impl Sample {
    fn from_input(input: &str) -> Self {
        fn parse_registers(line: &str) -> [u32; 4] {
            let mut result = [0; 4];
            let values: Vec<_> = line[9..19]
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect();
            result.copy_from_slice(&values);
            result
        }
        let mut lines = input.lines();
        Sample {
            before: parse_registers(lines.next().unwrap()),
            instruction: parse_instruction(lines.next().unwrap()),
            after: parse_registers(lines.next().unwrap()),
        }
    }

    fn behaves_like(&self, op: Op) -> bool {
        let mut registers = self.before.clone();
        op.execute(&self.instruction, &mut registers);
        registers == self.after
    }

    fn find_possible_ops(&self) -> Vec<Op> {
        Op::all()
            .into_iter()
            .filter(|op| self.behaves_like(*op))
            .collect()
    }

    fn get_op_number(&self) -> u32 {
        self.instruction[0]
    }
}

fn parse_instruction(line: &str) -> [u32; 4] {
    let mut result = [0; 4];
    let values: Vec<_> = line.split(" ").map(|s| s.parse().unwrap()).collect();
    result.copy_from_slice(&values);
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Op {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl Op {
    fn all() -> Vec<Op> {
        vec![
            Op::Addr,
            Op::Addi,
            Op::Mulr,
            Op::Muli,
            Op::Banr,
            Op::Bani,
            Op::Borr,
            Op::Bori,
            Op::Setr,
            Op::Seti,
            Op::Gtir,
            Op::Gtri,
            Op::Gtrr,
            Op::Eqir,
            Op::Eqri,
            Op::Eqrr,
        ]
    }

    fn execute(&self, instruction: &[u32; 4], registers: &mut [u32; 4]) {
        let a = instruction[1] as u32;
        let b = instruction[2] as u32;
        let c = instruction[3] as usize;
        registers[c] = match self {
            Op::Addr => registers[a as usize] + registers[b as usize],
            Op::Addi => registers[a as usize] + b,
            Op::Mulr => registers[a as usize] * registers[b as usize],
            Op::Muli => registers[a as usize] * b,
            Op::Banr => registers[a as usize] & registers[b as usize],
            Op::Bani => registers[a as usize] & b,
            Op::Borr => registers[a as usize] | registers[b as usize],
            Op::Bori => registers[a as usize] | b,
            Op::Setr => registers[a as usize],
            Op::Seti => a,
            Op::Gtir => {
                if a > registers[b as usize] {
                    1
                } else {
                    0
                }
            }
            Op::Gtri => {
                if registers[a as usize] > b {
                    1
                } else {
                    0
                }
            }
            Op::Gtrr => {
                if registers[a as usize] > registers[b as usize] {
                    1
                } else {
                    0
                }
            }
            Op::Eqir => {
                if a == registers[b as usize] {
                    1
                } else {
                    0
                }
            }
            Op::Eqri => {
                if registers[a as usize] == b {
                    1
                } else {
                    0
                }
            }
            Op::Eqrr => {
                if registers[a as usize] == registers[b as usize] {
                    1
                } else {
                    0
                }
            }
        };
    }
}

fn parse_input(input: &str) -> (Vec<Sample>, String) {
    let parts = input.split("\n\n\n").collect::<Vec<_>>();
    let samples = parts[0].split("\n\n").map(Sample::from_input).collect();
    let program = String::from(parts[1]);
    (samples, program)
}

fn solve_part_one(samples: &[Sample]) -> usize {
    samples
        .iter()
        .map(|s| s.find_possible_ops())
        .filter(|ops| ops.len() >= 3)
        .count()
}

fn find_op_numbers(samples: &[Sample]) -> HashMap<u32, Op> {
    let mut result = HashMap::new();
    let mut unidentified_samples = samples.to_vec();
    while !unidentified_samples.is_empty() {
        for s in unidentified_samples.iter() {
            let possible_ops: Vec<_> = s
                .find_possible_ops()
                .into_iter()
                .filter(|op| result.values().all(|v| v != op))
                .collect();
            if possible_ops.len() == 1 {
                let op = possible_ops[0];
                let number = s.get_op_number();
                result.insert(number, op);
            }
        }
        unidentified_samples.retain(|s| !result.contains_key(&s.get_op_number()))
    }
    result
}

fn run_program(program: &str, op_by_number: &HashMap<u32, Op>) -> u32 {
    let mut registers = [0; 4];
    for line in program.trim().lines() {
        let instruction = parse_instruction(line);
        let op = op_by_number.get(&instruction[0]).unwrap();
        op.execute(&instruction, &mut registers);
    }
    registers[0]
}

fn solve_part_two(samples: &[Sample], program: &str) -> u32 {
    let op_by_number = find_op_numbers(samples);
    run_program(program, &op_by_number)
}

fn main() {
    let (samples, program) = parse_input(INPUT);
    println!("{}", solve_part_one(&samples));
    println!("{}", solve_part_two(&samples, &program));
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_SAMPLE_INPUT: &str = "Before: [3, 2, 1, 1]\n9 2 1 2\nAfter:  [3, 2, 2, 1]";
    const SAMPLE_SAMPLE: Sample = Sample {
        before: [3, 2, 1, 1],
        instruction: [9, 2, 1, 2],
        after: [3, 2, 2, 1],
    };

    #[test]
    fn it_parses_samples_correctly() {
        assert_eq!(Sample::from_input(SAMPLE_SAMPLE_INPUT), SAMPLE_SAMPLE);
    }

    #[test]
    fn it_finds_correct_possible_ops() {
        let mut result = SAMPLE_SAMPLE.find_possible_ops();
        let mut expected = [Op::Mulr, Op::Addi, Op::Seti];
        // sort so order doesn't influence the equality
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }
}
