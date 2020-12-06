use crate::solver::{ReadExt, Solver};
use std::collections::BTreeSet;
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Group>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, mut r: R) -> Self::Input {
        r.split_groups()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().map(|g| g.count_answers()).sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input.iter().map(|g| g.count_ensemble_answers()).sum()
    }
}

pub struct Group {
    answers: Vec<String>,
}

impl FromStr for Group {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            answers: s.lines().map(|s| s.to_string()).collect(),
        })
    }
}

impl Group {
    fn count_answers(&self) -> usize {
        self.answers
            .iter()
            .map(|s| s.bytes())
            .flatten()
            .collect::<BTreeSet<_>>()
            .len()
    }

    fn count_ensemble_answers(&self) -> usize {
        let first = self.answers[0].bytes().collect::<BTreeSet<_>>();
        self.answers[1..]
            .iter()
            .map(|s| s.bytes().collect::<BTreeSet<_>>())
            .fold(first, |a, b| a.intersection(&b).cloned().collect())
            .len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_answers() {
        let g = Group::from_str("abcx\nabcy\nabcz").unwrap();
        assert_eq!(g.count_answers(), 6);
    }

    #[test]
    fn test_count_ensemble_answers() {
        let g = Group::from_str("abc").unwrap();
        assert_eq!(g.count_ensemble_answers(), 3);

        let g = Group::from_str("a\nb\nc").unwrap();
        assert_eq!(g.count_ensemble_answers(), 0);

        let g = Group::from_str("ab\nac").unwrap();
        assert_eq!(g.count_ensemble_answers(), 1);

        let g = Group::from_str("a\na\na\na").unwrap();
        assert_eq!(g.count_ensemble_answers(), 1);

        let g = Group::from_str("b").unwrap();
        assert_eq!(g.count_ensemble_answers(), 1);
    }
}
