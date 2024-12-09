use itertools::Itertools;

advent_of_code::solution!(4);

const SEARCH_WORD: &str = "XMAS";

fn check_forward(grid: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> u32 {
    if grid[row_idx].len() - col_idx < SEARCH_WORD.len() {
        // Not enough space to search
        return 0;
    }

    for i in 0..SEARCH_WORD.len() {
        if grid[row_idx][col_idx + i] != SEARCH_WORD.chars().nth(i).unwrap() {
            return 0;
        }
    }
    1
}

fn check_backward(grid: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> u32 {
    if col_idx < SEARCH_WORD.len() -1 {
        // Not enough space to search
        return 0;
    }

    for i in 0..SEARCH_WORD.len() {
        if grid[row_idx][col_idx - i] != SEARCH_WORD.chars().nth(i).unwrap() {
            return 0;
        }
    }
    1
}

fn check_down(grid: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> u32 {
    if grid.len() - row_idx < SEARCH_WORD.len() {
        // Not enough space to search
        return 0;
    }

    for i in 0..SEARCH_WORD.len() {
        if grid[row_idx + i][col_idx] != SEARCH_WORD.chars().nth(i).unwrap() {
            return 0;
        }
    }
    1
}

fn check_up(grid: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> u32 {
    if row_idx < SEARCH_WORD.len() - 1 {
        // Not enough space to search
        return 0;
    }

    for i in 0..SEARCH_WORD.len() {
        if grid[row_idx - i][col_idx] != SEARCH_WORD.chars().nth(i).unwrap() {
            return 0;
        }
    }
    1
}

fn check_up_left(grid: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> u32 {
    if row_idx < SEARCH_WORD.len() - 1 || col_idx < SEARCH_WORD.len() - 1 {
        // Not enough space to search
        return 0;
    }

    for i in 0..SEARCH_WORD.len() {
        if grid[row_idx - i][col_idx - i] != SEARCH_WORD.chars().nth(i).unwrap() {
            return 0;
        }
    }
    1
}

fn check_up_right(grid: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> u32 {
    if row_idx < SEARCH_WORD.len() - 1 || grid[row_idx].len() - col_idx < SEARCH_WORD.len() {
        // Not enough space to search
        return 0;
    }

    for i in 0..SEARCH_WORD.len() {
        if grid[row_idx - i][col_idx + i] != SEARCH_WORD.chars().nth(i).unwrap() {
            return 0;
        }
    }
    1
}

fn check_down_left(grid: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> u32 {
    if grid.len() - row_idx < SEARCH_WORD.len() || col_idx < SEARCH_WORD.len() - 1 {
        // Not enough space to search
        return 0;
    }

    for i in 0..SEARCH_WORD.len() {
        if grid[row_idx + i][col_idx - i] != SEARCH_WORD.chars().nth(i).unwrap() {
            return 0;
        }
    }
    1
}

fn check_down_right(grid: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> u32 {
    if grid.len() - row_idx < SEARCH_WORD.len() || grid[row_idx].len() - col_idx < SEARCH_WORD.len() {
        // Not enough space to search
        return 0;
    }

    for i in 0..SEARCH_WORD.len() {
        if grid[row_idx + i][col_idx + i] != SEARCH_WORD.chars().nth(i).unwrap() {
            return 0;
        }
    }
    1
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.lines().map(|line| line.chars().collect_vec()).collect_vec();
    let mut xmas_count = 0;

    for row_idx in 0..grid.len() {
        for col_idx in 0..grid[row_idx].len() {
            if grid[row_idx][col_idx] != 'X' {
                continue;
            }

            xmas_count += check_forward(&grid, row_idx, col_idx);
            xmas_count += check_backward(&grid, row_idx, col_idx);
            xmas_count += check_down(&grid, row_idx, col_idx);
            xmas_count += check_up(&grid, row_idx, col_idx);
            xmas_count += check_up_left(&grid, row_idx, col_idx);
            xmas_count += check_up_right(&grid, row_idx, col_idx);
            xmas_count += check_down_left(&grid, row_idx, col_idx);
            xmas_count += check_down_right(&grid, row_idx, col_idx);
        }
    }
    Some(xmas_count)
}

fn check_diagonals(grid: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> u32 {
    if row_idx < 1 || col_idx < 1 || grid.len() - row_idx <= 1 || grid[row_idx].len() - col_idx <= 1 {
        // Not enough space to search
        return 0;
    }

    let first = grid[row_idx - 1][col_idx - 1] == 'M' && grid[row_idx + 1][col_idx + 1] == 'S' ||
    grid[row_idx - 1][col_idx - 1] == 'S' && grid[row_idx + 1][col_idx + 1] == 'M';

    let second = grid[row_idx - 1][col_idx + 1] == 'M' && grid[row_idx + 1][col_idx - 1] == 'S' ||
    grid[row_idx - 1][col_idx + 1] == 'S' && grid[row_idx + 1][col_idx - 1] == 'M';

    (first && second) as u32
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input.lines().map(|line| line.chars().collect_vec()).collect_vec();
    let mut xmas_count = 0;

    for row_idx in 0..grid.len() {
        for col_idx in 0..grid[row_idx].len() {
            if grid[row_idx][col_idx] != 'A' {
                continue;
            }
            xmas_count += check_diagonals(&grid, row_idx, col_idx);
        }
    }

    Some(xmas_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
