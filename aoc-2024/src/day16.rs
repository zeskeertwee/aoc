use std::collections::VecDeque;
use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use aoclib::grid::Grid;
use aoclib::vec2::{Direction, Vector2, DIRECTIONS};
use fxhash::FxHashSet;

struct Input {
    grid: Grid<char>,
    starting_pos: Vector2<usize>,
    target_pos: Vector2<usize>
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Input {
    let mut grid = Grid::parse(input, |c| c);
    let starting_pos = grid.find_first_occurance(&'S').unwrap();
    let target_pos = grid.find_first_occurance(&'E').unwrap();
    grid[&starting_pos] = '.';

    Input {
        grid, starting_pos, target_pos
    }
}

#[aoc(day16, part1)]
fn part1(input: &Input) -> usize {
    let mut grid = input.grid.clone();
    let mut queue: Vec<(Vector2<usize>, usize, Direction)> = Vec::new();
    let mut visited: FxHashSet<Vector2<usize>> = FxHashSet::default();
    queue.push((input.starting_pos, 0, Direction::Right));

    while !queue.is_empty() {
        queue.sort_by_key(|v| v.1);
        assert!(queue.is_sorted_by_key(|v| v.1));
        let (v, cost, prev_dir) = queue.swap_remove(0);
        visited.insert(v);
        grid[&v] = 'O';
        //dbg!(&grid);

        for dir in DIRECTIONS {
            if dir.inv() == prev_dir {
                // don't walk back
                continue;
            }

            let rotate = dir != prev_dir;
            let new_cost = cost + if !rotate { 1 } else { 1001 };
            let i = dir + v;

            if input.grid[&i] == '.' && !visited.contains(&i) {
                // we can make the step
                queue.push((i, new_cost, dir));
            }

            if input.grid[&i] == 'E' {
                return new_cost;
            }
        }
    }

    0
}

#[aoc(day16, part2)]
fn part2(input: &Input) -> usize {
    let mut visited = FxHashSet::default();
    visited.insert(input.starting_pos);
    let sets = walk_maze(&input.grid, 0, input.starting_pos, Direction::Right, visited);
    //dbg!(&sets);
    let minimum_cost = sets.iter().min_by_key(|(_, cost)| cost).unwrap().1;
    sets.into_iter().filter(|(_, cost)| *cost == minimum_cost).fold(FxHashSet::default(), |mut acc: FxHashSet<Vector2<usize>>, v| {
        acc.extend(v.0.iter());
        {
            let mut grid = input.grid.clone();
            for i in v.0.iter() {
                grid[i] = 'O';
            }
            dbg!(grid);
        }
        acc
    }).len()
}

fn walk_maze(grid: &Grid<char>, cost: usize, position: Vector2<usize>, looking_dir: Direction, mut visited: FxHashSet<Vector2<usize>>) -> Vec<(FxHashSet<Vector2<usize>>, usize)> {
    if cost > 110000 {
        return vec![];
    }

    let mut vec = Vec::new();
    //if visited.len() % 1000 == 0 {
    //    dbg!(visited.len());
    //}

    for dir in DIRECTIONS {
        if dir.inv() == looking_dir {
            // don't walk back
            continue;
        }

        let rotate = dir != looking_dir;
        let new_cost = cost + if !rotate { 1 } else { 1001 };
        let i = dir + position;

        if grid[&i] == '.' && !visited.contains(&i) {
            // we can make the step
            let mut vis = visited.clone();
            vis.insert(i);
            let set = walk_maze(grid, new_cost, i, dir, vis);
            vec.extend(set);
        }

        if grid[&i] == 'E' {
            visited.insert(i);
            return vec![(visited, new_cost)];
        }
    }

    vec
}

aoc_test!(test_day16_sample1, "../samples/day16-1.txt", 7036, 45);
aoc_test!(test_day16_sample2, "../samples/day16-2.txt", 11048, 64);
aoc_test!(test_day16, "../input/2024/day16.txt", 109516);