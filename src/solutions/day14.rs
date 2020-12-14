use crate::solver::{ReadExt, Solver};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Instruction>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        execute_program(&input)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        execute_program_v2(&input)
    }
}

#[derive(Debug)]
pub enum Instruction {
    SetMask(Mask),
    WriteMemory(usize, u64),
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref SETMASK_RE: Regex = Regex::new(r"^mask = ([10X]{36})$").unwrap();
            static ref WRITEMEM_RE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
        }

        if let Some(c) = SETMASK_RE.captures(s) {
            let mask = Mask::from_str(&c[1])?;
            Ok(Self::SetMask(mask))
        } else if let Some(c) = WRITEMEM_RE.captures(s) {
            let address = c[1].parse()?;
            let value = c[2].parse()?;
            Ok(Self::WriteMemory(address, value))
        } else {
            Err("No match".into())
        }
    }
}

#[derive(Debug)]
pub struct Mask(Vec<Option<u8>>);

impl FromStr for Mask {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Mask(
            s.bytes()
                .map(|b| match b {
                    b'1' => Some(1),
                    b'0' => Some(0),
                    _ => None,
                })
                .collect(),
        ))
    }
}

impl Mask {
    fn apply(&self, val: u64) -> u64 {
        let mut output = 0;
        for (i, bitmask) in self.0.iter().enumerate() {
            output <<= 1;
            let bit = self.0.len() - (i + 1);
            if let Some(bitmask) = *bitmask {
                output |= bitmask as u64;
            } else {
                let v = (val >> bit) & 1;
                output |= v;
            }
        }
        output
    }

    fn addresses(&self, address: u64) -> Vec<u64> {
        let mut queue = VecDeque::new();

        queue.push_back(0);

        for (i, bitmask) in self.0.iter().enumerate() {
            let bit = self.0.len() - (i + 1);
            let current_address_bit = (address >> bit) & 1;

            let n = queue.len();
            for _ in 0..n {
                if let Some(mut addr) = queue.pop_front() {
                    addr <<= 1;
                    if let Some(bitmask) = *bitmask {
                        if bitmask == 1 {
                            addr |= 1;
                        } else {
                            addr |= current_address_bit;
                        }
                        queue.push_back(addr);
                    } else {
                        queue.push_back(addr);
                        addr |= 1;
                        queue.push_back(addr);
                    }
                }
            }
        }

        queue.into()
    }
}

fn execute_program(program: &[Instruction]) -> u64 {
    let mut memory = vec![0u64; 65536];
    let mut current_mask = None;

    for instr in program {
        match instr {
            Instruction::SetMask(m) => {
                current_mask = Some(m);
            }
            Instruction::WriteMemory(address, value) => {
                if let Some(v) = memory.get_mut(*address) {
                    *v = current_mask.map_or(0, |m| m.apply(*value));
                }
            }
        }
    }

    memory.iter().sum()
}

fn execute_program_v2(program: &[Instruction]) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut current_mask = None;

    for instr in program {
        match instr {
            Instruction::SetMask(m) => {
                current_mask = Some(m);
            }
            Instruction::WriteMemory(address, value) => {
                if let Some(m) = current_mask {
                    for addr in m.addresses(*address as u64) {
                        memory.insert(addr, *value);
                    }
                }
            }
        }
    }

    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_apply() {
        let m = Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
        assert_eq!(m.apply(11), 73);
        assert_eq!(m.apply(101), 101);
        assert_eq!(m.apply(0), 64);
    }

    #[test]
    fn test_mask_addresses() {
        let m = Mask::from_str("000000000000000000000000000000X1001X").unwrap();
        assert_eq!(m.addresses(42), vec![26, 27, 58, 59]);

        let m = Mask::from_str("00000000000000000000000000000000X0XX").unwrap();
        assert_eq!(m.addresses(26), vec![16, 17, 18, 19, 24, 25, 26, 27]);
    }
}
