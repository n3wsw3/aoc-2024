use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

advent_of_code::solution!(12);

#[derive(Debug, Clone)]
struct Plot {
    plot_type: char,
    fence_top: bool,
    fence_right: bool,
    fence_bottom: bool,
    fence_left: bool,
}

impl Plot {
    fn new(map: &Vec<Vec<char>>, i: usize, j: usize) -> Self {
        let fence_top = i == 0 || map[i - 1][j] != map[i][j];
        let fence_right = j == map[i].len() - 1 || map[i][j + 1] != map[i][j];
        let fence_bottom = i == map.len() - 1 || map[i + 1][j] != map[i][j];
        let fence_left = j == 0 || map[i][j - 1] != map[i][j];

        Plot {
            plot_type: map[i][j],
            fence_top,
            fence_right,
            fence_bottom,
            fence_left,
        }
    }
}

struct Garden {
    map: Vec<Vec<Plot>>,
    regions: Vec<HashMap<(usize, usize), Plot>>,
}

impl FromStr for Garden {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char_map: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

        let map: Vec<Vec<Plot>> = char_map
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, c)| Plot::new(&char_map, i, j))
                    .collect()
            })
            .collect();

        // Flood fill to find regions
        let mut regions = Vec::<HashMap<(usize, usize), Plot>>::new();
        let mut visited = vec![vec![false; map[0].len()]; map.len()];

        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if visited[i][j] {
                    continue;
                }

                let mut stack = vec![(i, j)];
                let mut region = HashMap::<(usize, usize), Plot>::new();

                while let Some((x, y)) = stack.pop() {
                    if visited[x][y] {
                        continue;
                    }

                    visited[x][y] = true;

                    if x > 0 && !visited[x - 1][y] && map[x - 1][y].plot_type == map[x][y].plot_type
                    {
                        stack.push((x - 1, y));
                    }

                    if x < map.len() - 1
                        && !visited[x + 1][y]
                        && map[x + 1][y].plot_type == map[x][y].plot_type
                    {
                        stack.push((x + 1, y));
                    }

                    if y > 0 && !visited[x][y - 1] && map[x][y - 1].plot_type == map[x][y].plot_type
                    {
                        stack.push((x, y - 1));
                    }

                    if y < map[x].len() - 1
                        && !visited[x][y + 1]
                        && map[x][y + 1].plot_type == map[x][y].plot_type
                    {
                        stack.push((x, y + 1));
                    }
                    region.insert((x, y), map[x][y].clone());
                }

                regions.push(region);
            }
        }

        Ok(Garden { map, regions })
    }
}

impl Garden {
    fn get_plot_area_and_perimiter(&self) -> Vec<(u64, u64)> {
        self.regions
            .iter()
            .map(|plots| {
                let perimiter = plots
                    .iter()
                    .map(|(pos, plot)| {
                        let mut perimiter = 0;
                        if plot.fence_top {
                            perimiter += 1;
                        }
                        if plot.fence_right {
                            perimiter += 1;
                        }
                        if plot.fence_bottom {
                            perimiter += 1;
                        }
                        if plot.fence_left {
                            perimiter += 1;
                        }
                        perimiter
                    })
                    .sum::<u64>();
                let area = plots.len() as u64;
                (area, perimiter)
            })
            .collect()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .parse::<Garden>()
            .unwrap()
            .get_plot_area_and_perimiter()
            .iter()
            .map(|(a, p)| (a * p))
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let garden = input.parse::<Garden>().unwrap();

    Some(
        garden
            .regions
            .iter()
            .map(|region| {
                let mut visited_top = HashSet::<(usize, usize)>::new();
                let mut visited_right = HashSet::<(usize, usize)>::new();
                let mut visited_bottom = HashSet::<(usize, usize)>::new();
                let mut visited_left = HashSet::<(usize, usize)>::new();

                let mut sides = 0u32;

                for (pos, plot) in region {
                    if plot.fence_top && !visited_top.contains(pos) {
                        let mut next_list = vec![*pos];

                        while let Some(next) = next_list.pop() {
                            if visited_top.contains(&next) {
                                continue;
                            }

                            visited_top.insert(next);

                            if next.1 < garden.map[0].len() - 1
                                && garden.map[next.0][next.1 + 1].plot_type == plot.plot_type
                                && garden.map[next.0][next.1 + 1].fence_top
                                && !visited_top.contains(&(next.0, next.1 + 1))
                            {
                                next_list.push((next.0, next.1 + 1));
                            }
                            if next.1 > 0
                                && garden.map[next.0][next.1 - 1].plot_type == plot.plot_type
                                && garden.map[next.0][next.1 - 1].fence_top
                                && !visited_top.contains(&(next.0, next.1 - 1))
                            {
                                next_list.push((next.0, next.1 - 1));
                            }
                        }

                        sides += 1;
                    }
                    if plot.fence_bottom && !visited_bottom.contains(pos) {
                        let mut next_list = vec![*pos];

                        while let Some(next) = next_list.pop() {
                            if visited_bottom.contains(&next) {
                                continue;
                            }

                            visited_bottom.insert(next);

                            if next.1 < garden.map[0].len() - 1
                                && garden.map[next.0][next.1 + 1].plot_type == plot.plot_type
                                && garden.map[next.0][next.1 + 1].fence_bottom
                                && !visited_bottom.contains(&(next.0, next.1 + 1))
                            {
                                next_list.push((next.0, next.1 + 1));
                            }
                            if next.1 > 0
                                && garden.map[next.0][next.1 - 1].plot_type == plot.plot_type
                                && garden.map[next.0][next.1 - 1].fence_bottom
                                && !visited_bottom.contains(&(next.0, next.1 - 1))
                            {
                                next_list.push((next.0, next.1 - 1));
                            }
                        }

                        sides += 1;
                    }
                    if plot.fence_right && !visited_right.contains(pos) {
                        let mut next_list = vec![*pos];

                        while let Some(next) = next_list.pop() {
                            if visited_right.contains(&next) {
                                continue;
                            }

                            visited_right.insert(next);

                            if next.0 < garden.map.len() - 1
                                && garden.map[next.0 + 1][next.1].plot_type == plot.plot_type
                                && garden.map[next.0 + 1][next.1].fence_right
                                && !visited_right.contains(&(next.0 + 1, next.1))
                            {
                                next_list.push((next.0 + 1, next.1));
                            }
                            if next.0 > 0
                                && garden.map[next.0 - 1][next.1].plot_type == plot.plot_type
                                && garden.map[next.0 - 1][next.1].fence_right
                                && !visited_right.contains(&(next.0 - 1, next.1))
                            {
                                next_list.push((next.0 - 1, next.1));
                            }
                        }

                        sides += 1;
                    }
                    if plot.fence_left && !visited_left.contains(pos) {
                        let mut next_list = vec![*pos];

                        while let Some(next) = next_list.pop() {
                            if visited_left.contains(&next) {
                                continue;
                            }

                            visited_left.insert(next);

                            if next.0 < garden.map.len() - 1
                                && garden.map[next.0 + 1][next.1].plot_type == plot.plot_type
                                && garden.map[next.0 + 1][next.1].fence_left
                                && !visited_left.contains(&(next.0 + 1, next.1))
                            {
                                next_list.push((next.0 + 1, next.1));
                            }
                            if next.0 > 0
                                && garden.map[next.0 - 1][next.1].plot_type == plot.plot_type
                                && garden.map[next.0 - 1][next.1].fence_left
                                && !visited_left.contains(&(next.0 - 1, next.1))
                            {
                                next_list.push((next.0 - 1, next.1));
                            }
                        }

                        sides += 1;
                    }
                }
                sides * region.len() as u32
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
