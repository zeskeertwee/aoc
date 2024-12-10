use fxhash::FxHashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use crate::util::Vector2;

const DIRECTIONS: [Vector2; 4] = [
    Vector2::new(1, 0),
    Vector2::new(0, 1),
    Vector2::new(0, -1),
    Vector2::new(-1, 0)
];

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|v| v.chars().map(|v| v.to_digit(10).unwrap() as u8).collect()).collect()
}

#[aoc(day10, part1)]
fn part1(input: &Vec<Vec<u8>>) -> usize {
    find_paths::<false>(input)
}

#[aoc(day10, part2)]
fn part2(input: &Vec<Vec<u8>>) -> usize {
    find_paths::<true>(input)
}

fn find_paths<const PART2: bool>(input: &Vec<Vec<u8>>) -> usize {
    input.iter().enumerate()
        .map(|(y, row)| row.iter().enumerate().filter(|(_, v)| **v == 0).map(move |(x, _)| Vector2::new(x as i64, y as i64)))
        .flatten()
        .map(|pos| make_step::<PART2>(pos, input, &mut FxHashSet::default()))
        .sum()
}

fn make_step<const PART2: bool>(position: Vector2, grid: &Vec<Vec<u8>>, found: &mut FxHashSet<Vector2>) -> usize {
    let val = grid[position.y as usize][position.x as usize];
    if val == 9 {
        // only count if we didn't count this top yet, or do if it's part 2
        return (PART2 || found.insert(position)) as usize;
    }

    let height = grid.len() as i64;
    let width = grid[0].len() as i64;

    DIRECTIONS.iter().map(|direction| {
        let new_pos = position + *direction;
        if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= width || new_pos.y >= height {
            0
        } else {
            let new_val = grid[new_pos.y as usize][new_pos.x as usize];
            if new_val == val + 1 {
                make_step::<PART2>(new_pos, grid, found)
            } else {
                0
            }
        }
    }).sum()
}