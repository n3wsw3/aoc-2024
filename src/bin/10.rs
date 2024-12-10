use std::str::FromStr;

advent_of_code::solution!(10);

struct Map {
    map: Vec<Vec<u32>>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Map {
            map: input
                .lines()
                .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
                .collect(),
        })
    }
}

impl Map {
    fn get_starting_points(&self) -> Vec<(usize, usize)> {
        let mut starts = Vec::new();

        for (y, row) in self.map.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == 0 {
                    starts.push((x, y));
                }
            }
        }

        starts
    }

    fn get_neighbors(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < self.map[0].len() - 1 {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < self.map.len() - 1 {
            neighbors.push((x, y + 1));
        }

        neighbors
    }

    fn valid_neighbors(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        self.get_neighbors((x, y))
            .into_iter()
            .filter(|pos| self.get_cell(*pos) == self.get_cell((x, y)) + 1)
            .collect()
    }

    fn get_cell(&self, (x, y): (usize, usize)) -> u32 {
        self.map[y][x]
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Map = input.parse().unwrap();

    let starting_points = map.get_starting_points();

    let mut amount_of_reachable_goals = 0;
    for starting_point in starting_points {
        let mut visited = vec![vec![false; map.map[0].len()]; map.map.len()];
        let mut queue = vec![starting_point];

        while let Some((x, y)) = queue.pop() {
            if visited[y][x] {
                continue;
            }

            visited[y][x] = true;

            if map.get_cell((x, y)) == 9 {
                amount_of_reachable_goals += 1;
            }

            for neighbor in map.valid_neighbors((x, y)) {
                queue.push(neighbor);
            }
        }
    }

    Some(amount_of_reachable_goals)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Map = input.parse().unwrap();

    let starting_points = map.get_starting_points();

    let mut amount_of_reachable_goals = 0;
    for starting_point in starting_points {
        let mut visited = vec![vec![false; map.map[0].len()]; map.map.len()];
        let mut queue = vec![starting_point];

        while let Some((x, y)) = queue.pop() {
            visited[y][x] = true;

            if map.get_cell((x, y)) == 9 {
                amount_of_reachable_goals += 1;
            }

            for neighbor in map.valid_neighbors((x, y)) {
                queue.push(neighbor);
            }
        }
    }

    Some(amount_of_reachable_goals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
