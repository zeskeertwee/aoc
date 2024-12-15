use fxhash::FxHashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use aoclib::grid::Grid;
use aoclib::vec2::{Direction, Vector2};
use rayon::prelude::*;

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left
];

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Grid<u8> {
    Grid::parse(input, |c| c.to_digit(10).unwrap() as u8)
}

#[aoc(day10, part1)]
fn part1(input: &Grid<u8>) -> usize {
    find_paths::<false>(input)
}

#[aoc(day10, part2)]
fn part2(input: &Grid<u8>) -> usize {
    find_paths::<true>(input)
}

fn find_paths<const PART2: bool>(input: &Grid<u8>) -> usize {
    input.iter_squares()
        .filter(|(v, _)| **v == 0)
        .par_bridge()
        .map(|(_, pos)| make_step::<PART2>(pos, input, &mut FxHashSet::default()))
        .sum()
}

fn make_step<const PART2: bool>(position: Vector2<usize>, grid: &Grid<u8>, found: &mut FxHashSet<Vector2<usize>>) -> usize {
    let val = grid[&position];
    if val == 9 {
        // only count if we didn't count this top yet, or do if it's part 2
        return (PART2 || found.insert(position)) as usize;
    }

    DIRECTIONS.iter().map(|direction| {
        let new_pos = *direction + position;
        if !grid.is_inside(&new_pos) {
            0
        } else {
            let new_val = grid[&new_pos];
            if new_val == val + 1 {
                make_step::<PART2>(new_pos, grid, found)
            } else {
                0
            }
        }
    }).sum()
}

aoc_test!(test_day10_sample, "../samples/day10.txt", 36, 81);
aoc_test!(test_day10, "../input/2024/day10.txt", 659, 1463);