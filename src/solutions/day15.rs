use crate::solver::{ReadExt, Solver};
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<u64>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_commas()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        run_game(input, 2020)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        run_game(input, 30000000)
    }
}

fn run_game(starting_numbers: &[u64], iterations: usize) -> usize {
    let mut last_seen = vec![None; iterations];

    // starting numbers
    for (i, &n) in starting_numbers.iter().enumerate() {
        last_seen[n as usize] = Some(i);
    }

    // next
    let mut previous = 0;
    for i in starting_numbers.len()..(iterations - 1) {
        let pos = last_seen[previous];
        last_seen[previous] = Some(i);
        previous = pos.map(|p| i - p).unwrap_or(0);
    }

    previous
}
