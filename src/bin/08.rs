use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

advent_of_code::solution!(8);

enum Square {
    Empty,
    Antenna(char),
}

struct Map {
    width: usize,
    height: usize,
    grid: Vec<Vec<Square>>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<Square>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Square::Empty,
                        _ => Square::Antenna(c),
                    })
                    .collect()
            })
            .collect();

        Ok(Map {
            width: grid[0].len(),
            height: grid.len(),
            grid,
        })
    }
}

impl Map {
    fn get_antennas(&self) -> HashMap<char, Vec<(i32, i32)>> {
        let mut antennas = HashMap::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if let Square::Antenna(c) = self.grid[y][x] {
                    antennas
                        .entry(c)
                        .or_insert(Vec::new())
                        .push((x as i32, y as i32));
                }
            }
        }

        antennas
    }

    fn within_bounds(&self, x: i32, y: i32) -> bool {
        0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32
    }
}

fn solve(
    input: &str,
    nodes: fn(&Map, ((i32, i32), (i32, i32))) -> Option<Vec<(i32, i32)>>,
) -> Option<u32> {
    let map: Map = input.parse().ok()?;

    let antennas = map.get_antennas();

    Some(
        antennas
            .keys()
            .flat_map(|key| {
                antennas[key]
                    .iter()
                    .cartesian_product(antennas[key].iter())
                    .map(|(a, b)| nodes(&map, (*a, *b)))
                    .filter(Option::is_some)
                    .map(Option::unwrap)
                    .flatten()
                    .filter(|&(x, y)| map.within_bounds(x, y))
            })
            .unique()
            .count() as u32,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, |_, (a, b)| {
        if a == b {
            return None;
        }
        let diff = (b.0 - a.0, b.1 - a.1);

        Some(vec![
            (a.0 - diff.0, a.1 - diff.1),
            (b.0 + diff.0, b.1 + diff.1),
        ])
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, |map, (a, b)| {
        if a == b {
            return None;
        }
        let diff = (b.0 - a.0, b.1 - a.1);
        let mut anti_nodes = vec![a, b];

        let mut last = a;
        while map.within_bounds(last.0, last.1) {
            last = (last.0 - diff.0, last.1 - diff.1);
            anti_nodes.push(last);
        }

        last = b;
        while map.within_bounds(last.0, last.1) {
            last = (last.0 + diff.0, last.1 + diff.1);
            anti_nodes.push(last);
        }

        Some(anti_nodes)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
