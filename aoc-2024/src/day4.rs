use std::cmp::{max, min};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse_input_day4(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[aoc(day4, part1)]
fn part1(grid: &[Vec<char>]) -> u32 {
    find_xmas(grid)
    + find_xmas(&rot_grid(grid))
    + find_xmas(&rot_grid(&diag_grid(grid, true)))
    + find_xmas(&rot_grid(&diag_grid(grid, false)))
}

#[aoc(day4, part2)]
fn part2(grid: &[Vec<char>]) -> u32 {
    let mut count = 0;

    // iterate over whole grid, except outer sides because center of X can never be on the outer side
    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            if grid[y][x] == 'A' {
                // can be the center of an X
                let top_left = grid[y-1][x-1];
                let top_right = grid[y-1][x+1];
                let bot_left = grid[y+1][x-1];
                let bot_right = grid[y+1][x+1];

                if ((top_left == 'M' && bot_right == 'S') || (top_left == 'S' && bot_right == 'M'))
                    && ((top_right == 'M' && bot_left == 'S') || (top_right == 'S' && bot_left == 'M')) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn printgrid(grid: &[Vec<char>]) {
    println!("{}x{}", grid[0].len(), grid.len());
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn find_in_vector(vec: &Vec<char>, pat: &str) -> u32 {
    let pchar: Vec<char> = pat.chars().collect();
    let mut count = 0;
    let mut i = 0;

    'outer: while i < vec.len() {
        if vec[i] == pchar[0] {
            if vec.len() - i < pchar.len() {
                // too short to fit pattern
                return count;
            }
            for j in 1..pchar.len() {
                if vec[i + j] != pchar[j] {
                    i += j;
                    continue 'outer;
                }
            }

            // pattern match
            count += 1;
            i += pchar.len() - 1;
        }

        i += 1;
    }

    count
}

fn find_xmas(grid: &[Vec<char>]) -> u32 {
    grid.iter()
        .map(|row| find_in_vector(row, "XMAS") + find_in_vector(row, "SAMX"))
        .sum()
}

fn rot_grid(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    // the amount of rows in the new grid needs to be the width of the old grid
    let mut result: Vec<Vec<char>> = vec![Vec::new(); grid[0].len()];

    for y in (0..grid.len()).rev() {
        for x in 0..grid[y].len() {
            result[x].push(grid[y][x]);
        }
    }

    result
}

fn diag_grid(grid: &[Vec<char>], left: bool) -> Vec<Vec<char>> {
    let width = (grid[0].len() * 2) - 1; // the width of the diag. grid will be 2* the width of the original - 1 at the top/bottom
    let height = grid.len();
    let mut result: Vec<Vec<char>> = vec![Vec::new(); height];

    for y in 0..grid.len() {
        if left {
            result[y].extend(&vec!['.'; y]); // pad left side
            result[y].extend(&grid[y]); // add row, right side will pad at the end
        } else {
            result[y].extend(&vec!['.'; height - y - 1]);
            result[y].extend(&grid[y]);
        }
        for i in result[y].len()..width {
            result[y].push('.');
        }
    }

    result
}