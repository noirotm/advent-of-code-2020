use crate::solver::{ReadExt, Solver};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::io::Read;
use std::iter::FromIterator;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<u64>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut range = (0, 25);
        let mut preamble = BTreeSet::from_iter(input[range.0..range.1].iter().cloned());

        while let Some(&n) = input.get(range.1) {
            if !is_n_sum_of_preamble(n, &preamble) {
                return n;
            }

            preamble.remove(&input[range.0]);
            preamble.insert(input[range.1]);

            range = (range.0 + 1, range.1 + 1);
        }

        0
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let sum = 69316178; //self.solve_first(input);

        (0..input.len())
            .find_map(|i| range_sum_bounds(&input[i..], sum))
            .map(|(a, b)| a + b)
            .unwrap_or_default()
    }
}

fn is_n_sum_of_preamble(n: u64, preamble: &BTreeSet<u64>) -> bool {
    preamble
        .iter()
        .flat_map(|&a| n.checked_sub(a))
        .any(|b| preamble.contains(&b))
}

fn range_sum_bounds(range: &[u64], target_sum: u64) -> Option<(u64, u64)> {
    let mut sum = *range.get(0)?;
    let mut min = sum;
    let mut max = sum;

    for &n in &range[1..] {
        if n < min {
            min = n;
        }
        if n > max {
            max = n;
        }

        sum += n;
        match sum.cmp(&target_sum) {
            Ordering::Equal => return Some((min, max)),
            Ordering::Greater => return None,
            _ => {}
        }
    }

    None
}
