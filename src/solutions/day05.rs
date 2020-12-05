use crate::solver::Solver;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<BoardingPass>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .map(|s| BoardingPass::new(&s))
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().map(|bp| bp.id()).max().unwrap_or_default()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let booked_seats = input.iter().map(|bp| bp.id()).collect::<HashSet<_>>();
        let max_id = 127 * 8 + 7;
        (1..max_id)
            .find(|id| {
                !booked_seats.contains(&id)
                    && booked_seats.contains(&(id - 1))
                    && booked_seats.contains(&(id + 1))
            })
            .unwrap_or_default()
    }
}

pub struct BoardingPass {
    code: Vec<u8>,
}

#[derive(Eq, Hash, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

impl BoardingPass {
    fn new(s: &str) -> Self {
        Self {
            code: Vec::from(s.as_bytes()),
        }
    }

    fn position(&self) -> Position {
        let row = &self.code[0..7];
        let mut row_range = 0..128;
        for &b in row {
            let middle = (row_range.end + row_range.start) / 2;
            if b == b'F' {
                row_range = row_range.start..middle;
            } else {
                row_range = middle..row_range.end;
            }
        }
        let row = row_range.start;

        let col = &self.code[7..10];
        let mut col_range = 0..8;
        for &b in col {
            let middle = (col_range.end + col_range.start) / 2;
            if b == b'L' {
                col_range = col_range.start..middle;
            } else {
                col_range = middle..col_range.end;
            }
        }
        let col = col_range.start;

        Position { row, col }
    }

    fn id(&self) -> usize {
        let p = self.position();
        p.row * 8 + p.col
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boarding_pass_position() {
        let t = &[
            ("BFFFBBFRRR", Position { row: 70, col: 7 }),
            ("FFFFFFFLLL", Position { row: 0, col: 0 }),
            ("BBBBBBBRRR", Position { row: 127, col: 7 }),
        ];

        for (s, pos) in t {
            let p = BoardingPass::new(s).position();
            assert_eq!(p.row, pos.row);
            assert_eq!(p.col, pos.col);
        }
    }
}
