use std::{collections::HashMap, mem::swap};

advent_of_code::solution!(11);

fn solve(input: &str, steps: usize) -> Option<u64> {
    let mut stone_count_1 = input
        .split_whitespace()
        .map(|s| (s.parse::<u64>().unwrap(), 1))
        .collect::<HashMap<u64, u64>>();
    let mut stone_count_2 = HashMap::<u64, u64>::new();

    for _ in 0..steps {
        for (stone, count) in stone_count_1.iter() {
            match stone {
                0 => {
                    *stone_count_2.entry(1).or_insert(0) += count;
                }
                n if i64::ilog10(*n as i64) % 2 == 1 => {
                    let len = i64::ilog10(*stone as i64) + 1;

                    let pow = 10u64.pow(len / 2 as u32);

                    let first = n / pow;
                    let second = n % pow;

                    *stone_count_2.entry(first).or_insert(0) += count;
                    *stone_count_2.entry(second).or_insert(0) += count;
                }
                n => {
                    *stone_count_2.entry(n * 2024).or_insert(0) += count;
                }
            }
        }

        swap(&mut stone_count_1, &mut stone_count_2);
        stone_count_2.clear();
    }

    Some(stone_count_1.values().sum::<u64>())
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 25)?)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 75)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
