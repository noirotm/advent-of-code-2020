use crate::solver::{ReadExt, Solver};
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Instr>;
    type Output1 = i32;
    type Output2 = i32;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        exec_program(input).0
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        (0..input.len())
            .flat_map(|i| alter_program_at_index(input, i))
            .find_map(|p| match exec_program(&p) {
                (acc, ExitStatus::Ok) => Some(acc),
                (_, ExitStatus::Loop) => None,
            })
            .unwrap_or_default()
    }
}

#[derive(Clone)]
pub enum Instr {
    Nop(isize),
    Jmp(isize),
    Acc(i32),
}

impl FromStr for Instr {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(jmp|acc|nop) ([-+]\d+)$").unwrap();
        }
        let caps = RE.captures(s).ok_or("No match")?;

        match &caps[1] {
            "nop" => Ok(Self::Nop(caps[2].parse()?)),
            "acc" => Ok(Self::Acc(caps[2].parse()?)),
            "jmp" => Ok(Self::Jmp(caps[2].parse()?)),
            _ => Err("Unknown instruction".into()),
        }
    }
}

enum ExitStatus {
    Ok,
    Loop,
}

fn exec_program(program: &[Instr]) -> (i32, ExitStatus) {
    let mut ip = 0;
    let mut acc = 0;
    let mut exec_map = vec![0; program.len()];

    loop {
        if let Some(&n) = exec_map.get(ip) {
            if n > 0 {
                return (acc, ExitStatus::Loop);
            }
        } else {
            return (acc, ExitStatus::Ok);
        }

        exec_map[ip] += 1;

        match &program[ip] {
            Instr::Nop(_) => {
                ip += 1;
            }
            Instr::Jmp(offset) => {
                ip = ((ip as isize) + offset) as usize;
            }
            Instr::Acc(value) => {
                acc += value;
                ip += 1;
            }
        }
    }
}

fn alter_program_at_index(program: &[Instr], i: usize) -> Option<Vec<Instr>> {
    match program.get(i) {
        Some(&Instr::Nop(offset)) => {
            let mut prg_clone = program.to_vec();
            prg_clone[i] = Instr::Jmp(offset);
            Some(prg_clone)
        }
        Some(&Instr::Jmp(offset)) => {
            let mut prg_clone = program.to_vec();
            prg_clone[i] = Instr::Nop(offset);
            Some(prg_clone)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instr() {
        let i = Instr::from_str("jmp +32");
        assert!(matches!(i, Ok(Instr::Jmp(32))));
    }
}
