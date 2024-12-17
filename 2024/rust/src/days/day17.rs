use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use num_bigint::ToBigUint;

type LiteralOperand = u8;

#[derive(Clone, Copy, Debug)]
enum ComboOperand {
    Literal(u8),
    RegisterA,
    RegisterB,
    RegisterC,
    // Reserved,
}

impl TryFrom<u8> for ComboOperand {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x @ (0..=3) => Ok(ComboOperand::Literal(x)),
            4 => Ok(ComboOperand::RegisterA),
            5 => Ok(ComboOperand::RegisterB),
            6 => Ok(ComboOperand::RegisterC),
            _ => Err(anyhow::anyhow!("unrecognized combo operand: {value}")),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Adv { operand: ComboOperand },
    Bxl { operand: LiteralOperand },
    Bst { operand: ComboOperand },
    Jnz { operand: LiteralOperand },
    Bxc,
    Out { operand: ComboOperand },
    Bdv { operand: ComboOperand },
    Cdv { operand: ComboOperand },
}

impl Instruction {
    fn new(opcode: u8, operand: u8) -> Result<Self, anyhow::Error> {
        match opcode {
            0 => Ok(Instruction::Adv {
                operand: ComboOperand::try_from(operand)?,
            }),

            1 => Ok(Instruction::Bxl { operand }),

            2 => Ok(Instruction::Bst {
                operand: ComboOperand::try_from(operand)?,
            }),

            3 => Ok(Instruction::Jnz { operand }),

            4 => Ok(Instruction::Bxc),

            5 => Ok(Instruction::Out {
                operand: ComboOperand::try_from(operand)?,
            }),

            6 => Ok(Instruction::Bdv {
                operand: ComboOperand::try_from(operand)?,
            }),

            7 => Ok(Instruction::Cdv {
                operand: ComboOperand::try_from(operand)?,
            }),

            _ => Err(anyhow::anyhow!("unrecognized opcode: {opcode}")),
        }
    }
}

type Register = num_bigint::BigUint;

fn parse_register(line: &str) -> Result<Register, anyhow::Error> {
    line.split_once(": ")
        .ok_or(anyhow::anyhow!("unexpected register format"))?
        .1
        .parse()
        .map_err(anyhow::Error::msg)
}

#[derive(Clone)]
struct Program {
    register_a: Register,
    register_b: Register,
    register_c: Register,

    instructions: Vec<Instruction>,
    instruction_pointer: usize,

    output: Vec<String>,
    raw_instructions: String,
}

impl Program {
    fn from_file(path: &Path) -> Self {
        let file = File::open(path).unwrap();

        let lines = BufReader::new(file)
            .lines()
            .map(Result::unwrap)
            .collect::<Vec<_>>();

        let register_a = parse_register(&lines[0]).unwrap();
        let register_b = parse_register(&lines[1]).unwrap();
        let register_c = parse_register(&lines[2]).unwrap();

        let raw_instructions = lines[4].split_once(": ").unwrap().1.to_string();

        let instructions = raw_instructions
            .split(',')
            .map(|digit| digit.parse::<u8>().unwrap())
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|chunk| Instruction::new(chunk[0], chunk[1]).unwrap())
            .collect::<Vec<_>>();

        Program {
            register_a,
            register_b,
            register_c,
            instructions,
            instruction_pointer: 0,
            output: vec![],
            raw_instructions,
        }
    }

    fn dereference_combo_operand(&self, operand: ComboOperand) -> num_bigint::BigUint {
        match operand {
            ComboOperand::Literal(value) => num_bigint::BigUint::from(value),
            ComboOperand::RegisterA => self.register_a.clone(),
            ComboOperand::RegisterB => self.register_b.clone(),
            ComboOperand::RegisterC => self.register_c.clone(),
        }
    }

    fn perform_next_instruction(&mut self) -> bool {
        if self.instruction_pointer >= self.instructions.len() {
            return false;
        }

        match &self.instructions[self.instruction_pointer] {
            Instruction::Adv { operand } => {
                let operand = self.dereference_combo_operand(*operand);

                let denominator = 2_usize
                    .to_biguint()
                    .unwrap()
                    .pow(operand.try_into().unwrap());

                self.register_a /= denominator;
            }

            Instruction::Bxl { operand } => self.register_b ^= (*operand).to_biguint().unwrap(),

            Instruction::Bst { operand } => {
                let operand = self.dereference_combo_operand(*operand);

                self.register_b = operand % 8_usize.to_biguint().unwrap();
            }

            Instruction::Jnz { operand } => {
                if self.register_a != num_bigint::BigUint::ZERO {
                    assert!(*operand % 2 == 0);

                    self.instruction_pointer = usize::from(*operand) / 2;

                    return true;
                }
            }

            Instruction::Bxc => self.register_b ^= self.register_c.clone(),

            Instruction::Out { operand } => {
                let operand = self.dereference_combo_operand(*operand);

                self.output
                    .push((operand % 8_usize.to_biguint().unwrap()).to_string());
            }

            Instruction::Bdv { operand } => {
                let operand = self.dereference_combo_operand(*operand);

                let numerator = self.register_a.clone();
                let denominator = 2_usize
                    .to_biguint()
                    .unwrap()
                    .pow(operand.try_into().unwrap());

                self.register_b = numerator / denominator;
            }

            Instruction::Cdv { operand } => {
                let operand = self.dereference_combo_operand(*operand);

                let numerator = self.register_a.clone();
                let denominator = 2_usize
                    .to_biguint()
                    .unwrap()
                    .pow(operand.try_into().unwrap());

                self.register_c = numerator / denominator;
            }
        };

        self.instruction_pointer += 1;

        true
    }

    fn get_output(&self) -> String {
        self.output.join(",")
    }
}

pub fn solve_part1(path: &Path) -> String {
    let mut program = Program::from_file(path);

    while program.perform_next_instruction() {}

    program.get_output()
}

pub fn solve_part2(path: &Path) -> String {
    let is_full_input = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .contains("input");

    if !is_full_input {
        return String::from("N/A");
    }

    let program = Program::from_file(path);

    let mut partial_results = BinaryHeap::new();
    partial_results.push(Reverse(0.to_biguint().unwrap()));

    while let Some(partial_result) = partial_results.pop() {
        for a in (0..8).map(|num| -> num_bigint::BigUint {
            (partial_result.0.clone() << 3) + num.to_biguint().unwrap()
        }) {
            let mut test_run = program.clone();
            test_run.register_a.clone_from(&a);

            while test_run.perform_next_instruction() {}

            if program.raw_instructions == test_run.get_output() {
                return a.to_string();
            }

            if program.raw_instructions.ends_with(&test_run.get_output()) {
                partial_results.push(Reverse(a));
            }
        }
    }

    unreachable!()
}
