use std::{collections::HashSet, fmt::Display};

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{opt, recognize},
    sequence::preceded,
    IResult,
};

advent_of_code::solution!(14);

struct Robot {
    x: i32,
    y: i32,
    v_x: i32,
    v_y: i32,
}

struct Map {
    robots: Vec<Robot>,
    size: (i32, i32),
}

impl Map {
    fn step(&mut self) {
        for robot in &mut self.robots {
            robot.x += robot.v_x;
            robot.y += robot.v_y;

            // the robot positions will wrap around the map
            // make sure they are always positive
            robot.x = ((robot.x % self.size.0) + self.size.0) % self.size.0;
            robot.y = ((robot.y % self.size.1) + self.size.1) % self.size.1;
        }
    }

    fn safety_factor(&self) -> u32 {
        // Get robots in each quadrant, robots between quadrants are ignored
        let mut quadrants = vec![0, 0, 0, 0];
        let half_size = (self.size.0 / 2, self.size.1 / 2);

        for robot in &self.robots {
            if robot.x == half_size.0 || robot.y == half_size.1 {
                // The robot is on the border of the map, ignore it
                continue;
            }
            let quadrant = if robot.x < half_size.0 {
                if robot.y < half_size.1 {
                    0
                } else {
                    1
                }
            } else {
                if robot.y < half_size.1 {
                    2
                } else {
                    3
                }
            };
            quadrants[quadrant] += 1;
        }

        quadrants.iter().product()
    }

    fn average_robot_density(&self) -> u32 {
        let mut pos_set = HashSet::<(i32, i32)>::new();
        for robot in &self.robots {
            pos_set.insert((robot.x, robot.y));
        }

        self.robots
            .iter()
            .map(|robot| {
                pos_set.contains(&(robot.x - 1, robot.y - 1)) as u32
                    + pos_set.contains(&(robot.x, robot.y - 1)) as u32
                    + pos_set.contains(&(robot.x + 1, robot.y - 1)) as u32
                    + pos_set.contains(&(robot.x - 1, robot.y)) as u32
                    + pos_set.contains(&(robot.x + 1, robot.y)) as u32
                    + pos_set.contains(&(robot.x - 1, robot.y + 1)) as u32
                    + pos_set.contains(&(robot.x, robot.y + 1)) as u32
                    + pos_set.contains(&(robot.x + 1, robot.y + 1)) as u32
            })
            .sum::<u32>()
            / self.robots.len() as u32
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = vec![vec!['.'; self.size.0 as usize]; self.size.1 as usize];

        for robot in &self.robots {
            map[robot.y as usize][robot.x as usize] = '#';
        }

        for row in map {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }

        Ok(())
    }
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    let (input, _) = tag("p=")(input)?;
    let (input, x) = digit1(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = digit1(input)?;
    let (input, _) = tag(" v=")(input)?;
    let (input, v_x) = recognize(preceded(opt(tag("-")), digit1))(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, v_y) = recognize(preceded(opt(tag("-")), digit1))(input)?;
    let (input, _) = tag("\n")(input)?;

    Ok((
        input,
        Robot {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            v_x: v_x.parse().unwrap(),
            v_y: v_y.parse().unwrap(),
        },
    ))
}
fn parse_input(input: &str, size: (i32, i32)) -> Option<Map> {
    let (_, robots) = nom::multi::many1(parse_robot)(input).ok()?;
    Some(Map { robots, size })
}

fn part_one_1(input: &str, size: (i32, i32)) -> Option<u32> {
    let mut map = parse_input(input, size)?;

    for _ in 0..100 {
        map.step();
    }

    Some(map.safety_factor())
}

pub fn part_one(input: &str) -> Option<u32> {
    part_one_1(input, (101, 103))
}

fn part_two_1(input: &str, size: (i32, i32)) -> Option<u32> {
    let mut map = parse_input(input, size)?;

    let mut candidates = Vec::new();

    for i in 0..10000 {
        map.step();
        if map.average_robot_density() > 1 {
            candidates.push(i + 1);
        }
    }

    candidates.first().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    part_two_1(input, (101, 103))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_1(
            &advent_of_code::template::read_file("examples", DAY),
            (11, 7),
        );
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_1(
            &advent_of_code::template::read_file("examples", DAY),
            (11, 7),
        );
        assert_eq!(result, None);
    }
}
