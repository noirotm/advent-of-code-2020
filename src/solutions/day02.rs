use crate::solver::{ReadExt, Solver};
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<PasswordEntry>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().filter(|e| e.is_valid()).count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input.iter().filter(|e| e.is_valid_correct()).count()
    }
}

pub struct PasswordEntry {
    i1: usize,
    i2: usize,
    policy: char,
    password: String,
}

impl FromStr for PasswordEntry {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();
        }

        let cap = RE.captures(s).ok_or("no match")?;

        Ok(Self {
            i1: cap[1].parse()?,
            i2: cap[2].parse()?,
            policy: cap[3].chars().next().ok_or("missing policy")?,
            password: cap[4].into(),
        })
    }
}

impl PasswordEntry {
    fn is_valid(&self) -> bool {
        let c = self.password.chars().filter(|&c| c == self.policy).count();
        c >= self.i1 && c <= self.i2
    }

    fn is_valid_correct(&self) -> bool {
        let a = self.password.chars().nth(&self.i1 - 1);
        let b = self.password.chars().nth(&self.i2 - 1);

        if let (Some(a), Some(b)) = (a, b) {
            (a == self.policy && b != self.policy) || (a != self.policy && b == self.policy)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let s = "1-3 a: abcde";
        let p = PasswordEntry::from_str(s).unwrap();
        assert_eq!(p.i1, 1);
        assert_eq!(p.i2, 3);
        assert_eq!(p.policy, 'a');
        assert_eq!(p.password, "abcde");
    }

    #[test]
    fn test_is_valid() {
        let passwords = &[
            ("1-1 a: a", true),
            ("4-5 a: aaaaa", true),
            ("4-5 a: aaaaaa", false),
            ("4-5 a: bbbbb", false),
        ];

        for &(p, is_valid) in passwords {
            assert_eq!(PasswordEntry::from_str(p).unwrap().is_valid(), is_valid);
        }
    }

    #[test]
    fn test_is_valid_correct() {
        let passwords = &[
            ("1-3 a: abcde", true),
            ("1-3 b: cdefg", false),
            ("2-9 c: ccccccccc", false),
        ];

        for &(p, is_valid_correct) in passwords {
            assert_eq!(
                PasswordEntry::from_str(p).unwrap().is_valid_correct(),
                is_valid_correct
            );
        }
    }
}
