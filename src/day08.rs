use crate::utils::load_file;
use std::{path::Path, str::FromStr};

enum Operation {
    NOP,
    ACC,
    JMP,
}

impl FromStr for Operation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nop" => Ok(Self::NOP),
            "acc" => Ok(Self::ACC),
            "jmp" => Ok(Self::JMP),
            _ => Err(format!("Unknown instruction {}", s)),
        }
    }
}

struct Instruction {
    instr: Operation,
    param: i64,
    execd: bool,
}
impl Instruction {
    fn accum(&self) -> i64 {
        // Returns the value by which the accumulator should be modified
        match self.instr {
            Operation::NOP => 0,
            Operation::ACC => self.param,
            Operation::JMP => 0,
        }
    }

    fn next(&mut self) -> i64 {
        // Returns the next instruction pointer relative to the current one
        self.execd = true;
        match self.instr {
            Operation::NOP => 1,
            Operation::ACC => 1,
            Operation::JMP => self.param,
        }
    }

    fn fix(&mut self) -> bool {
        // Tries to fix the instruction by swapping between JMP and NOP
        match self.instr {
            Operation::NOP => {
                self.instr = Operation::JMP;
                true
            }
            Operation::JMP => {
                self.instr = Operation::NOP;
                true
            }
            _ => false,
        }
    }
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.splitn(2, ' ');
        let instr: Operation = iter.next().unwrap().parse().unwrap();
        let param: i64 = iter.next().unwrap().parse().unwrap();
        Ok(Self {
            instr,
            param,
            execd: false,
        })
    }
}

#[derive(Debug, Clone)]
enum ProgramResult {
    InfiniteLoop(i64),
    OutOfBounds(i64),
}

fn run_program(program: &mut Vec<Instruction>) -> Result<i64, ProgramResult> {
    let mut instruction_pointer: i64 = 0;
    let mut accumulator: i64 = 0;

    // Reset the program
    program.iter_mut().for_each(|instr| instr.execd = false);

    loop {
        // Got to the end of the program successfully
        if instruction_pointer == program.len() as i64 {
            return Ok(accumulator);
        }

        // Retrieve the current instruction, or fail because we did an invalid jump
        let current_instruction = match program.get_mut(instruction_pointer as usize) {
            Some(current_instruction) => current_instruction,
            None => return Err(ProgramResult::OutOfBounds(accumulator)),
        };

        // When an instruction has already been executed it means we're in an infinite loop
        if current_instruction.execd {
            return Err(ProgramResult::InfiniteLoop(accumulator));
        }

        accumulator += current_instruction.accum();
        instruction_pointer += current_instruction.next();
    }
}

pub fn star_one(input: &str) -> i64 {
    let mut program: Vec<Instruction> = input
        .lines()
        .map(str::parse::<Instruction>)
        .map(Result::unwrap)
        .collect();

    if let ProgramResult::InfiniteLoop(accumulator) = run_program(&mut program).err().unwrap() {
        return accumulator;
    }
    panic!("Solution not found")
}

pub fn star_two(input: &str) -> i64 {
    let mut program: Vec<Instruction> = input
        .lines()
        .map(str::parse::<Instruction>)
        .map(Result::unwrap)
        .collect();

    for instruction_to_fix in 0..program.len() {
        // Try to fix the instruction
        // In case it's not possible, there's no need to run the program
        if !program[instruction_to_fix].fix() {
            continue;
        }

        if let Ok(accumulator) = run_program(&mut program) {
            return accumulator;
        } else {
            // Unfix the instruction since this wasn't the one that was broken
            program[instruction_to_fix].fix();
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::{get_input, star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(&get_input()), 5)
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(&get_input()), 8)
    }
}

fn get_input() -> String {
    let current_path = Path::new(&file!());
    let current_path_extension =
        format!(".{}", current_path.extension().unwrap().to_str().unwrap());
    let current_name = current_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .trim_end_matches(&current_path_extension);
    let file_name = format!("{}_example.txt", current_name);
    load_file(&file_name)
}
