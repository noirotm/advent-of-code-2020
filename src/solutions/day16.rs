use crate::solver::{ReadExt, Solver};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeSet, HashMap};
use std::error::Error;
use std::io::Read;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Spec;
    type Output1 = u32;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        Spec::from_reader(r).unwrap()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input
            .nearby_tickets
            .iter()
            .flat_map(|t| t.invalid_field(&input.rules))
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let valid_tickets = input
            .nearby_tickets
            .iter()
            .filter(|t| t.is_valid(&input.rules));

        // build a set of possible positions for each rule
        // at start, all positions are possible
        let pos_set = (0..input.rules.len()).collect::<BTreeSet<_>>();
        let mut possible_positions = input
            .rules
            .iter()
            .map(|r| (r.name.as_str(), pos_set.clone()))
            .collect::<HashMap<_, _>>();

        // iter tickets to find impossibilities
        // for each impossibility, update each entry in the possible positions set
        // by removing the impossible position
        for ticket in valid_tickets.into_iter().cycle() {
            for (i, &val) in ticket.0.iter().enumerate() {
                for rule in input.rules.iter() {
                    if !rule.is_value_valid(val) {
                        let number_to_remove =
                            if let Some(set) = possible_positions.get_mut(&rule.name.as_str()) {
                                set.remove(&i);
                                // if a set has only one entry, now we need to update all of the others
                                // by deleting the remaining entry from them
                                if set.len() == 1 {
                                    set.iter().next().cloned()
                                } else {
                                    None
                                }
                            } else {
                                None
                            };

                        if let Some(number_to_remove) = number_to_remove {
                            for rule_name in input
                                .rules
                                .iter()
                                .map(|r| r.name.as_str())
                                .filter(|&s| s != rule.name.as_str())
                            {
                                if let Some(set) = possible_positions.get_mut(rule_name) {
                                    set.remove(&number_to_remove);
                                }
                            }
                        }
                    }
                }
            }

            if possible_positions.iter().all(|(_, e)| e.len() == 1) {
                break;
            }
        }

        possible_positions
            .iter()
            .filter(|(&rule, _)| rule.starts_with("departure"))
            .map(|(&rule, values)| (rule, values.iter().next().unwrap()))
            .map(|(_, &idx)| input.my_ticket.0[idx] as u64)
            .product()
    }
}

pub struct Spec {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Spec {
    fn from_reader<R: Read>(mut r: R) -> Result<Self, Box<dyn Error>> {
        let groups: Vec<String> = r.split_groups();
        Ok(Self {
            rules: Self::parse_rules(&groups[0])?,
            my_ticket: Self::parse_my_ticket(&groups[1])?,
            nearby_tickets: Self::parse_nearby_tickets(&groups[2]),
        })
    }

    fn parse_rules(s: &str) -> Result<Vec<Rule>, Box<dyn Error>> {
        Ok(s.lines().flat_map(Rule::from_str).collect())
    }

    fn parse_my_ticket(s: &str) -> Result<Ticket, Box<dyn Error>> {
        s.lines().nth(1).map(Ticket::from_str).ok_or("No match")?
    }

    fn parse_nearby_tickets(s: &str) -> Vec<Ticket> {
        s.lines().skip(1).flat_map(Ticket::from_str).collect()
    }
}

pub struct Rule {
    name: String,
    range1: RangeInclusive<u32>,
    range2: RangeInclusive<u32>,
}

impl FromStr for Rule {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        }

        let c = RE.captures(s).ok_or("No match")?;
        Ok(Self {
            name: c[1].to_string(),
            range1: (c[2].parse()?)..=(c[3].parse()?),
            range2: (c[4].parse()?)..=(c[5].parse()?),
        })
    }
}

impl Rule {
    fn is_value_valid(&self, val: u32) -> bool {
        self.range1.contains(&val) || self.range2.contains(&val)
    }
}

pub struct Ticket(Vec<u32>);

impl FromStr for Ticket {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ticket(s.as_bytes().split_commas()))
    }
}

impl Ticket {
    fn is_valid(&self, rules: &[Rule]) -> bool {
        self.0
            .iter()
            .all(|&v| rules.iter().any(|r| r.is_value_valid(v)))
    }

    fn invalid_field(&self, rules: &[Rule]) -> Option<u32> {
        self.0
            .iter()
            .find(|&&v| rules.iter().all(|r| !r.is_value_valid(v)))
            .cloned()
    }
}
