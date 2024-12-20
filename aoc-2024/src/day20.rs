use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use aoclib::grid::Grid;
use aoclib::vec2::Vector2;
use rayon::prelude::*;

struct Input {
    grid: Grid<char>,
    start: Vector2<usize>,
    target: Vector2<usize>,
}

#[aoc_generator(day20)]
fn parse_input(input: &str) -> Input {
    let mut grid = Grid::parse(input, |c| c);
    let start = grid.find_first_occurance(&'S').unwrap();
    let target = grid.find_first_occurance(&'E').unwrap();

    grid[&start] = '.';
    grid[&target] = '.';

    Input {
        grid, start, target
    }
}

#[aoc(day20, part1)]
fn part1(input: &Input) -> usize {
    find_cheats(input, 2)
}

#[aoc(day20, part2)]
fn part2(input: &Input) -> usize {
    find_cheats(input, 20)
}

fn find_cheats(input: &Input, max_cheat_len: usize) -> usize {
    let costs = input.grid
        .bfs_find_node_costs(input.start, input.target, &'.')
        .unwrap();

    costs.iter_squares()
        .par_bridge()
        .map(|(cost, v)| {
            let mut val: usize = 0;
            for i in 2..=max_cheat_len {
                for cheat_pos in costs.neighbour_squares_radius(&v, i) {
                    if costs[&cheat_pos] == usize::MAX {
                        continue;
                    }

                    let saved_cost = (*cost as i64 - costs[&cheat_pos] as i64) - i as i64;
                    if saved_cost >= 100 {
                        val += 1;
                    }
                }
            }

            val
    }).sum()
}

fn print_grid(grid: &Grid<usize>) {
    dbg!(grid);
    for y in 0..grid.height {
        for x in 0..grid.width {
            let v = grid[&Vector2::new(x, y)];
            if v < usize::MAX {
                print!(" {} ", v);
            } else {
                print!(" # ");
            }
        }
        println!();
    }
}

aoc_test!(test_day20, "../input/2024/day20.txt", 1358, 1005856);