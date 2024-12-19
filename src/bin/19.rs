use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(19);

fn can_make_pattern<'a>(
    pattern: &'a str,
    towels: &Vec<&str>,
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    if pattern.is_empty() {
        return 1;
    }
    if let Some(&count) = memo.get(pattern) {
        return count;
    }

    let mut count = 0;
    for towel in towels {
        count += pattern
            .strip_prefix(towel)
            .map_or(0, |rest| can_make_pattern(rest, towels, memo));
    }
    memo.insert(pattern, count);
    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let (towels, patterns) = input.split("\n\n").collect_tuple().unwrap();
    let towels = towels.split(", ").collect();
    let patterns: Vec<&str> = patterns.lines().collect();

    let mut memo = HashMap::new();

    patterns
        .iter()
        .filter(|pattern| can_make_pattern(pattern, &towels, &mut memo) > 0)
        .count()
        .try_into()
        .ok()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (towels, patterns) = input.split("\n\n").collect_tuple().unwrap();
    let towels = towels.split(", ").collect();
    let patterns: Vec<&str> = patterns.lines().collect();

    let mut memo = HashMap::new();

    patterns
        .iter()
        .map(|pattern| can_make_pattern(pattern, &towels, &mut memo))
        .sum1()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
