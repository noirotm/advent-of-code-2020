use crate::solver::{ReadExt, Solver};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Input;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, mut r: R) -> Self::Input {
        let s: Vec<String> = r.split_groups();
        Input {
            rules: rules_into_hashmap(s[0].as_bytes().split_lines()),
            messages: s[1].as_bytes().split_lines(),
        }
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input
            .messages
            .iter()
            .map(|s| string_matches_rule(s, 0, &input.rules))
            .filter(|&(s, ok)| ok && s.is_empty())
            .count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut rules = input.rules.clone();

        rules.insert(8, Rule::Alt((vec![42], vec![42, 8])));
        rules.insert(11, Rule::Alt((vec![42, 31], vec![42, 11, 31])));

        input
            .messages
            .iter()
            .map(|s| {
                let (out, ok) = string_matches_rule(s, 0, &rules);
                if ok && out.is_empty() {
                    dbg!(s, ok);
                }
                (out, ok)
            })
            .filter(|&(s, ok)| ok && s.is_empty())
            .count()
    }
}

pub struct Input {
    rules: HashMap<usize, Rule>,
    messages: Vec<String>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<String> = s.as_bytes().split_groups();

        Ok(Self {
            rules: rules_into_hashmap(s[0].as_bytes().split_lines()),
            messages: s[1].as_bytes().split_lines(),
        })
    }
}

type SeqRef = Vec<usize>;

#[derive(Clone)]
pub enum Rule {
    Term(char),
    Seq(SeqRef),
    Alt((SeqRef, SeqRef)),
}

pub struct RuleWithId {
    id: usize,
    rule: Rule,
}

impl FromStr for RuleWithId {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref ALT_RE: Regex = Regex::new(r"^(\d+): (\d+) (\d+) \| (\d+) (\d+)$").unwrap();
            static ref ALT2_RE: Regex = Regex::new(r"^(\d+): (\d+) \| (\d+)$").unwrap();
            static ref SEQREF_RE: Regex = Regex::new(r"^(\d+): (\d+) (\d+)$").unwrap();
            static ref ONE_RE: Regex = Regex::new(r"^(\d+): (\d+)$").unwrap();
            static ref TERM_RE: Regex = Regex::new(r#"^(\d+): "(.)"$"#).unwrap();
        }

        if let Some(c) = ALT_RE.captures(s) {
            Ok(RuleWithId {
                id: c[1].parse()?,
                rule: Rule::Alt((
                    vec![c[2].parse()?, c[3].parse()?],
                    vec![c[4].parse()?, c[5].parse()?],
                )),
            })
        } else if let Some(c) = ALT2_RE.captures(s) {
            Ok(RuleWithId {
                id: c[1].parse()?,
                rule: Rule::Alt((vec![c[2].parse()?], vec![c[3].parse()?])),
            })
        } else if let Some(c) = SEQREF_RE.captures(s) {
            Ok(RuleWithId {
                id: c[1].parse()?,
                rule: Rule::Seq(vec![c[2].parse()?, c[3].parse()?]),
            })
        } else if let Some(c) = ONE_RE.captures(s) {
            Ok(RuleWithId {
                id: c[1].parse()?,
                rule: Rule::Seq(vec![c[2].parse()?]),
            })
        } else if let Some(c) = TERM_RE.captures(s) {
            Ok(RuleWithId {
                id: c[1].parse()?,
                rule: Rule::Term(c[2].chars().next().ok_or("No match")?),
            })
        } else {
            dbg!(s);
            Err("No match".into())
        }
    }
}

fn rules_into_hashmap(v: Vec<RuleWithId>) -> HashMap<usize, Rule> {
    v.into_iter().map(|r| (r.id, r.rule)).collect()
}

fn string_matches_rule<'a>(s: &'a str, id: usize, rules: &HashMap<usize, Rule>) -> (&'a str, bool) {
    if s.is_empty() {
        return (s, false);
    }

    match rules.get(&id) {
        Some(Rule::Term(c)) => string_matches_char(s, *c),
        Some(Rule::Seq(seq)) => string_matches_seq(s, seq, rules),
        Some(Rule::Alt((r1, r2))) => string_matches_alt(s, r1, r2, rules),
        None => (s, false),
    }
}

fn string_matches_char(s: &str, c: char) -> (&str, bool) {
    if s.chars().next().unwrap_or_default() == c {
        (&s[1..], true)
    } else {
        (s, false)
    }
}

fn string_matches_seq<'a>(
    s: &'a str,
    r: &[usize],
    rules: &HashMap<usize, Rule>,
) -> (&'a str, bool) {
    let mut s = s;
    for id in r {
        let (ls, matches) = string_matches_rule(s, *id, rules);
        if !matches {
            return (s, false);
        }
        s = ls;
    }

    (s, true)
}

fn string_matches_alt<'a>(
    s: &'a str,
    r1: &[usize],
    r2: &[usize],
    rules: &HashMap<usize, Rule>,
) -> (&'a str, bool) {
    let (s1, matches1) = string_matches_seq(s, r1, rules);
    let (s2, matches2) = string_matches_seq(s, r2, rules);
    if matches1 && !matches2 {
        (s1, true)
    } else if !matches1 && matches2 {
        (s2, true)
    } else if matches1 && matches2 {
        if s1.len() < s2.len() {
            (s1, true)
        } else {
            (s2, true)
        }
    } else {
        (s, false)
    }
}
