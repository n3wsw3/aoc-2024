use std::{cmp::Ordering, collections::BinaryHeap};

use nom::{bytes::complete::tag, character::complete::digit1, multi::many1, IResult};

advent_of_code::solution!(18);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Open,
    Corrupted,
    Visited(u32),
}

fn parse_block(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, x) = digit1(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = digit1(input)?;
    let (input, _) = tag("\n")(input)?;

    Ok((input, (x.parse().unwrap(), y.parse().unwrap())))
}

fn parse_falling_blocks(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    let (input, blocks) = many1(parse_block)(input)?;

    Ok((input, blocks))
}

fn should_visit(square: Square, steps: u32) -> bool {
    match square {
        Square::Visited(visited_steps) => visited_steps > steps,
        Square::Open => true,
        _ => false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    steps: u32,
    current: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.steps.cmp(&self.steps)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn steps_to_goal(
    map: &mut Vec<Vec<Square>>,
    map_size: usize,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<u32> {
    let mut queue = BinaryHeap::new();
    queue.push(State {
        steps: 0,
        current: start,
    });

    while let Some(State { current, steps }) = queue.pop() {
        if current == end {
            return Some(steps);
        }

        let (row, col) = current;

        if row > 0 && should_visit(map[row - 1][col], steps + 1) {
            map[row - 1][col] = Square::Visited(steps + 1);
            queue.push(State {
                steps: steps + 1,
                current: (row - 1, col),
            });
        }

        if row < map_size - 1 && should_visit(map[row + 1][col], steps + 1) {
            map[row + 1][col] = Square::Visited(steps + 1);
            queue.push(State {
                steps: steps + 1,
                current: (row + 1, col),
            });
        }

        if col > 0 && should_visit(map[row][col - 1], steps + 1) {
            map[row][col - 1] = Square::Visited(steps + 1);
            queue.push(State {
                steps: steps + 1,
                current: (row, col - 1),
            });
        }

        if col < map_size - 1 && should_visit(map[row][col + 1], steps + 1) {
            map[row][col + 1] = Square::Visited(steps + 1);
            queue.push(State {
                steps: steps + 1,
                current: (row, col + 1),
            });
        }
    }

    None
}

fn part_one_solve(input: &str, map_size: usize, take_size: usize) -> Option<u32> {
    let (_, blocks) = parse_falling_blocks(input).unwrap();

    let mut map = vec![vec![Square::Open; map_size]; map_size];

    for &(col, row) in blocks.iter().take(take_size) {
        map[row][col] = Square::Corrupted;
    }

    let start: (usize, usize) = (0, 0);
    let end: (usize, usize) = (map_size - 1, map_size - 1);

    steps_to_goal(&mut map, map_size, start, end)
}

pub fn part_one(input: &str) -> Option<u32> {
    part_one_solve(input, 71, 1024)
}

fn part_two_solve(input: &str, map_size: usize, take_size: usize) -> Option<String> {
    let (_, blocks) = parse_falling_blocks(input).unwrap();

    let mut map = vec![vec![Square::Open; map_size]; map_size];

    let mut blocks_iter = blocks.iter();

    for _ in 0..take_size {
        if let Some(&(col, row)) = blocks_iter.next() {
            map[row][col] = Square::Corrupted;
        }
    }

    let start: (usize, usize) = (0, 0);
    let end: (usize, usize) = (map_size - 1, map_size - 1);

    let mut last_block = (0, 0);

    while let Some(_) = steps_to_goal(&mut map, map_size, start, end) {
        if let Some(&(col, row)) = blocks_iter.next() {
            map[row][col] = Square::Corrupted;
            last_block = (col, row);

            // Reset the map

            for row in map.iter_mut() {
                for square in row.iter_mut() {
                    if let Square::Visited(_) = square {
                        *square = Square::Open;
                    }
                }
            }
        } else {
            panic!("Ran out of blocks");
        }
    }

    Some(format!("{},{}", last_block.0, last_block.1))
}

pub fn part_two(input: &str) -> Option<String> {
    part_two_solve(input, 71, 1024)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_solve(&advent_of_code::template::read_file("examples", DAY), 7, 12);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_solve(&advent_of_code::template::read_file("examples", DAY), 7, 12);
        assert_eq!(result, Some("6,1".to_owned()));
    }
}
