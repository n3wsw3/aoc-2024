use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut x = input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            c.to_digit(10)
                .map(|d| vec![if i % 2 == 0 { Some(i as u64 / 2) } else { None }; d as usize])
        })
        .flatten()
        .flatten()
        .collect_vec();

    let mut right = 0;
    let mut left = x.len() - 1;

    let mut sum = 0;

    while right < left {
        while x[right].is_some() {
            sum += x[right].unwrap() * right as u64;
            right += 1;
        }
        while x[left].is_none() {
            left -= 1;
        }

        x.swap(right, left);
    }

    Some(sum)
}

struct Block {
    value: Option<u64>,
    size: usize,
    has_tried_to_move: bool,
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut x = input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            c.to_digit(10).map(|d| Block {
                value: if i % 2 == 0 { Some(i as u64 / 2) } else { None },
                size: d as usize,
                has_tried_to_move: false,
            })
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect_vec();

    while let Some(last_block_idx) = x
        .iter()
        .rposition(|b| b.value.is_some() && !b.has_tried_to_move)
    {
        let space = x
            .iter()
            .find_position(|b| b.value.is_none() && b.size >= x[last_block_idx].size);

        if space.is_none() || space.unwrap().0 >= last_block_idx {
            x[last_block_idx].has_tried_to_move = true;
            continue;
        }

        let space_idx = space.unwrap().0;
        let move_value = x[last_block_idx].value;
        let move_size = x[last_block_idx].size;

        x[last_block_idx].has_tried_to_move = true;
        x[last_block_idx].value = None;

        if x[space_idx].size > x[last_block_idx].size {
            x[space_idx].size -= x[last_block_idx].size;
        } else {
            x.remove(space_idx);
        }
        x.insert(
            space_idx,
            Block {
                value: move_value,
                size: move_size,
                has_tried_to_move: true,
            },
        );
    }

    let mut idx = 0;
    let mut sum = 0u64;

    for i in 0..x.len() {
        for _ in 0..x[i].size {
            if x[i].value.is_some() {
                sum += x[i].value.unwrap() * idx as u64;
            }

            idx += 1;
        }
    }

    Some(
        sum
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
