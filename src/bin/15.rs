use std::{
    fmt::{Display, Formatter},
    ops::{Add, Sub},
    str::FromStr,
};

use itertools::Itertools;

advent_of_code::solution!(15);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_position(&self) -> Position {
        match self {
            Direction::Up => Position { x: 0, y: -1 },
            Direction::Down => Position { x: 0, y: 1 },
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Right => Position { x: 1, y: 0 },
        }
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Direction::Up),
            "v" => Ok(Direction::Down),
            "<" => Ok(Direction::Left),
            ">" => Ok(Direction::Right),
            _ => Err(format!("Unknown direction: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Wall,
    BoxL,
    BoxR,
    Box,
    Robot,
    Empty,
}

struct Map {
    squares: Vec<Vec<Square>>,
    robot: Position,
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.squares {
            for square in row {
                let c = match square {
                    Square::Wall => '#',
                    Square::BoxL => '[',
                    Square::BoxR => ']',
                    Square::Robot => '@',
                    Square::Empty => '.',
                    Square::Box => 'O',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum Version {
    PartOne,
    PartTwo,
}

impl Map {
    fn parse(s: &str, version: Version) -> Result<Self, String> {
        let mut squares = Vec::new();
        let mut robot = Position { x: 0, y: 0 };

        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let mut squares = match (version, c) {
                    (Version::PartOne, '#') => vec![Square::Wall],
                    (Version::PartTwo, '#') => vec![Square::Wall, Square::Wall],
                    (Version::PartOne, 'O') => vec![Square::Box],
                    (Version::PartTwo, 'O') => vec![Square::BoxL, Square::BoxR],
                    (Version::PartOne, '@') => {
                        robot = Position {
                            x: x as i32,
                            y: y as i32,
                        };
                        vec![Square::Robot]
                    }
                    (Version::PartTwo, '@') => {
                        robot = Position {
                            x: (x as i32) * 2,
                            y: y as i32,
                        };
                        vec![Square::Robot, Square::Empty]
                    }
                    (Version::PartOne, '.') => vec![Square::Empty],
                    (Version::PartTwo, '.') => vec![Square::Empty, Square::Empty],
                    _ => return Err(format!("Unknown character: {}", c)),
                };
                row.append(&mut squares);
            }
            squares.push(row);
        }

        Ok(Map { squares, robot })
    }
    fn can_move(&self, direction: Direction, position: Position) -> bool {
        let move_vector = direction.to_position();
        let new_position = position + move_vector;

        let current_square = self.squares[position.y as usize][position.x as usize];

        match (direction, current_square) {
            (Direction::Up | Direction::Down, Square::BoxL) => {
                self.can_move(direction, new_position)
                    && self.can_move(direction, new_position + Direction::Right.to_position())
            }
            (Direction::Up | Direction::Down, Square::BoxR) => {
                self.can_move(direction, new_position)
                    && self.can_move(direction, new_position + Direction::Left.to_position())
            }
            (_, Square::BoxL | Square::BoxR) => {
                self.can_move(direction, new_position + move_vector)
            }
            (_, Square::Wall) => false,
            (_, Square::Empty) => true,
            _ => self.can_move(direction, new_position),
        }
    }

    fn try_move_objects(&mut self, direction: Direction, position: Position) {
        let move_vector = direction.to_position();
        let new_position = position + move_vector;

        let current_square = self.squares[position.y as usize][position.x as usize];

        match (direction, current_square) {
            (Direction::Up | Direction::Down, Square::BoxR | Square::BoxL) => {
                let side = if current_square == Square::BoxL {
                    Direction::Right
                } else {
                    Direction::Left
                };
                self.try_move_objects(direction, new_position);
                self.try_move_objects(direction, new_position + side.to_position());

                let other_square = if current_square == Square::BoxL {
                    Square::BoxR
                } else {
                    Square::BoxL
                };
                let other = position + side.to_position();
                let new_other = other + move_vector;

                self.squares[new_position.y as usize][new_position.x as usize] = current_square;
                self.squares[new_other.y as usize][new_other.x as usize] = other_square;
                self.squares[position.y as usize][position.x as usize] = Square::Empty;
                self.squares[other.y as usize][other.x as usize] = Square::Empty;
            }
            (_, Square::BoxL | Square::BoxR) => {
                self.try_move_objects(direction, new_position);

                self.squares[new_position.y as usize][new_position.x as usize] = current_square;
                self.squares[position.y as usize][position.x as usize] = Square::Empty;
            }
            (_, Square::Empty) => {}
            _ => {
                self.try_move_objects(direction, new_position);

                self.squares[new_position.y as usize][new_position.x as usize] = current_square;
                self.squares[position.y as usize][position.x as usize] = Square::Empty;

                if current_square == Square::Robot {
                    self.robot = new_position;
                }
            }
        }
    }

    fn move_robot(&mut self, direction: Direction) {
        if !self.can_move(direction, self.robot) {
            return;
        }

        self.try_move_objects(direction, self.robot);
    }

    fn calculate_gps_coords_sum(&self) -> u32 {
        self.squares
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(y, &square)| {
                        if square == Square::Box || square == Square::BoxL {
                            Some(x * 100 + y)
                        } else {
                            None
                        }
                    })
                    .collect_vec()
            })
            .flatten()
            .sum::<usize>() as u32
    }
}

fn parse_instructions(s: &str) -> Vec<Direction> {
    s.chars()
        .map(|c| c.to_string().parse())
        .flatten()
        .collect_vec()
}

fn parse_input(s: &str, version: Version) -> (Map, Vec<Direction>) {
    let (first, last) = s.split_once("\n\n").unwrap();
    let map = Map::parse(first, version).unwrap();
    let instructions = parse_instructions(last);
    (map, instructions)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut map, instructions) = parse_input(input, Version::PartOne);

    for direction in instructions {
        map.move_robot(direction);
    }

    Some(map.calculate_gps_coords_sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut map, instructions) = parse_input(input, Version::PartTwo);

    for direction in instructions {
        map.move_robot(direction);
    }

    Some(map.calculate_gps_coords_sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
