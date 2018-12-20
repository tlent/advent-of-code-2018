use failure::{ensure, format_err, Error};
use std::str::FromStr;

const INPUT: &str = include_str!("../input");
const VERBOSE: bool = false;

type Registers = [u32; 6];

#[derive(Debug, Clone)]
struct Program {
    instructions: Vec<Instruction>,
    instruction_pointer_register: usize,
    registers: Registers,
}

impl Program {
    fn run(&mut self) {
        loop {
            let ip = self.registers[self.instruction_pointer_register] as usize;
            if ip >= self.instructions.len() {
                break;
            }
            if VERBOSE {
                print!(
                    "ip={} {:?} {:?} {:?} ",
                    ip, self.registers, self.instructions[ip].op, self.instructions[ip].args
                );
            }
            self.instructions[ip].execute(&mut self.registers);
            if VERBOSE {
                println!("{:?}", self.registers);
            }
            self.registers[self.instruction_pointer_register] += 1;
        }
    }

    fn run_to_line(&mut self, line: usize) {
        loop {
            let ip = self.registers[self.instruction_pointer_register] as usize;
            if ip >= self.instructions.len() || ip == line {
                break;
            }
            self.instructions[ip].execute(&mut self.registers);
            self.registers[self.instruction_pointer_register] += 1;
        }
    }
}

impl FromStr for Program {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let split_first_line: Vec<_> = lines
            .next()
            .map(|s| s.split(' ').collect())
            .ok_or_else(|| format_err!("Program cannot be empty"))?;
        ensure!(
            split_first_line[0] == "#ip",
            "Program missing #ip directive"
        );
        let instruction_pointer_register = split_first_line[1].parse()?;
        let instructions = lines.map(|line| line.parse()).collect::<Result<_, _>>()?;
        Ok(Self {
            instructions,
            instruction_pointer_register,
            registers: [0; 6],
        })
    }
}

type InstructionArgs = [u8; 3];

#[derive(Debug, Clone)]
struct Instruction {
    op: Op,
    args: InstructionArgs,
}

impl Instruction {
    fn execute(&self, registers: &mut Registers) {
        self.op.execute(self.args, registers);
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();
        let op = parts[0].parse()?;
        let parsed_args: Vec<_> = parts[1..]
            .iter()
            .map(|v| v.parse())
            .collect::<Result<_, _>>()?;
        let mut args = [0; 3];
        args.copy_from_slice(&parsed_args);
        Ok(Self { op, args })
    }
}

#[derive(Debug, Clone, Copy)]
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
    #[rustfmt::skip]
    fn execute(self, args: InstructionArgs, registers: &mut Registers) {
        let a = args[0] as usize;
        let b = args[1] as usize;
        let c = args[2] as usize;
        registers[c] = match self {
            Op::Addr => registers[a] + registers[b],
            Op::Addi => registers[a] + b as u32,
            Op::Mulr => registers[a] * registers[b],
            Op::Muli => registers[a] * b as u32,
            Op::Banr => registers[a] & registers[b],
            Op::Bani => registers[a] & b as u32,
            Op::Borr => registers[a] | registers[b],
            Op::Bori => registers[a] | b as u32,
            Op::Setr => registers[a],
            Op::Seti => a as u32,
            Op::Gtir => { if a as u32 > registers[b] { 1 } else { 0 } }
            Op::Gtri => { if registers[a] > b as u32 { 1 } else { 0 } }
            Op::Gtrr => { if registers[a] > registers[b] { 1 } else { 0 } }
            Op::Eqir => { if a as u32 == registers[b] { 1 } else { 0 } }
            Op::Eqri => { if registers[a] == b as u32 { 1 } else { 0 } }
            Op::Eqrr => { if registers[a] == registers[b] { 1 } else { 0 } }
        };
    }
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "addr" => Ok(Op::Addr),
            "addi" => Ok(Op::Addi),
            "mulr" => Ok(Op::Mulr),
            "muli" => Ok(Op::Muli),
            "banr" => Ok(Op::Banr),
            "bani" => Ok(Op::Bani),
            "borr" => Ok(Op::Borr),
            "bori" => Ok(Op::Bori),
            "setr" => Ok(Op::Setr),
            "seti" => Ok(Op::Seti),
            "gtir" => Ok(Op::Gtir),
            "gtri" => Ok(Op::Gtri),
            "gtrr" => Ok(Op::Gtrr),
            "eqir" => Ok(Op::Eqir),
            "eqri" => Ok(Op::Eqri),
            "eqrr" => Ok(Op::Eqrr),
            _ => Err(format_err!("Invalid op name: {}", s)),
        }
    }
}

fn sum_factors(n: u32) -> u32 {
    let mut factors = vec![1, n];
    for i in (2u32..).take_while(|i| i * i < n) {
        if n % i == 0 {
            factors.push(i);
            factors.push(n / i);
        }
    }
    factors.iter().sum()
}

fn main() {
    let program: Program = INPUT.parse().unwrap();
    let mut part_one_program = program.clone();
    part_one_program.run();
    println!("Part one: {}", part_one_program.registers[0]);
    let mut part_two_program = program.clone();
    part_two_program.registers[0] = 1;
    part_two_program.run_to_line(1);
    let n = part_two_program.registers[4];
    println!("Part two: {}", sum_factors(n));
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample-input");

    #[test]
    fn it_solves_part_one_correctly() {
        let mut program: Program = SAMPLE_INPUT.parse().unwrap();
        program.run();
        assert_eq!(program.registers[0], 7);
    }
}
