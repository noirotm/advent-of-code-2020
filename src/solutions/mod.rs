// DO NOT EDIT THIS FILE
use crate::solver::Solver;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

pub fn exec_day(day: i32) {
    match day {
        1 => day01::Problem {}.solve(day),
        2 => day02::Problem {}.solve(day),
        3 => day03::Problem {}.solve(day),
        4 => day04::Problem {}.solve(day),
        5 => day05::Problem {}.solve(day),
        6 => day06::Problem {}.solve(day),
        7 => day07::Problem {}.solve(day),
        8 => day08::Problem {}.solve(day),
        d => println!("Day {} hasn't been solved yet :(", d),
    }
}
