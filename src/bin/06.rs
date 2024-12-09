use std::{collections::HashSet, hash::Hash};

use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum Square {
    Empty,
    Obstacle,
    Guard(Direction)
}

impl Square {

    fn from_char(s: char) -> Result<Self, ()> {
        match s {
            '.' => Ok(Square::Empty),
            '#' => Ok(Square::Obstacle),
            'v' => Ok(Square::Guard(Direction::Down)),
            '^' => Ok(Square::Guard(Direction::Up)),
            '<' => Ok(Square::Guard(Direction::Left)),
            '>' => Ok(Square::Guard(Direction::Right)),
            _ => Err(())
        }
    }
}

fn get_new_position(map: &Vec<Vec<Square>>, row: usize, col: usize, direction: Direction) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => {
            if row == 0 {
                None
            } else {
                Some((row - 1, col))
            }
        },
        Direction::Down => {
            if row == map.len() - 1 {
                None
            } else {
                Some((row + 1, col))
            }
        },
        Direction::Left => {
            if col == 0 {
                None
            } else {
                Some((row, col - 1))
            }
        },
        Direction::Right => {
            if col == map[0].len() - 1 {
                None
            } else {
                Some((row, col + 1))
            }
        }
    }
}

fn rotate_right(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::new(input);

    let guard = map.initial_guard();

    Some(map.visited_locations(guard).len() as u32)
}

type Guard = (usize, usize, Direction);

enum Step {
    Step(Guard),
    OutOfBounds
}

struct Map {
    map: Vec<Vec<Square>>,
}

impl Map {
    fn step(&self, (row, col, direction): Guard) -> Step {
        if let Some((new_row, new_col)) = get_new_position(&self.map, row, col, direction) {
            if self.map[new_row][new_col] != Square::Obstacle {
                return Step::Step((new_row, new_col, direction));
            } else {
                return Step::Step((row, col, rotate_right(direction)));
            }
        } else {
            return Step::OutOfBounds;
        }
    }

    fn new(input: &str) -> Map {
        Map {
            map: input.lines().map(|row| row.chars().map(|c| Square::from_char(c).unwrap()).collect_vec()).collect_vec()
        }
    }

    fn initial_guard(&self) -> Guard {
        let (guard_row, guard_col) = self.map.iter().enumerate().find_map(|(row_idx, row)| {
            row.iter().enumerate().find_map(|(col_idx, square)| {
                match square {
                    Square::Guard(_) => Some((row_idx, col_idx)),
                    _ => None
                }
            })
        }).unwrap();

        match self.map[guard_row][guard_col] {
            Square::Guard(direction) => (guard_row, guard_col, direction),
            _ => panic!("Guard not found")
        }
    }

    fn visited_locations(&self, mut guard: Guard) -> HashSet<(usize, usize)> {
        let mut visited = HashSet::new();
        visited.insert((guard.0, guard.1));

        loop {
            match self.step(guard) {
                Step::Step((new_row, new_col, new_direction)) => {
                    visited.insert((new_row, new_col));
                    guard = (new_row, new_col, new_direction);
                },
                Step::OutOfBounds => {
                    break;
                }
            }
        }

        visited
    }

    fn possible_loops(&mut self, guard: Guard) -> usize {
        let mut loops = 0;

        let visited_locations = self.visited_locations(guard);

        for (row, col) in visited_locations {
            let initial = self.map[row][col];
            self.map[row][col] = Square::Obstacle;

            if self.is_loop(guard) {
                loops += 1;
            }

            self.map[row][col] = initial;
        }

        loops
    }

    fn is_loop(&self, (row, col, direction): Guard) -> bool {
        let mut current = (row, col, direction);

        let mut visited = HashSet::new();
        visited.insert((row, col, direction));

        // Step until we reach the same position and direction again or we go out of bounds
        loop {
            match self.step(current) {
                Step::Step((new_row, new_col, new_direction)) => {
                    if visited.contains(&(new_row, new_col, new_direction)) {
                        return true;
                    }
                    visited.insert((new_row, new_col, new_direction));
                    current = (new_row, new_col, new_direction);
                },
                Step::OutOfBounds => {
                    break;
                }
            }
        }

        false
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::new(input);

    let guard = map.initial_guard();

    Some(map.possible_loops(guard) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
