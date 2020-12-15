use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::str::FromStr;

fn input_file(day: i32) -> String {
    format!("input/day{:02}", day)
}

pub trait Solver {
    type Input;
    type Output1: Display;
    type Output2: Display;

    fn parse_input<R: io::Seek + io::Read>(&self, r: R) -> Self::Input;
    fn solve_first(&self, input: &Self::Input) -> Self::Output1;
    fn solve_second(&self, input: &Self::Input) -> Self::Output2;

    fn load_input<P: AsRef<Path>>(&self, p: P) -> io::Result<Self::Input> {
        let f = File::open(p)?;
        Ok(self.parse_input(f))
    }

    fn solve(&self, day: i32) {
        let input_file = input_file(day);
        let input = self
            .load_input(input_file)
            .expect("unable to open input file");
        let s1 = self.solve_first(&input);
        let s2 = self.solve_second(&input);
        println!("Solution 1: {}", s1);
        println!("Solution 2: {}", s2);
    }
}

pub trait ReadExt<T> {
    fn split_commas(self) -> Vec<T>;
    fn split_lines(self) -> Vec<T>;
    fn split_groups(&mut self) -> Vec<T>;
}

impl<R, T> ReadExt<T> for R
where
    R: Read,
    T: FromStr,
{
    fn split_commas(self) -> Vec<T> {
        BufReader::new(self)
            .split(b',')
            .flatten()
            .flat_map(String::from_utf8)
            .flat_map(|s| s.parse())
            .collect()
    }

    fn split_lines(self) -> Vec<T> {
        BufReader::new(self)
            .lines()
            .flatten()
            .flat_map(|l| l.parse())
            .collect()
    }

    fn split_groups(&mut self) -> Vec<T> {
        let mut s = String::new();
        self.read_to_string(&mut s).unwrap();
        s.split("\r\n\r\n").flat_map(|s| s.parse()).collect()
    }
}
