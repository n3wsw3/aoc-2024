use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, digit1, none_of},
    combinator::{map_res, recognize, verify},
    multi::{many0, many1},
    sequence::tuple,
    IResult,
};

advent_of_code::solution!(3);

#[derive(Debug)]
struct Multiplication {
    x: u32,
    y: u32,
}

fn noise(input: &str) -> IResult<&str, Vec<char>> {
    many0(none_of("md"))(input)
}

fn number(input: &str) -> IResult<&str, u32> {
    map_res(
        verify(recognize(many1(digit1)), |s: &str| {
            1 <= s.len() && s.len() <= 3
        }),
        |s: &str| s.parse::<u32>(),
    )(input)
}

fn multiplication(input: &str) -> IResult<&str, Option<Multiplication>> {
    let (input, _) = tag("mul")(input)?;

    match tuple((char('('), number, char(','), number, char(')')))(input) {
        Ok((input, (_, x, _, y, _))) => Ok((input, Some(Multiplication { x, y }))),
        Err(_) => Ok((input, None)),
    }
}

fn one(input: &str) -> IResult<&str, char> {
    anychar(input)
}

pub fn part_one(mut input: &str) -> Option<u32> {
    let mut res = 0;

    while !input.is_empty() {
        match tuple((noise, multiplication))(input) {
            Ok((remaining, (_, Some(mul)))) => {
                res += mul.x * mul.y;
                input = remaining;
            }
            Ok((remaining, (_, None))) => {
                input = remaining;
            }
            Err(_) => {
                if let Ok((remaining, _)) = one(input) {
                    input = remaining;
                } else {
                    break;
                }
            }
        }
    }
    Some(res)
}

enum Instructions {
    Mul(Multiplication),
    Do,
    Dont,
}

fn multiplication_instruction(input: &str) -> IResult<&str, Option<Instructions>> {
    let (input, mul) = multiplication(input)?;
    Ok((input, mul.map(Instructions::Mul)))
}

fn do_instruction(input: &str) -> IResult<&str, Option<Instructions>> {
    let (input, _) = tag("do()")(input)?;

    Ok((input, Some(Instructions::Do)))
}

fn dont_instruction(input: &str) -> IResult<&str, Option<Instructions>> {
    let (input, _) = tag("don't()")(input)?;

    Ok((input, Some(Instructions::Dont)))
}

fn process_instructions(instructions: &Vec<Instructions>) -> Option<u32> {
    let mut res = 0;
    let mut enabled = true;
    for instruction in instructions {
        match instruction {
            Instructions::Mul(mul) => {
                if enabled {
                    res += mul.x * mul.y;
                }
            }
            Instructions::Do => {
                enabled = true;
            }
            Instructions::Dont => {
                enabled = false;
            }
        }
    }
    Some(res)
}

pub fn part_two(mut input: &str) -> Option<u32> {
    let mut instructions = Vec::new();

    while !input.is_empty() {
        match tuple((
            noise,
            alt((multiplication_instruction, do_instruction, dont_instruction)),
        ))(input)
        {
            Ok((remaining, (_, Some(instruction)))) => {
                instructions.push(instruction);
                input = remaining;
            }
            Ok((remaining, (_, None))) => {
                input = remaining;
            }
            Err(_) => {
                if let Ok((remaining, _)) = one(input) {
                    input = remaining;
                } else {
                    break;
                }
            }
        }
    }
    process_instructions(&instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
