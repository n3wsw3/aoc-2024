use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use itertools::Itertools;

advent_of_code::solution!(20);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Wall,
    Empty,
    Start,
    End,
}

type Location = (i64, i64);

struct Map {
    squares: Vec<Vec<Square>>,
    end: Location,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut squares = Vec::new();
        let mut end = None;

        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let square = match c {
                    '#' => Square::Wall,
                    '.' => Square::Empty,
                    'S' => {
                        Square::Start
                    }
                    'E' => {
                        end = Some((x as i64, y as i64));
                        Square::End
                    }
                    _ => panic!("unexpected character: {}", c),
                };
                row.push(square);
            }
            squares.push(row);
        }

        let end = end.ok_or("end not found")?;

        Ok(Map {
            squares,
            end,
        })
    }
}

impl Map {
    fn neighbours(&self, pos: Location) -> Vec<Location> {
        let mut neighbours = Vec::new();
        let (x, y) = pos;

        if x > 0 && self.squares[y as usize][x as usize - 1] != Square::Wall {
            neighbours.push((x - 1, y));
        }
        if y > 0 && self.squares[y as usize - 1][x as usize] != Square::Wall {
            neighbours.push((x, y - 1));
        }
        if x < self.squares[0].len() as i64 - 1 && self.squares[y as usize][x as usize + 1] != Square::Wall {
            neighbours.push((x + 1, y));
        }
        if y < self.squares.len() as i64 - 1 && self.squares[y as usize + 1][x as usize] != Square::Wall {
            neighbours.push((x, y + 1));
        }

        neighbours
    }
}

fn manhattan_neighbours(pos: Location, distance: i64) -> Vec<Location> {
    // Find all locations that are at most `distance` manhattan distance away from `pos`

    let mut neighbours = Vec::new();

    for x in pos.0 - distance..=pos.0 + distance {
        for y in pos.1 - distance..=pos.1 + distance {
            if x == pos.0 && y == pos.1 {
                continue;
            }
            if (x - pos.0).abs() + (y - pos.1).abs() <= distance {
                neighbours.push((x, y));
            }
        }
    }

    neighbours
}

fn manhattan_distance(a: Location, b: Location) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn solve(input: &str, distance: i64) -> Option<usize> {
    let map: Map = input.parse().expect("Unable to parse input");

    let mut steps_to_end = HashMap::<Location, i64>::new();

    let mut queue = VecDeque::<(Location, i64)>::new();

    queue.push_back((map.end, 0));

    while let Some((pos, steps)) = queue.pop_front() {
        if steps_to_end.contains_key(&pos) && steps_to_end[&pos] <= steps {
            continue;
        }

        steps_to_end.insert(pos, steps);

        for neighbour in map.neighbours(pos) {
            if steps_to_end.contains_key(&neighbour) && steps_to_end[&neighbour] < steps {
                continue;
            }
            queue.push_back((neighbour, steps + 1));
        }
    }

    Some(steps_to_end.iter().map(|(loc, steps)| {
        manhattan_neighbours(*loc, distance)
            .iter()
            .filter_map(|loc| steps_to_end.get(loc).map(|step| (*loc, *step)))
            .map(|(neighbour_loc, neighbour_step)| {
                neighbour_step - steps - manhattan_distance(neighbour_loc, *loc)
            })
            .collect_vec()
    }).flatten().sorted().dedup_with_count().filter(|(_, saved)| *saved > 0).filter_map(|(count, saved)| {
        if saved >= 100 {
            return Some(count)
        } else {
            None
        }
    }).sum())
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 20)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
