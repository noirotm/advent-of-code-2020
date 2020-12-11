use crate::grid::{Coord, Grid};
use crate::solver::Solver;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Grid<Seat>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        Grid::from_reader(r).unwrap()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut state = input.clone();
        let mut occupied = input.n_occupied();
        loop {
            state = state.next_gen();
            let cur_occ = state.n_occupied();
            if cur_occ != occupied {
                occupied = cur_occ;
            } else {
                break;
            }
        }

        occupied
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut state = input.clone();
        let mut occupied = input.n_occupied();
        loop {
            state = state.next_gen_v2();
            let cur_occ = state.n_occupied();
            if cur_occ != occupied {
                occupied = cur_occ;
            } else {
                break;
            }
        }

        occupied
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl Default for Seat {
    fn default() -> Self {
        Self::Floor
    }
}

impl TryFrom<u8> for Seat {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(Self::Floor),
            b'L' => Ok(Self::Empty),
            b'#' => Ok(Self::Occupied),
            _ => Err("Invalid value".into()),
        }
    }
}

impl Display for Seat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Seat::Floor => '.',
                Seat::Empty => 'L',
                Seat::Occupied => '#',
            }
        )
    }
}

impl Grid<Seat> {
    fn next_gen(&self) -> Self {
        let mut output = self.clone();
        for y in 0..self.h {
            for x in 0..self.w {
                if let Some(old_state) = self.get((x, y)) {
                    let new_state = match old_state {
                        Seat::Empty => {
                            if self
                                .neighbours8((x, y))
                                .iter()
                                .all(|&s| s != &Seat::Occupied)
                            {
                                Seat::Occupied
                            } else {
                                Seat::Empty
                            }
                        }
                        Seat::Occupied => {
                            if self
                                .neighbours8((x, y))
                                .iter()
                                .filter(|&&s| s == &Seat::Occupied)
                                .count()
                                >= 4
                            {
                                Seat::Empty
                            } else {
                                Seat::Occupied
                            }
                        }
                        Seat::Floor => Seat::Floor,
                    };
                    output.set((x, y), new_state);
                }
            }
        }

        output
    }

    fn next_gen_v2(&self) -> Self {
        let mut output = self.clone();
        for y in 0..self.h {
            for x in 0..self.w {
                if let Some(old_state) = self.get((x, y)) {
                    let new_state = match old_state {
                        Seat::Empty => {
                            if self
                                .actual_neighbours((x, y))
                                .iter()
                                .all(|&s| s != &Seat::Occupied)
                            {
                                Seat::Occupied
                            } else {
                                Seat::Empty
                            }
                        }
                        Seat::Occupied => {
                            if self
                                .actual_neighbours((x, y))
                                .iter()
                                .filter(|&&s| s == &Seat::Occupied)
                                .count()
                                >= 5
                            {
                                Seat::Empty
                            } else {
                                Seat::Occupied
                            }
                        }
                        Seat::Floor => Seat::Floor,
                    };
                    output.set((x, y), new_state);
                }
            }
        }

        output
    }

    fn n_occupied(&self) -> usize {
        let mut occupied = 0;
        for y in 0..self.h {
            for x in 0..self.w {
                if let Some(Seat::Occupied) = self.get((x, y)) {
                    occupied += 1;
                }
            }
        }

        occupied
    }

    fn actual_neighbours(&self, c: impl Coord) -> Vec<&Seat> {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .flat_map(|&(dx, dy)| self.first_neighbour(&c, dx, dy))
        .collect()
    }

    fn first_neighbour(&self, c: &impl Coord, dx: isize, dy: isize) -> Option<&Seat> {
        let mut c = c.coords();
        loop {
            if (c.x() == 0 && dx == -1) || (c.y() == 0 && dy == -1) {
                return None;
            }
            c = (
                ((c.x() as isize) + dx) as usize,
                ((c.y() as isize) + dy) as usize,
            );

            if let Some(seat) = self.get(c) {
                if seat == &Seat::Floor {
                    continue;
                } else {
                    return Some(seat);
                }
            } else {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_layout() {
        let grid: Grid<Seat> = Grid::from_str(
            r"#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
        )
        .unwrap();

        let count = grid
            .neighbours8((9, 0))
            .iter()
            .filter(|&&s| s == &Seat::Occupied)
            .inspect(|s| {
                dbg!(s);
            })
            .count();
        assert_eq!(count, 3);
    }
}
