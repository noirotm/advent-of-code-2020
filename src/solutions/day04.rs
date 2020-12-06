use crate::solver::Solver;
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Passport>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, mut r: R) -> Self::Input {
        Self::split_groups(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().filter(|p| p.is_valid()).count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input.iter().filter(|p| p.is_strictly_valid()).count()
    }
}

pub struct Passport {
    fields: HashMap<String, String>,
}

impl FromStr for Passport {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = RegexBuilder::new(r"(...):([^\s]+)")
                .multi_line(true)
                .build()
                .unwrap();
        }

        let values = RE
            .captures_iter(s)
            .map(|cap| (cap[1].to_string(), cap[2].to_string()))
            .collect::<HashMap<_, _>>();

        Ok(Passport { fields: values })
    }
}

impl Passport {
    fn is_valid(&self) -> bool {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|&s| self.fields.contains_key(s))
    }

    fn is_strictly_valid(&self) -> bool {
        self.validate().is_ok()
    }

    fn validate(&self) -> Result<(), Box<dyn Error>> {
        let byr = self
            .fields
            .get("byr")
            .ok_or("Missing byr")?
            .parse::<u32>()?;
        if byr < 1920 || byr > 2002 {
            return Err("Invalid byr".into());
        }

        let iyr = self
            .fields
            .get("iyr")
            .ok_or("Missing iyr")?
            .parse::<u32>()?;
        if iyr < 2010 || iyr > 2020 {
            return Err("Invalid iyr".into());
        }

        let eyr = self
            .fields
            .get("eyr")
            .ok_or("Missing eyr")?
            .parse::<u32>()?;
        if eyr < 2020 || eyr > 2030 {
            return Err("Invalid eyr".into());
        }

        lazy_static! {
            static ref HGT_RE: Regex = Regex::new(r"^(\d+)(..)$").unwrap();
        }
        let hgt = self.fields.get("hgt").ok_or("Missing hgt")?;
        let caps = HGT_RE.captures(hgt).ok_or("Invalid hgt")?;
        let hgt = caps[1].parse::<u32>()?;
        match &caps[2] {
            "cm" => {
                if hgt < 150 || hgt > 193 {
                    return Err("Invalid hgt".into());
                }
            }
            "in" => {
                if hgt < 59 || hgt > 76 {
                    return Err("Invalid hgt".into());
                }
            }
            _ => return Err("Invalid hgt unit".into()),
        }

        lazy_static! {
            static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        }
        let hcl = self.fields.get("hcl").ok_or("Missing hcl")?;
        if !HCL_RE.is_match(hcl) {
            return Err("Invalid hcl".into());
        }

        let ecl = self.fields.get("ecl").ok_or("Missing ecl")?;
        if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .any(|c| c == ecl)
        {
            return Err("Invalid ecl".into());
        }

        let pid_re = Regex::new(r"^\d{9}$")?;
        let pid = self.fields.get("pid").ok_or("Missing pid")?;
        if !pid_re.is_match(pid) {
            return Err("Invalid pid".into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passport() {
        let s = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm";
        let p = s.parse::<Passport>().unwrap();

        assert_eq!(p.fields.get("ecl"), Some(&String::from("gry")));
        assert_eq!(p.fields.get("cid"), Some(&String::from("147")));
    }

    #[test]
    fn test_passport_is_valid() {
        let passports = &[
            (
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm",
                true,
            ),
            (
                "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929",
                false,
            ),
            (
                "hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm",
                true,
            ),
            (
                "hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in",
                false,
            ),
        ];
        for &(p, is_valid) in passports {
            assert_eq!(Passport::from_str(p).unwrap().is_valid(), is_valid);
        }
    }

    #[test]
    fn test_passport_is_strictly_valid() {
        let passports = &[
            (
                "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
                true,
            ),
            (
                "eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
                true,
            ),
            (
                "hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022",
                true,
            ),
            (
                "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
                true,
            ),
            (
                "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
                false,
            ),
            (
                "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946",
                false,
            ),
            (
                "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
                false,
            ),
            (
                "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007",
                false,
            ),
        ];
        for &(p, is_valid) in passports {
            assert_eq!(Passport::from_str(p).unwrap().is_strictly_valid(), is_valid);
        }
    }
}
