use crate::solver::{ReadExt, Solver};
use std::collections::BTreeSet;
use std::io::Read;
use std::iter::FromIterator;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<usize>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let sorted = BTreeSet::from_iter(input.iter().cloned());

        let mut last = 0;
        let mut sum_3_diff = 0;
        let mut sum_1_diff = 0;

        for n in sorted {
            match n - last {
                1 => sum_1_diff += 1,
                3 => sum_3_diff += 1,
                _ => {}
            }
            last = n;
        }

        // count last one as 3
        sum_3_diff += 1;

        sum_1_diff * sum_3_diff
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        arrangements2(input)
    }
}

/// from https://gitlab.com/mboehnke/aoc-2020/-/blob/master/aoc-2020-10/src/solution.rs
/// I can't make sense of this =)
#[allow(dead_code)]
fn arrangements(input: &[usize]) -> usize {
    let mut sorted = input.to_vec();
    sorted.sort_unstable();

    let max = sorted.last().cloned().unwrap_or_default() + 3;
    sorted.push(max);

    let mut a = vec![0; max + 1];
    a[0] = 1;

    for n in sorted {
        a[n] = a[(n).saturating_sub(3)..n].iter().sum();
        dbg!(&a);
    }

    a.last().cloned().unwrap_or_default()
}

/// Only diffs < 3 count to add combinations
/// This means that we need to identify all sequences of diffs = 1
/// We then sum consecutive 1 diff sequences
/// and compute the number of combos for each
/// finally, multiply all combos
/// This relies on the assumption that there are only 1 & 3 diffs.
fn arrangements2(input: &[usize]) -> usize {
    let mut sorted = input.to_vec();
    sorted.push(0);
    sorted.sort_unstable();

    let diffs = sorted.windows(2).map(|a| a[1] - a[0]).collect::<Vec<_>>();
    diffs
        .split(|&num| num != 1)
        .filter(|s| !s.is_empty())
        .map(|s| s.iter().sum::<usize>())
        .map(combos)
        .product()
}

/// This is the nth term of the Tribonacci sequence
fn combos(n: usize) -> usize {
    1 + (n * (n - 1) / 2)
}
