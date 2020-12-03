use crate::grid::{Grid, GridPoint};
use crate::solver::Solver;
use std::convert::TryFrom;
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Grid<Slope>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        Grid::from_reader(r).unwrap()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        check_slope(input, 3, 1)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        check_slope(input, 1, 1)
            * check_slope(input, 3, 1)
            * check_slope(input, 5, 1)
            * check_slope(input, 7, 1)
            * check_slope(input, 1, 2)
    }
}

fn check_slope(input: &Grid<Slope>, dx: usize, dy: usize) -> usize {
    let mut tree_counter = 0;
    let mut pt = GridPoint { x: 0, y: 0 };

    while pt.y < input.h {
        if pt.x >= input.w {
            pt.x -= input.w;
        }

        if let Some(&Slope::Tree) = input.get(&pt) {
            tree_counter += 1;
        }

        pt.x += dx;
        pt.y += dy;
    }

    tree_counter
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Slope {
    Empty,
    Tree,
}

impl TryFrom<u8> for Slope {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(Self::Empty),
            b'#' => Ok(Self::Tree),
            v => Err(format!("Invalid cell: {}", v)),
        }
    }
}

impl Default for Slope {
    fn default() -> Self {
        Self::Empty
    }
}
