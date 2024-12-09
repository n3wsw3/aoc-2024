use itertools::Itertools;

advent_of_code::solution!(2);

fn is_safe_1(line: &str) -> bool {
    let x = line.split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .tuple_windows()
        .map(|(a, b)| ((a - b).abs(), a < b))
        .collect_vec();

    x.iter().all(|(diff, _)| 1 <= *diff && *diff <= 3) &&
    (x.iter().all(|(_, increasing)| *increasing) || x.iter().all(|(_, increasing)| !increasing))
}

fn is_safe(difference: i32, is_increasing: bool) -> bool {
    if is_increasing {
        difference.signum() != 1 && 1 <= difference.abs() && difference.abs() <= 3
    } else {
        difference.signum() != -1 && 1 <= difference.abs() && difference.abs() <= 3
    }
}

fn is_safe_list(values: &Vec<i32>, is_increasing: bool, skip_index: Option<usize>) -> bool {
    for i in 0..values.len() {
        if skip_index == Some(i) {
            continue;
        }
        let next_index = if skip_index == Some(i+1) {  i + 2 } else { i + 1 };
        if next_index >= values.len() {
            continue;
        }
        let current = values[i];
        let next = values[next_index];

        if !is_safe(current - next, is_increasing) {
            return false;
        }
    }
    true
}

fn is_safe_2(line: &str) -> bool {
    let x = line.split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect_vec();

    is_safe_list(&x, false, None) ||
    is_safe_list(&x, true, None) ||
    (0..x.len()).any(|i| is_safe_list(&x, true, Some(i)) || is_safe_list(&x, false, Some(i)))
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(is_safe_1).map(u32::from).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(is_safe_2).map(u32::from).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
