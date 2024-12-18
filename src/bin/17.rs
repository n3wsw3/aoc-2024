use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, one_of},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

use std::str::FromStr;

advent_of_code::solution!(17);

fn combo_value(operand: u64, a: u64, b: u64, c: u64) -> u64 {
    match operand {
        0..4 => operand,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Invalid operand"),
    }
}

fn run(program: &Vec<u64>, a: u64, b: u64, c: u64) -> Vec<u64> {
    let mut pc = 0;
    let mut output = Vec::new();
    let (mut a, mut b, mut c) = (a, b, c);

    while pc < program.len() {
        let instruction = program[pc];
        let operand = program[pc + 1];

        pc += 2;

        match instruction {
            0 => {
                a = a >> combo_value(operand, a, b, c);
            }
            1 => {
                b = b ^ operand;
            }
            2 => {
                b = combo_value(operand, a, b, c) & 0b111;
            }
            3 => {
                if a != 0 {
                    pc = operand as usize;
                }
            }
            4 => {
                b = b ^ c;
            }
            5 => {
                output.push(combo_value(operand, a, b, c) & 0b111);
            }
            6 => {
                b = a >> combo_value(operand, a, b, c);
            }
            7 => {
                c = a >> combo_value(operand, a, b, c);
            }
            _ => panic!("Invalid instruction"),
        }
    }

    output
}

fn parse_register(input: &str) -> IResult<&str, u64> {
    let (input, _) = tag("Register ")(input)?;
    let (input, _) = one_of("ABC")(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, value) = digit1(input)?;
    let (input, _) = tag("\n")(input)?;

    Ok((input, value.parse().unwrap()))
}

fn parse_program(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tag("Program: ")(input)?;

    let (input, program) = separated_list1(tag(","), map_res(digit1, u64::from_str))(input)?;

    Ok((input, program))
}

fn parse(input: &str) -> IResult<&str, (Vec<u64>, u64, u64, u64)> {
    let (input, a) = parse_register(input)?;
    let (input, b) = parse_register(input)?;
    let (input, c) = parse_register(input)?;

    let (input, _) = tag("\n")(input)?;

    let (input, program) = parse_program(input)?;

    Ok((input, (program, a, b, c)))
}

pub fn part_one(input: &str) -> Option<String> {
    let (program, a, b, c) = parse(input).ok()?.1;
    let output = run(&program, a, b, c);

    Some(output.iter().map(ToString::to_string).join(","))
}

fn find_next(program: &Vec<u64>, expect: Vec<u64>, num: u64) -> Option<u64> {
    let possible_nexts = (0..8)
        .filter_map(|a| {
            let a = num << 3 | a;
            let res = run(&program, a, 0, 0);

            if res == expect {
                Some(a)
            } else {
                None
            }
        })
        .collect_vec();

    if expect.len() == program.len() {
        return possible_nexts
            .iter()
            .filter_map(|possible| {
                let res = run(&program, num << 3 | *possible, 0, 0);

                if res == expect {
                    Some(*possible)
                } else {
                    None
                }
            })
            .next();
    }

    let next_expect = vec![
        vec![program[program.len() - expect.len() - 1]],
        expect.clone(),
    ]
    .concat();

    possible_nexts
        .iter()
        .map(|next| find_next(program, next_expect.clone(), num << 3 | next))
        .filter(Option::is_some)
        .min()
        .flatten()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (program, _, _, _) = parse(input).ok()?.1;

    find_next(&program, vec![program[program.len() - 1]], 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
