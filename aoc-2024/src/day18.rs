use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use aoclib::grid::Grid;
use aoclib::vec2::Vector2;

#[aoc_generator(day18)]
fn parse_input(input: &str) -> Vec<Vector2<usize>> {
    input.lines().map(|l| {
        let val: Vec<usize> = l.split(',').map(|v| v.parse().unwrap()).collect();
        Vector2::new(val[0], val[1])
    }).collect()
}

#[aoc(day18, part1)]
fn part1(input: &[Vector2<usize>]) -> usize {
    let grid = if input.len() > 30 {
        let mut grid = Grid::fill('.', 71, 71);
        (0..1024).for_each(|i| grid[&input[i]] = '#');
        grid
    } else {
        let mut grid = Grid::fill('.', 7, 7);
        (0..12).for_each(|i| grid[&input[i]] = '#');
        grid
    };

    grid.bfs_find_path(Vector2::new(0, 0), Vector2::new(grid.width - 1, grid.height - 1), &'.').unwrap()
}

#[aoc(day18, part2)]
fn part2(input: &[Vector2<usize>]) -> String {
    // binary search to find the index at which there is no path anymore (starting from 1024)
    let mut step_size: i64 = (input.len() as i64 - if input.len() > 30 { 1024 } else { 12 }) / 2;
    let mut current_value = if input.len() > 30 { 1024 } else { 12 } + step_size;

    loop {
        let mut grid = if input.len() > 30 {
            Grid::fill('.', 71, 71)
        } else {
            Grid::fill('.', 7, 7)
        };

        (0..current_value).for_each(|i| grid[&input[i as usize]] = '#');

        match grid.bfs_find_path(Vector2::new(0, 0), Vector2::new(grid.width - 1, grid.height - 1), &'.') {
            None => {
                step_size = -1 * step_size.abs() / 2;
            },
            _ => {
                if step_size.abs() <= 1 {
                    let pos = input[current_value as usize];
                    return format!("{},{}", pos.x, pos.y);
                }

                step_size = step_size.abs() / 2;
            }
        }

        current_value += step_size;
    }
}

aoc_test!(test_day18_sample, "../samples/day18.txt", 22, "6,1");
aoc_test!(test_day18, "../input/2024/day18.txt", 312, "28.26");