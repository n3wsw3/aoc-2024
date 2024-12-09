use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::{char, digit1}, multi::separated_list1, IResult};

advent_of_code::solution!(7);

#[derive(Debug)]
struct Equation {
    total: u64,
    nums: Vec<u64>,
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
    Concat
}

impl Equation {
    fn is_valid(&self, operators: &Vec<&Operator>) -> bool {
        let mut evaluated_total = self.nums[0];

        for idx in 1..self.nums.len() {
            let num = self.nums[idx];
            let operator = operators.get(idx - 1).unwrap();

            match operator {
                Operator::Add => evaluated_total += num,
                Operator::Multiply => evaluated_total *= num,
                Operator::Concat => {
                    evaluated_total = evaluated_total * 10u64.pow(num.ilog10() + 1) + num;
                }
            }
        }

        evaluated_total == self.total
    }
}

fn parse_line(input: &str) -> IResult<&str, Equation> {
    let (input, total) =digit1(input)?;
    let (input, _) = tag(": ")(input)?;

    let (input, nums) = separated_list1(char(' '), digit1)(input)?;
    let (input, _) = tag("\n")(input)?;

    Ok((input, Equation { total: total.parse().unwrap(), nums: nums.iter().map(|n| n.parse().unwrap()).collect() }))
}

fn parse(mut input: &str) -> Vec<Equation> {
    let mut equations = vec![];

    while let Ok((new_input, eq)) = parse_line(input) {
        equations.push(eq);
        input = new_input;
    }

    equations
}


fn solve(input: &str, operators: &[Operator]) -> Option<u64> {
    let equations = parse(input);

    equations.iter().map(|eq| {
        (0..(eq.nums.len() - 1)).map(|_| operators.iter()).multi_cartesian_product().map(|operators| {
            if eq.is_valid(&operators) {
                return Some(eq.total)
            }
            None
        }).filter(Option::is_some).next()
    }).filter(Option::is_some).map(Option::unwrap).sum()
}

const OPERATORS_P1: &[Operator] = &[Operator::Add, Operator::Multiply];
pub fn part_one(input: &str) -> Option<u64> {
    solve(input, OPERATORS_P1)
}

const OPERATORS_P2: &[Operator] = &[Operator::Add, Operator::Multiply, Operator::Concat];
pub fn part_two(input: &str) -> Option<u64> {
    solve(input, OPERATORS_P2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
