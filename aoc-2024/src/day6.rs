use fxhash::FxHashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use rayon::prelude::*;
use aoclib::vec2::Vector2;
use aoclib::grid::Grid;
use aoclib::vec2::Direction;

struct Input {
    map: Grid<char>,
    // y,x coord of starting position
    starting_pos: Vector2<usize>
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Input {
    let mut starting_pos = Vector2::new(0, 0);
    let height = input.lines().count();

    Input {
        map: Grid::from_vec(input.lines().enumerate()
            .map(|(y, l)| {
                if let Some(x) = l.find('^') {
                    starting_pos = Vector2::new(x, y)
                }

                l.chars()
            })
            .flatten()
            .collect(), height),
        starting_pos
    }
}

#[aoc(day6, part1)]
fn part1(input: &Input) -> usize {
    run_through_map_p1(input.starting_pos, &input.map).len()
}

#[aoc(day6, part2)]
fn part2(input: &Input) -> usize {
    let mut visited_positions = run_through_map_p1(input.starting_pos, &input.map);
    visited_positions.remove(&input.starting_pos);

    // don't drop an obstacle on the guard
    visited_positions.remove(&input.starting_pos);

    visited_positions.into_par_iter().map(|pos| {
        let mut grid  = input.map.clone();
        grid[&pos] = '#';

        run_through_map_detect_loop(input.starting_pos, &grid) as usize
    }).sum()
}

fn run_through_map_p1(starting_pos: Vector2<usize>, map: &Grid<char>) -> FxHashSet<Vector2<usize>> {
    let mut position = starting_pos;
    let mut direction = Direction::Up; // starting direction up

    let mut visited: FxHashSet<Vector2<usize>> = FxHashSet::default();
    visited.insert(position); // include starting position

    loop {
        let next_position = direction + position;
        if !map.is_inside(&next_position) {
            // fall off the map
            return visited;
        }

        if map[&next_position] == '#' {
            direction = direction.rot();
        } else {
            position = next_position;
            visited.insert(position);
        }
    }
}

// returns visited squares, hit loop
fn run_through_map_detect_loop(starting_pos: Vector2<usize>, map: &Grid<char>) -> bool {
    let mut position = starting_pos;
    let mut direction = Direction::Up; // starting direction up

    let mut rot_pos: FxHashSet<(Vector2<usize>, Direction)> = FxHashSet::default();

    loop {
        let next_position = direction + position;
        if !map.is_inside(&next_position) {
            // fall off the map
            return false;
        }

        let mut scan_pos = next_position;
        while map[&scan_pos] != '#' {
            scan_pos = direction + scan_pos;
            if !map.is_inside(&scan_pos) {
                // fall off the map
                return false;
            }
        }

        if !rot_pos.insert((scan_pos, direction)) {
            // loop, we already were here in the same direction
            return true;
        }

        position = direction.inv() + scan_pos;
        direction = direction.rot();
    }
}

aoc_test!(test_day6_sample, "../samples/day6.txt", 41, 6);
aoc_test!(test_day6, "../input/2024/day6.txt", 4826, 1721);