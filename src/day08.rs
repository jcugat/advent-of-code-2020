use crate::utils::load_file;
use std::{path::Path, str::FromStr, todo};

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

pub fn star_one(input: &str) -> i64 {
    let mut program: Vec<Instruction> = input
        .lines()
        .map(str::parse::<Instruction>)
        .map(Result::unwrap)
        .collect();

    let mut instruction_pointer: i64 = 0;
    let mut accumulator: i64 = 0;

    loop {
        let current_instruction = program.get_mut(instruction_pointer as usize).unwrap();
        if current_instruction.execd {
            break;
        }

        accumulator += current_instruction.accum();
        instruction_pointer += current_instruction.next();
    }

    accumulator
}

pub fn star_two(input: &str) -> i64 {
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
        assert_eq!(star_two(&get_input()), 1)
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
