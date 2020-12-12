use crate::solver::{ReadExt, Solver};
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Instruction>;
    type Output1 = i32;
    type Output2 = i32;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let c = eval_instructions(input);
        c.x.abs() + c.y.abs()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let c = eval_instructions_with_waypoint(input);
        c.x.abs() + c.y.abs()
    }
}

pub enum Instruction {
    MoveNorth(i32),
    MoveSouth(i32),
    MoveEast(i32),
    MoveWest(i32),
    TurnLeft(i32),
    TurnRight(i32),
    MoveForward(i32),
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(.)(\d+)$").unwrap();
        }

        let c = RE.captures(s).ok_or("No match")?;
        let v = c[2].parse()?;

        match &c[1] {
            "N" => Ok(Self::MoveNorth(v)),
            "S" => Ok(Self::MoveSouth(v)),
            "E" => Ok(Self::MoveEast(v)),
            "W" => Ok(Self::MoveWest(v)),
            "L" => Ok(Self::TurnLeft(v)),
            "R" => Ok(Self::TurnRight(v)),
            "F" => Ok(Self::MoveForward(v)),
            _ => Err("Invalid direction".into()),
        }
    }
}

fn eval_instructions(instructions: &[Instruction]) -> Point {
    let mut pos = Point { x: 0, y: 0 };
    let mut dir = Dir::E;

    for i in instructions {
        match i {
            Instruction::MoveNorth(m) => {
                pos = pos.offset(&(0, -*m));
            }
            Instruction::MoveSouth(m) => {
                pos = pos.offset(&(0, *m));
            }
            Instruction::MoveEast(m) => {
                pos = pos.offset(&(*m, 0));
            }
            Instruction::MoveWest(m) => {
                pos = pos.offset(&(-*m, 0));
            }
            Instruction::TurnLeft(a) => {
                dir = turn_left(*a, &dir);
            }
            Instruction::TurnRight(a) => {
                dir = turn_right(*a, &dir);
            }
            Instruction::MoveForward(m) => {
                pos = pos.offset(&match dir {
                    Dir::N => (0, -*m),
                    Dir::S => (0, *m),
                    Dir::E => (*m, 0),
                    Dir::W => (-*m, 0),
                });
            }
        }
    }

    pos
}

fn turn_left(angle: i32, dir: &Dir) -> Dir {
    match (angle, dir) {
        (90, &Dir::N) => Dir::W,
        (90, &Dir::S) => Dir::E,
        (90, &Dir::E) => Dir::N,
        (90, &Dir::W) => Dir::S,
        (180, d) => reverse(d),
        (270, d) => turn_right(90, d),
        _ => panic!("Shouldn't happen"),
    }
}

fn turn_right(angle: i32, dir: &Dir) -> Dir {
    match (angle, dir) {
        (90, &Dir::N) => Dir::E,
        (90, &Dir::S) => Dir::W,
        (90, &Dir::E) => Dir::S,
        (90, &Dir::W) => Dir::N,
        (180, d) => reverse(d),
        (270, d) => turn_left(90, d),
        _ => panic!("Shouldn't happen"),
    }
}

fn reverse(dir: &Dir) -> Dir {
    match dir {
        Dir::N => Dir::S,
        Dir::S => Dir::N,
        Dir::E => Dir::W,
        Dir::W => Dir::E,
    }
}

fn eval_instructions_with_waypoint(instructions: &[Instruction]) -> Point {
    let mut ship_pos = Point { x: 0, y: 0 };
    let mut waypoint_pos = Point { x: 10, y: -1 };

    for i in instructions {
        match i {
            Instruction::MoveNorth(m) => {
                waypoint_pos = waypoint_pos.offset(&(0, -*m));
            }
            Instruction::MoveSouth(m) => {
                waypoint_pos = waypoint_pos.offset(&(0, *m));
            }
            Instruction::MoveEast(m) => {
                waypoint_pos = waypoint_pos.offset(&(*m, 0));
            }
            Instruction::MoveWest(m) => {
                waypoint_pos = waypoint_pos.offset(&(-*m, 0));
            }
            Instruction::TurnLeft(a) => {
                waypoint_pos = rotate_waypoint_left(*a, &waypoint_pos, &ship_pos);
            }
            Instruction::TurnRight(a) => {
                waypoint_pos = rotate_waypoint_right(*a, &waypoint_pos, &ship_pos);
            }
            Instruction::MoveForward(m) => {
                let offset = (waypoint_pos.x - ship_pos.x, waypoint_pos.y - ship_pos.y);
                for _ in 0..*m {
                    ship_pos = ship_pos.offset(&offset);
                    waypoint_pos = waypoint_pos.offset(&offset);
                }
            }
        }
    }

    ship_pos
}

fn rotate_waypoint_left(angle: i32, pos: &Point, to: &Point) -> Point {
    rotate_waypoint_right(360 - angle, pos, to)
}

fn rotate_waypoint_right(angle: i32, pt: &Point, orig: &Point) -> Point {
    let (sin, cos) = fast_sin_cos(angle);

    let x1 = pt.x - orig.x;
    let y1 = pt.y - orig.y;

    let x2 = cos * x1 - sin * y1;
    let y2 = sin * x1 + cos * y1;

    Point {
        x: orig.x + x2,
        y: orig.y + y2,
    }
}

fn fast_sin_cos(angle: i32) -> (i32, i32) {
    match angle {
        90 => (1, 0),
        180 => (0, -1),
        270 => (-1, 0),
        _ => panic!("unexpected angle"),
    }
}

enum Dir {
    N,
    S,
    E,
    W,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn offset(self, (dx, dy): &(i32, i32)) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let Point { x, y } =
            rotate_waypoint_right(90, &Point { x: 10, y: 0 }, &Point { x: 0, y: 0 });
        assert_eq!(x, 0);
        assert_eq!(y, 10);

        let Point { x, y } =
            rotate_waypoint_left(90, &Point { x: 10, y: 0 }, &Point { x: 0, y: 0 });
        assert_eq!(x, 0);
        assert_eq!(y, -10);

        let Point { x, y } =
            rotate_waypoint_right(180, &Point { x: 10, y: 0 }, &Point { x: 0, y: 0 });
        assert_eq!(x, -10);
        assert_eq!(y, 0);
    }
}
