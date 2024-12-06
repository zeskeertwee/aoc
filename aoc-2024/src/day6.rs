use std::collections::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

type Direction = (isize, isize);
struct Input {
    map: Vec<Vec<char>>,
    // y,x coord of starting position
    starting_pos: (isize, isize)
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Input {
    let mut starting_pos = (0,0);

    Input {
        map: input.lines().enumerate()
            .map(|(y, l)| {
                if let Some(x) = l.find('^') {
                    starting_pos = (y as isize, x as isize)
                }

                l.chars().collect()
            })
            .collect(),
        starting_pos
    }
}

#[aoc(day6, part1)]
fn part1(input: &Input) -> usize {
    run_through_map(input.starting_pos, &input.map, usize::MAX).0.len()
}

#[aoc(day6, part2)]
fn part2(input: &Input) -> usize {
    let visited_positions = run_through_map(input.starting_pos, &input.map, usize::MAX).0;

    visited_positions.into_par_iter().map(|(y,x)| {
        let mut grid  = input.map.clone();
        grid[y as usize][x as usize] = '#';

        if run_through_map(input.starting_pos, &grid, usize::MAX).1 == usize::MAX {
            1
        } else {
            0
        }
    }).sum()
}

#[aoc(day6, part2, naive)]
fn part2_naive(input: &Input) -> usize {
    let upper_limit = input.map.len() * input.map[0].len(); // if we walk as many squares as the grid has, we probably hit a loop, right?

    (0..input.map.len()).into_par_iter().map(|y| {
        (0..input.map[0].len()).into_par_iter().map(|x| {
            if input.map[y][x] == '#' {
                0
            } else {
                let mut grid  = input.map.clone();
                grid[y][x] = '#';

                if run_through_map(input.starting_pos, &grid, upper_limit).1 == upper_limit {
                    1
                } else {
                    0
                }
            }
        }).sum::<usize>()
    }).sum()
}

// returns visited squares, steps
fn run_through_map(starting_pos: (isize, isize), map: &Vec<Vec<char>>, max_steps: usize) -> (HashSet<(isize, isize)>, usize) {
    let mut position = starting_pos;
    let mut direction = (-1, 0); // starting direction up
    let mut steps = 0;

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    visited.insert(position); // include starting position

    let mut rot_pos: HashSet<(isize, isize, Direction)> = HashSet::new();

    loop {
        let next_position = (position.0 + direction.0, position.1 + direction.1);
        if next_position.0 < 0 || next_position.1 < 0 || next_position.0 as usize >= map.len() || next_position.1 as usize >= map[0].len() {
            //for l in grid {
            //    println!("{}", l.iter().collect::<String>());
            //}
            // fall off the map
            return (visited, steps);
        }

        if map[next_position.0 as usize][next_position.1 as usize] == '#' {
            if !rot_pos.insert((position.0, position.1, direction)) {
                // loop, we already were here in the same direction
                //dbg!(steps);
                return (visited, max_steps);
            }
            direction = rot_direction(direction);
        } else {
            steps += 1;

            position = next_position;
            visited.insert(position);

            if steps == max_steps {
                return (visited, steps);
            }
        }
    }
}

fn rot_direction(dir: Direction) -> Direction {
    match dir {
        (-1, 0) => (0, 1), // up -> right
        (0, 1) => (1, 0), // right -> down
        (1, 0) => (0, -1), // down -> left
        (0, -1) => (-1, 0), // left -> up
        _ => panic!("Unexpected direction!")
    }
}