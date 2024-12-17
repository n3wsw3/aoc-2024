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
enum Square2 {
    Wall,
    BoxL,
    BoxR,
    Box,
    Robot,
    Empty,
}

struct Map2 {
    squares: Vec<Vec<Square2>>,
    robot: Position,
}

impl Display for Map2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.squares {
            for square in row {
                let c = match square {
                    Square2::Wall => '#',
                    Square2::BoxL => '[',
                    Square2::BoxR => ']',
                    Square2::Robot => '@',
                    Square2::Empty => '.',
                    Square2::Box => 'O',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map2 {
    fn parse_1(s: &str) -> Result<Self, String> {
        let mut squares = Vec::new();
        let mut robot = Position { x: 0, y: 0 };

        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let mut squares = match c {
                    '#' => vec![Square2::Wall],
                    'O' => vec![Square2::Box],
                    '@' => {
                        robot = Position {
                            x: x as i32,
                            y: y as i32,
                        };
                        vec![Square2::Robot]
                    }
                    '.' => vec![Square2::Empty],
                    _ => return Err(format!("Unknown character: {}", c)),
                };
                row.append(&mut squares);
            }
            squares.push(row);
        }

        Ok(Map2 { squares, robot })
    }

    fn parse_2(s: &str) -> Result<Self, String> {
        let mut squares = Vec::new();
        let mut robot = Position { x: 0, y: 0 };

        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let mut squares = match c {
                    '#' => vec![Square2::Wall, Square2::Wall],
                    'O' => vec![Square2::BoxL, Square2::BoxR],
                    '@' => {
                        robot = Position {
                            x: (x as i32) * 2,
                            y: y as i32,
                        };
                        vec![Square2::Robot, Square2::Empty]
                    }
                    '.' => vec![Square2::Empty, Square2::Empty],
                    _ => return Err(format!("Unknown character: {}", c)),
                };
                row.append(&mut squares);
            }
            squares.push(row);
        }

        Ok(Map2 { squares, robot })
    }
    fn can_move(&self, direction: Direction, position: Position) -> bool {
        let move_vector = direction.to_position();
        let new_position = position + move_vector;

        let current_square = self.squares[position.y as usize][position.x as usize];

        match (direction, current_square) {
            (Direction::Up | Direction::Down, Square2::BoxL) => {
                self.can_move(direction, new_position)
                    && self.can_move(direction, new_position + Direction::Right.to_position())
            }
            (Direction::Up | Direction::Down, Square2::BoxR) => {
                self.can_move(direction, new_position)
                    && self.can_move(direction, new_position + Direction::Left.to_position())
            }
            (_, Square2::BoxL | Square2::BoxR) => {
                self.can_move(direction, new_position + move_vector)
            }
            (_, Square2::Wall) => false,
            (_, Square2::Empty) => true,
            _ => self.can_move(direction, new_position),
        }
    }

    fn try_move_objects(&mut self, direction: Direction, position: Position) {
        let move_vector = direction.to_position();
        let new_position = position + move_vector;

        let current_square = self.squares[position.y as usize][position.x as usize];

        match (direction, current_square) {
            (Direction::Up | Direction::Down, Square2::BoxR | Square2::BoxL) => {
                let side = if current_square == Square2::BoxL {
                    Direction::Right
                } else {
                    Direction::Left
                };
                self.try_move_objects(direction, new_position);
                self.try_move_objects(direction, new_position + side.to_position());

                let other_square = if current_square == Square2::BoxL {
                    Square2::BoxR
                } else {
                    Square2::BoxL
                };
                let other = position + side.to_position();
                let new_other = other + move_vector;

                self.squares[new_position.y as usize][new_position.x as usize] = current_square;
                self.squares[new_other.y as usize][new_other.x as usize] = other_square;
                self.squares[position.y as usize][position.x as usize] = Square2::Empty;
                self.squares[other.y as usize][other.x as usize] = Square2::Empty;
            }
            (_, Square2::BoxL | Square2::BoxR) => {
                self.try_move_objects(direction, new_position);

                self.squares[new_position.y as usize][new_position.x as usize] = current_square;
                self.squares[position.y as usize][position.x as usize] = Square2::Empty;
            }
            (_, Square2::Empty) => {}
            _ => {
                self.try_move_objects(direction, new_position);

                self.squares[new_position.y as usize][new_position.x as usize] = current_square;
                self.squares[position.y as usize][position.x as usize] = Square2::Empty;

                if current_square == Square2::Robot {
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
}

pub fn part_one(input: &str) -> Option<u32> {
    let (first, last) = input.split_once("\n\n")?;
    let mut map = Map2::parse_1(first).ok()?;
    let instructions: Vec<Direction> = last
        .chars()
        .map(|c| c.to_string().parse())
        .flatten()
        .collect_vec();

    for direction in instructions {
        map.move_robot(direction);
    }

    Some(
        map.squares
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(y, square)| {
                        if let Square2::Box = square {
                            Some(x * 100 + y)
                        } else {
                            None
                        }
                    })
                    .collect_vec()
            })
            .flatten()
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (first, last) = input.split_once("\n\n")?;
    let mut map: Map2 = Map2::parse_2(first).ok()?;
    let instructions: Vec<Direction> = last
        .chars()
        .map(|c| c.to_string().parse())
        .flatten()
        .collect_vec();

    for direction in instructions {
        map.move_robot(direction);
    }

    let left_boxes = map
        .squares
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, square)| {
                    if let Square2::BoxL = square {
                        Some(Position {
                            x: x as i32,
                            y: y as i32,
                        })
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .flatten()
        .map(|pos| pos.y * 100 + pos.x)
        .sum::<i32>();

    Some(left_boxes as u32)
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
