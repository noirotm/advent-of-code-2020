use crate::solver::Solver;
use modinverse::modinverse;
use std::error::Error;
use std::io::{BufRead, BufReader, Read};
use std::iter::successors;

pub struct Problem;

impl Solver for Problem {
    type Input = Schedule;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        Schedule::from_reader(r).unwrap()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let (id, time) = input
            .ids
            .iter()
            .flatten()
            .map(|&id| {
                (
                    id,
                    successors(Some(0), |n| Some(n + id))
                        .find(|&n| n >= input.departure)
                        .unwrap_or_default(),
                )
            })
            .min_by_key(|&(_, time)| time)
            .unwrap_or_default();

        id * (time - input.departure)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        solve_second_optim(&input.ids)
    }
}

// don't run this, it will take ages
#[allow(dead_code)]
fn solve_second_brute(input: &Schedule) -> u64 {
    let constrained = input.ids.iter().flatten().count();

    let (t, _) = (0..u64::MAX)
        .into_iter()
        .map(|t| (t, check_timestamp(t, &input.ids, constrained)))
        .find(|&(_, ok)| ok)
        .unwrap_or_default();

    t
}

#[allow(dead_code)]
fn check_timestamp(t: u64, ids: &[Option<u64>], constrained: usize) -> bool {
    let mut ok = 0;
    for (i, &id) in ids.iter().enumerate() {
        let target = t + i as u64;
        if let Some(id) = id {
            let timestamp = successors(Some(0), |n| Some(n + id))
                .find(|&n| n >= target)
                .unwrap_or_default();
            if timestamp == target {
                ok += 1;
            } else {
                break;
            }
        }
    }

    ok == constrained
}

// this uses the chinese remainder theorem, O(n)
fn solve_second_optim(input: &[Option<u64>]) -> u64 {
    let v = input
        .iter()
        .enumerate()
        .filter(|&(_, id)| id.is_some())
        .map(|(i, id)| {
            let id = id.unwrap() as i64;
            (id - (i as i64), id)
        })
        .collect::<Vec<_>>();
    dbg!(&v);
    let t = chinese_remainder(&v);
    t.unwrap_or_default() as u64
}

fn chinese_remainder(equations: &[(i64, i64)]) -> Option<i64> {
    let prod = equations.iter().map(|&(_, m)| m).product::<i64>();
    let mut sum = 0;

    for &(residue, modulus) in equations.iter() {
        let p = prod / modulus;
        sum += residue * modinverse(p, modulus)? * p;
    }

    Some(sum % prod)
}

#[derive(Debug)]
pub struct Schedule {
    departure: u64,
    ids: Vec<Option<u64>>,
}

impl Schedule {
    fn from_reader<R: Read>(r: R) -> Result<Self, Box<dyn Error>> {
        let mut lines = BufReader::new(r).lines();
        let departure = lines.next().ok_or("Missing line")??.parse()?;
        let ids = lines
            .next()
            .ok_or("Missing line")??
            .split(',')
            .map(|id| id.parse().ok())
            .collect();

        Ok(Self { departure, ids })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chinese_remainder() {
        assert_eq!(chinese_remainder(&[(2, 3), (3, 5), (2, 7)]), Some(23));
        assert_eq!(chinese_remainder(&[(11, 10), (22, 4), (19, 9)]), None);
    }
}
