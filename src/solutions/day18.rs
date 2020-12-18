use crate::solver::{ReadExt, Solver};
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<String>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().flat_map(|s| parser1::eval_expr(s)).sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input.iter().flat_map(|s| parser2::eval_expr(s)).sum()
    }
}

mod parser1 {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, digit1, multispace0};
    use nom::combinator::{all_consuming, map_res};
    use nom::multi::fold_many0;
    use nom::sequence::{delimited, pair, preceded, terminated};
    use nom::IResult;
    use std::str::FromStr;

    fn ws(i: &str) -> IResult<&str, &str> {
        multispace0(i)
    }

    fn parens_expr(i: &str) -> IResult<&str, u64> {
        let lparens = preceded(ws, tag("("));
        let rparens = preceded(ws, tag(")"));
        delimited(lparens, expr, rparens)(i)
    }

    fn term(i: &str) -> IResult<&str, u64> {
        let number = map_res(preceded(ws, digit1), FromStr::from_str);
        alt((number, parens_expr))(i)
    }

    fn expr(i: &str) -> IResult<&str, u64> {
        let (i, first) = term(i)?;

        let add = preceded(ws, char('+'));
        let mul = preceded(ws, char('*'));
        let op = alt((mul, add));
        fold_many0(pair(op, term), first, |acc, (op, val)| {
            if op == '+' {
                acc + val
            } else {
                acc * val
            }
        })(i)
    }

    fn complete_expr(s: &str) -> IResult<&str, u64> {
        all_consuming(terminated(expr, ws))(s)
    }

    pub fn eval_expr(s: &str) -> Option<u64> {
        complete_expr(s).ok().map(|(_, o)| o)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse() {
            assert_eq!(complete_expr("5"), Ok(("", 5)));
            assert_eq!(complete_expr("1 + 1"), Ok(("", 2)));
            assert_eq!(complete_expr("(1 + 1)"), Ok(("", 2)));
            assert_eq!(complete_expr("1 + 1 * 2"), Ok(("", 4)));
            assert_eq!(complete_expr("1 + (1 * 2)"), Ok(("", 3)));
        }
    }
}

mod parser2 {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, digit1, multispace0};
    use nom::combinator::{all_consuming, map_res};
    use nom::multi::fold_many0;
    use nom::sequence::{delimited, pair, preceded, terminated};
    use nom::IResult;
    use std::str::FromStr;

    fn ws(i: &str) -> IResult<&str, &str> {
        multispace0(i)
    }

    fn parens_expr(i: &str) -> IResult<&str, u64> {
        let lparens = preceded(ws, tag("("));
        let rparens = preceded(ws, tag(")"));
        delimited(lparens, expr, rparens)(i)
    }

    fn term(i: &str) -> IResult<&str, u64> {
        let number = map_res(preceded(ws, digit1), FromStr::from_str);

        alt((number, parens_expr))(i)
    }

    fn factor(i: &str) -> IResult<&str, u64> {
        let (i, first) = term(i)?;

        let add = preceded(ws, char('+'));

        fold_many0(pair(add, term), first, |acc, (_, val)| acc + val)(i)
    }

    fn expr(i: &str) -> IResult<&str, u64> {
        let (i, first) = factor(i)?;

        let mul = preceded(ws, char('*'));

        fold_many0(pair(mul, factor), first, |acc, (_, val)| acc * val)(i)
    }

    fn complete_expr(s: &str) -> IResult<&str, u64> {
        all_consuming(terminated(expr, ws))(s)
    }

    pub fn eval_expr(s: &str) -> Option<u64> {
        complete_expr(s).ok().map(|(_, o)| o)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse() {
            assert_eq!(complete_expr("5"), Ok(("", 5)));
            assert_eq!(complete_expr("1 + 1"), Ok(("", 2)));
            assert_eq!(complete_expr("(1 + 1)"), Ok(("", 2)));
            assert_eq!(complete_expr("1 + 1 * 2"), Ok(("", 4)));
            assert_eq!(complete_expr("1 + (1 * 2)"), Ok(("", 3)));
            assert_eq!(complete_expr("2 * 3 + 1"), Ok(("", 8)));
        }
    }
}
