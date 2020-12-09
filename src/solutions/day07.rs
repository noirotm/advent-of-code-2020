use crate::solver::{ReadExt, Solver};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Rules;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        rules_map(&r.split_lines())
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .filter(|&(e, _)| rule_contains_target(input, e))
            .count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        bag_contents(input, "shiny gold") - 1
    }
}

pub type Rules = HashMap<String, HashMap<String, usize>>;

struct Rule {
    bag: String,
    contents: HashMap<String, usize>,
}

impl FromStr for Rule {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref START_RE: Regex = Regex::new(r"^(\w+\s\w+) bags").unwrap();
            static ref RE: Regex = Regex::new(r"(\d) (\w+\s\w+) bag").unwrap();
        }

        let caps = START_RE.captures(s).ok_or("no match")?;
        let bag = caps[1].to_string();

        let contents = RE
            .captures_iter(s)
            .map(|cap| (cap[2].to_string(), cap[1].parse().unwrap_or_default()))
            .collect();

        Ok(Self { bag, contents })
    }
}

fn rules_map(rules: &[Rule]) -> Rules {
    rules
        .iter()
        .map(|e| (e.bag.to_string(), e.contents.to_owned()))
        .collect::<HashMap<_, _>>()
}

fn rule_contains_target(rules: &Rules, bag: &str) -> bool {
    if let Some(rule) = rules.get(bag) {
        for (bag, _) in rule.iter() {
            if bag.eq("shiny gold") || rule_contains_target(rules, &bag) {
                return true;
            }
        }
    }

    false
}

fn bag_contents(rules: &Rules, bag: &str) -> usize {
    let mut total = 1;

    if let Some(rule) = rules.get(bag) {
        for (bag, n) in rule.iter() {
            total += n * bag_contents(rules, bag);
        }
    }

    total
}
