use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::opt, IResult,
};

advent_of_code::solution!(13);

#[derive(Debug, Clone, Copy)]
struct Vector {
    x: i64,
    y: i64,
}

struct Game {
    a_button: Vector,
    b_button: Vector,
    prize: Vector,
}

fn parse_price(input: &str) -> IResult<&str, Vector> {
    let (input, _) = tag("Prize: X=")(input)?;
    let (input, x) = digit1(input)?;
    let (input, _) = tag(", Y=")(input)?;
    let (input, y) = digit1(input)?;
    let (input, _) = tag("\n")(input)?;

    Ok((
        input,
        Vector {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        },
    ))
}

fn parse_vector(input: &str) -> IResult<&str, Vector> {
    let (input, _) = tag("Button ")(input)?;
    let (input, _) = alt((tag("A: "), tag("B: ")))(input)?;
    let (input, _) = tag("X+")(input)?;
    let (input, x) = digit1(input)?;
    let (input, _) = tag(", Y+")(input)?;
    let (input, y) = digit1(input)?;
    let (input, _) = tag("\n")(input)?;

    Ok((
        input,
        Vector {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        },
    ))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, a_button) = parse_vector(input)?;
    let (input, b_button) = parse_vector(input)?;
    let (input, prize) = parse_price(input)?;
    let (input, _) = opt(tag("\n"))(input)?;

    Ok((
        input,
        Game {
            a_button,
            b_button,
            prize,
        },
    ))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = nom::multi::many0(parse_game)(input)?;

    Ok((input, games))
}

fn solve_euler(game: &Game) -> Option<u64> {
    let a = &game.a_button;
    let b = &game.b_button;
    let p = &game.prize;

    let det = a.x * b.y - a.y * b.x;
    let x = (b.y * p.x - b.x * p.y) / det;
    let y = (a.x * p.y - a.y * p.x) / det;

    // The solution we found might be floored due to integer division, so we need to check if it's valid
    if x < 0 || y < 0 || a.x * x + b.x * y != p.x || a.y * x + b.y * y != p.y {
        return None;
    }

    let result = x * 3 + y;

    Some(result as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    let games = parse_games(input).ok()?.1;

    Some(
        games
            .iter()
            .map(solve_euler)
            .filter(Option::is_some)
            .map(Option::unwrap)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let games = parse_games(input).ok()?.1;

    Some(
        games
            .iter()
            .map(|game| {
                return Game {
                    a_button: game.a_button,
                    b_button: game.b_button,
                    prize: Vector {
                        x: game.prize.x + 10_000_000_000_000,
                        y: game.prize.y + 10_000_000_000_000,
                    },
                };
            })
            .map(|game| solve_euler(&game))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
