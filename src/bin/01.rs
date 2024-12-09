use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut right = Vec::new();
    let mut left = Vec::new();

    for line in input.lines() {
        let (l, r) = line.split_once("   ")?;
        left.push(l.parse::<u32>().ok()?);
        right.push(r.parse::<u32>().ok()?);
    }

    right.sort();
    left.sort();

    Some(left.iter().zip(right.iter()).map(|(r, l)| if r > l { r - l } else { l - r }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut right = HashMap::new();
    let mut left = Vec::new();

    for line in input.lines() {
        let (l, r) = line.split_once("   ").map(|(l, r)| (l.parse::<u32>().ok().unwrap(), r.parse::<u32>().ok().unwrap()))?;
        left.push(l);
        right.insert(r, right.get(&r).unwrap_or(&0u32) + 1);
    }

    Some(left.iter().map(|l| right.get(l).unwrap_or(&0) * l).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
