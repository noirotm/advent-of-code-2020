use crate::grid::Grid;
use crate::solver::Solver;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Grid<Cube>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        Grid::from_reader(r).unwrap()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut gen = part1::grid_to_set(input);

        for _ in 0..6 {
            gen = part1::next_gen(&gen);
        }

        gen.len()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut gen = part2::grid_to_set(input);

        for _ in 0..6 {
            gen = part2::next_gen(&gen);
        }

        gen.len()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Cube {
    Inactive,
    Active,
}

impl TryFrom<u8> for Cube {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(Self::Inactive),
            b'#' => Ok(Self::Active),
            v => Err(format!("Invalid cell: {}", v)),
        }
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self::Inactive
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Inactive => '.',
                Self::Active => '#',
            }
        )
    }
}

mod part1 {
    use crate::grid::Grid;
    use crate::solutions::day17::Cube;
    use itertools::Itertools;
    use std::collections::HashSet;

    type Point = (isize, isize, isize);

    pub fn grid_to_set(cubes: &Grid<Cube>) -> HashSet<Point> {
        (0..cubes.w)
            .cartesian_product(0..cubes.h)
            .filter(|&(x, y)| cubes.get((x, y)) == Some(&Cube::Active))
            .map(|(x, y)| (x as isize, y as isize, 0))
            .collect()
    }

    pub fn next_gen(cubes: &HashSet<Point>) -> HashSet<Point> {
        let mut next = HashSet::new();
        let ((minx, miny, minz), (maxx, maxy, maxz)) = min_max_pt(cubes);

        let coords = (minx - 1..=maxx + 1)
            .cartesian_product(miny - 1..=maxy + 1)
            .cartesian_product(minz - 1..=maxz + 1);

        for ((x, y), z) in coords {
            let pt = (x, y, z);
            let neighbours = neighbours(&pt);
            let neighbours_alive = n_alive(cubes, &neighbours);
            let alive = cubes.contains(&pt);
            let insert = match (alive, neighbours_alive) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
            if insert {
                next.insert(pt);
            }
        }

        next
    }

    fn min_max_pt(cubes: &HashSet<Point>) -> (Point, Point) {
        let (mut minx, mut miny, mut minz) = (0, 0, 0);
        let (mut maxx, mut maxy, mut maxz) = (0, 0, 0);

        for &(x, y, z) in cubes {
            if x < minx {
                minx = x;
            }
            if x > maxx {
                maxx = x;
            }

            if y < miny {
                miny = y;
            }
            if y > maxy {
                maxy = y;
            }

            if z < minz {
                minz = z;
            }
            if z > maxz {
                maxz = z;
            }
        }

        ((minx, miny, minz), (maxx, maxy, maxz))
    }

    fn neighbours(pt: &Point) -> Vec<Point> {
        let (x, y, z) = pt;
        (-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .map(|((xn, yn), zn)| (xn + x, yn + y, zn + z))
            .filter(|p| p != pt)
            .collect()
    }

    fn n_alive(cubes: &HashSet<Point>, points: &[Point]) -> usize {
        points.iter().filter(|&c| cubes.contains(c)).count()
    }
}

mod part2 {
    use crate::grid::Grid;
    use crate::solutions::day17::Cube;
    use itertools::Itertools;
    use std::collections::HashSet;

    type Point = (isize, isize, isize, isize);

    pub fn grid_to_set(cubes: &Grid<Cube>) -> HashSet<Point> {
        (0..cubes.w)
            .cartesian_product(0..cubes.h)
            .filter(|&(x, y)| cubes.get((x, y)) == Some(&Cube::Active))
            .map(|(x, y)| (x as isize, y as isize, 0, 0))
            .collect()
    }

    pub fn next_gen(cubes: &HashSet<Point>) -> HashSet<Point> {
        let mut next = HashSet::new();
        let ((minx, miny, minz, minw), (maxx, maxy, maxz, maxw)) = min_max_pt(cubes);

        let coords = (minx - 1..=maxx + 1)
            .cartesian_product(miny - 1..=maxy + 1)
            .cartesian_product(minz - 1..=maxz + 1)
            .cartesian_product(minw - 1..=maxw + 1);

        for (((x, y), z), w) in coords {
            let pt = (x, y, z, w);
            let neighbours = neighbours(&pt);
            let neighbours_alive = n_alive(cubes, &neighbours);
            let alive = cubes.contains(&pt);
            let insert = match (alive, neighbours_alive) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
            if insert {
                next.insert(pt);
            }
        }

        next
    }

    fn min_max_pt(cubes: &HashSet<Point>) -> (Point, Point) {
        let (mut minx, mut miny, mut minz, mut minw) = (0, 0, 0, 0);
        let (mut maxx, mut maxy, mut maxz, mut maxw) = (0, 0, 0, 0);

        for &(x, y, z, w) in cubes {
            if x < minx {
                minx = x;
            }
            if x > maxx {
                maxx = x;
            }

            if y < miny {
                miny = y;
            }
            if y > maxy {
                maxy = y;
            }

            if z < minz {
                minz = z;
            }
            if z > maxz {
                maxz = z;
            }

            if w < minw {
                minw = w;
            }
            if w > maxw {
                maxw = w;
            }
        }

        ((minx, miny, minz, minw), (maxx, maxy, maxz, maxw))
    }

    fn neighbours(pt: &Point) -> Vec<Point> {
        let (x, y, z, w) = pt;
        (-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .map(|(((xn, yn), zn), wn)| (xn + x, yn + y, zn + z, wn + w))
            .filter(|p| p != pt)
            .collect()
    }

    fn n_alive(cubes: &HashSet<Point>, points: &[Point]) -> usize {
        points.iter().filter(|&c| cubes.contains(c)).count()
    }
}
