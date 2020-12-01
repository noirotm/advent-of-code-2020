use crate::solver::Solver;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<u32>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        let r = BufReader::new(r);
        r.lines().flatten().flat_map(|l| l.parse()).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut needed = HashSet::new();

        for n in input {
            let missing = 2020 - n;
            if needed.contains(n) {
                return missing * n;
            } else {
                needed.insert(missing);
            }
        }

        0
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut needed = HashMap::new();

        for (i, n) in input.iter().enumerate() {
            if let Some((a, b)) = needed.get(n) {
                return n * a * b;
            }

            for &v in input[0..i].iter() {
                if v + n < 2020 {
                    needed.insert(2020 - (v + n), (v, *n));
                }
            }
        }

        0
    }
}
