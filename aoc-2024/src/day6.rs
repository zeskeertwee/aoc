use fxhash::FxHashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;
use crate::util::Vector2;

type Direction = Vector2;
struct Input {
    map: Vec<Vec<char>>,
    // y,x coord of starting position
    starting_pos: Vector2
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Input {
    let mut starting_pos = Vector2::new(0, 0);

    Input {
        map: input.lines().enumerate()
            .map(|(y, l)| {
                if let Some(x) = l.find('^') {
                    starting_pos = Vector2::new(x as i64, y as i64)
                }

                l.chars().collect()
            })
            .collect(),
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
        grid[pos.y as usize][pos.x as usize] = '#';

        run_through_map_detect_loop(input.starting_pos, &grid) as usize
    }).sum()
}

#[aoc(day6, part2, naive)]
fn part2_naive(input: &Input) -> usize {
    (0..input.map.len()).into_par_iter().map(|y| {
        (0..input.map[0].len()).into_par_iter().map(|x| {
            if input.starting_pos.y as usize == y && input.starting_pos.x as usize == x {
                // don't drop an obstacle on the guard
                0
            } else if input.map[y][x] == '#' {
                0
            } else {
                let mut grid  = input.map.clone();
                grid[y][x] = '#';

                run_through_map_detect_loop(input.starting_pos, &grid) as usize
            }
        }).sum::<usize>()
    }).sum()
}

fn run_through_map_p1(starting_pos: Vector2, map: &Vec<Vec<char>>) -> FxHashSet<Vector2> {
    let map_width = map.len() as i64;
    let map_height = map[0].len() as i64;

    let mut position = starting_pos;
    let mut direction = Vector2::new(0, -1); // starting direction up

    let mut visited: FxHashSet<Vector2> = FxHashSet::default();
    visited.insert(position); // include starting position

    loop {
        let next_position = position + direction;
        if next_position.x < 0 || next_position.y < 0 || next_position.x >= map_width || next_position.y >= map_height {
            // fall off the map
            return visited;
        }

        if map[next_position.y as usize][next_position.x as usize] == '#' {
            direction = rot_direction(direction);
        } else {
            position = next_position;
            visited.insert(position);
        }
    }
}

// returns visited squares, hit loop
fn run_through_map_detect_loop(starting_pos: Vector2, map: &Vec<Vec<char>>) -> bool {
    let map_width = map.len() as i64;
    let map_height = map[0].len() as i64;

    let mut position = starting_pos;
    let mut direction = Vector2::new(0, -1); // starting direction up

    let mut rot_pos: FxHashSet<(Vector2, Direction)> = FxHashSet::default();

    loop {
        let next_position = position + direction;
        if next_position.x < 0 || next_position.y < 0 || next_position.x >= map_width || next_position.y >= map_height {
            // fall off the map
            return false;
        }

        let mut scan_pos = next_position;
        while map[scan_pos.y as usize][scan_pos.x as usize] != '#' {
            scan_pos = scan_pos + direction;
            if scan_pos.x < 0 || scan_pos.y < 0 || scan_pos.x >= map_width || scan_pos.y >= map_height {
                // fall off the map
                return false;
            }
        }

        if !rot_pos.insert((scan_pos, direction)) {
            // loop, we already were here in the same direction
            return true;
        }

        position = scan_pos - direction;
        direction = rot_direction(direction);
    }
}

fn rot_direction(dir: Direction) -> Direction {
    match (dir.x, dir.y) {
        (0, -1) => Vector2::new(1, 0), // up -> right
        (1, 0) => Vector2::new(0, 1), // right -> down
        (0, 1) => Vector2::new(-1, 0), // down -> left
        (-1, 0) => Vector2::new(0, -1), // left -> up
        _ => panic!("Unexpected direction!")
    }
}