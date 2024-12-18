use anyhow::*;
use itertools::enumerate;
use regex::Regex;
use rustc_hash::FxHashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "17";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

enum OperandType {
    LITERAL,
    COMBO,
}

#[derive(Clone, Copy)]
enum Opcode {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl From<Opcode> for String {
    fn from(value: Opcode) -> Self {
        match value {
            Opcode::ADV => "ADV".to_string(),
            Opcode::BXL => "BXL".to_string(),
            Opcode::BST => "BST".to_string(),
            Opcode::JNZ => "JNZ".to_string(),
            Opcode::BXC => "BXC".to_string(),
            Opcode::OUT => "OUT".to_string(),
            Opcode::BDV => "BDV".to_string(),
            Opcode::CDV => "CDV".to_string(),
        }
    }
}

impl From<usize> for Opcode {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::ADV,
            1 => Self::BXL,
            2 => Self::BST,
            3 => Self::JNZ,
            4 => Self::BXC,
            5 => Self::OUT,
            6 => Self::BDV,
            7 => Self::CDV,
            _ => unreachable!()
        }
    }
}

impl From<Opcode> for OperandType {
    fn from(value: Opcode) -> Self {
        match value {
            Opcode::ADV => Self::COMBO,
            Opcode::BXL => Self::LITERAL,
            Opcode::BST => Self::COMBO,
            Opcode::JNZ => Self::COMBO,
            Opcode::BXC => Self::LITERAL,
            Opcode::OUT => Self::COMBO,
            Opcode::BDV => Self::COMBO,
            Opcode::CDV => Self::COMBO,
        }
    }
}

#[derive(Debug)]
struct Computer {
    a: usize,
    b: usize,
    c: usize,
    pc: usize,
    halted: bool,
    program: Vec<usize>
}

impl Computer {
    fn new_from_input<R: BufRead>(reader: R) -> Self {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut program = Vec::new();

        let regex = Regex::new(r"(\d+)").unwrap();

        for (i, line) in enumerate(reader.lines()) {
            let line = line.unwrap();
            if i == 0 {
                a = regex.captures(&line).unwrap().get(1).unwrap().as_str().parse().unwrap();
            } else if i == 1 {
                b = regex.captures(&line).unwrap().get(1).unwrap().as_str().parse().unwrap();
            } else if i == 2 {
                c = regex.captures(&line).unwrap().get(1).unwrap().as_str().parse().unwrap();
            } else if i == 4 {
                program = regex
                    .captures_iter(&line)
                    .map(|cap| cap.get(0).unwrap().as_str().parse().unwrap())
                    .collect();
            }
        }

        Self {
            a,
            b,
            c,
            pc: 0,
            halted: false,
            program
        }
    }

    fn step(&mut self) {
        if let Some(opcode) = self.program.get(self.pc) {
            let opcode = Opcode::from(*opcode);
            let operand = match OperandType::from(opcode) {
                OperandType::LITERAL => self.program[self.pc + 1],
                OperandType::COMBO => {
                    match self.program[self.pc + 1] {
                        0 => 0,
                        1 => 1,
                        2 => 2,
                        3 => 3,
                        4 => self.a,
                        5 => self.b,
                        6 => self.c,
                        _ => unreachable!(),
                    }
                },
            };

            self.run_opcode(opcode, operand);
            self.pc += 2;
        } else {
            self.halted = true;
        }
    }

    fn run_opcode(&mut self, opcode: Opcode, operand: usize) {
        match opcode {
            Opcode::ADV => self.a /= 2usize.pow(operand as u32),
            Opcode::BXL => self.b ^= operand,
            Opcode::BST => self.b = operand % 8,
            Opcode::JNZ => {
                if self.a != 0 {
                    self.pc = operand - 2;
                }
            },
            Opcode::BXC => self.b ^= self.c,
            Opcode::OUT => print!("{},", operand % 8),
            Opcode::BDV => self.b = self.a / 2usize.pow(operand as u32),
            Opcode::CDV => self.c = self.a / 2usize.pow(operand as u32),
        }
    }

    fn find_quine(&mut self) -> usize {
        let mut quines = FxHashSet::default();
        quines.insert(0);
        for num in self.program.iter().rev() {
            let mut new_quines = FxHashSet::default();
            for curr in quines {
                for i in 0..8 {
                    let new = (curr << 3) + i;
                    if Self::get_out(new) == *num {
                        new_quines.insert(new);
                    }
                }
            }
            quines = new_quines;
        }

        *quines.iter().min().unwrap()
    }

    fn get_out(a: usize) -> usize {
        let partial = (a % 8) ^ 1;
        ((partial ^ (a >> partial)) ^ 5) % 8
    }
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut computer = Computer::new_from_input(reader);
        println!("{:?}", computer);
        while !computer.halted {
            computer.step();
        }
        println!();
        Ok(0)
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut computer = Computer::new_from_input(reader);
        Ok(computer.find_quine())
    }
        
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
