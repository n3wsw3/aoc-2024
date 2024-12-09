use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(5);

fn parse_dependencies(input: &str) -> HashMap<u32, Vec<u32>> {
    let mut dependencies = HashMap::new();

    for line in input.lines() {
        let (dependency, dependent) = line.split_once('|').unwrap();

        dependencies
            .entry(dependent.parse().unwrap())
            .or_insert_with(Vec::new)
            .push(dependency.parse().unwrap());
    }

    dependencies
}

fn is_valid_vec(dependencies: &HashMap<u32, Vec<u32>>, updates: &Vec<u32>) -> bool {
    let updates_set = updates.iter().copied().collect::<HashSet<_>>();
    let mut updated = HashSet::new();

    for update in updates {
        if let Some(dependents) = dependencies.get(&update) {
            for dependent in dependents {
                if updates_set.contains(dependent) && !updated.contains(dependent) {
                    return false;
                }
            }
        }

        updated.insert(update);
    }

    true
}

fn is_valid(dependencies: &HashMap<u32, Vec<u32>>, updates: &str) -> bool {
    let updates = updates
        .split(',')
        .map(|update| update.parse().unwrap())
        .collect_vec();

    is_valid_vec(dependencies, &updates)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (dependencies, updates) = input.split_once("\n\n")?;

    let dependencies = parse_dependencies(dependencies);

    Some(
        updates
            .lines()
            .filter(|line| is_valid(&dependencies, line))
            .map(|line| {
                line.split(',')
                    .map(|update| update.parse::<u32>().unwrap())
                    .collect_vec()
            })
            .map(|line| line[line.len() / 2])
            .sum(),
    )
}

fn return_invalid(dependencies: &HashMap<u32, Vec<u32>>, updates: &str) -> Option<Vec<u32>> {
    let updates = updates
        .split(',')
        .map(|update| update.parse().unwrap())
        .collect_vec();

    if is_valid_vec(dependencies, &updates) {
        return None;
    }

    Some(updates)
}

fn fix_invalid(dependencies: &HashMap<u32, Vec<u32>>, invalid: &Vec<u32>) -> Vec<u32> {
    let mut invalid = invalid.clone();

    let mut i = 0;

    while i < invalid.len() {
        let update = invalid[i];
        let mut should_increment = true;

        if let Some(dependents) = dependencies.get(&update) {
            for dependent in dependents {
                if let Some(index) = invalid.iter().position(|&x| x == *dependent) {
                    if index < i {
                        continue;
                    }
                    should_increment = false;
                    invalid.remove(index);
                    invalid.insert(i, *dependent);
                }
            }
        }

        if should_increment {
            i += 1;
        }
    }

    invalid
}

pub fn part_two(input: &str) -> Option<u32> {
    let (dependencies, updates) = input.split_once("\n\n")?;

    let dependencies = parse_dependencies(dependencies);

    Some(
        updates
            .lines()
            .map(|line| return_invalid(&dependencies, line))
            .flatten()
            .map(|invalid| fix_invalid(&dependencies, &invalid))
            .map(|line| line[line.len() / 2])
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
