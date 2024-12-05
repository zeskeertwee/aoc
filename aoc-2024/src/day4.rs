use std::cmp::{max, min};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse_input_day4(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[aoc(day4, part1)]
fn part1(grid: &[Vec<char>]) -> u32 {
    printgrid(grid);
    let a = find_xmas(grid);
    let b = find_xmas(&rot_grid(grid));

    // ignore first row to prevent counting diagonals in the middle of the grid double
    let c = find_xmas(&rot_grid(&diag_grid(grid, true))[1..]);
    let d = find_xmas(&rot_grid(&diag_grid(grid, false))[1..]);

    let e = find_xmas(&rot_grid(&diag_grid(&rot_grid(grid), true)));
    let f = find_xmas(&rot_grid(&diag_grid(&rot_grid(grid), false)));
    dbg!(a, b, c, d, e, f);

    a + b + c + d + e + f
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
            i += pchar.len();
        }

        i += 1;
    }

    count
}

fn find_xmas(grid: &[Vec<char>]) -> u32 {
    println!();
    grid.iter()
        .map(|row| find_in_vector(row, "XMAS") + find_in_vector(row, "SAMX"))
        .map(|v| { dbg!(v); v })
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
    let mut result: Vec<Vec<char>> = vec![Vec::new(); grid.len()];

    for y in 0..grid.len() {
        if left {
            result[y].extend(&grid[y][y..]);
        } else {
            result[y].extend(&grid[y][grid[y].len() - y - 1..]);
        }

        for i in result[y].len()..grid[y].len() {
            result[y].push('.');
        }
    }

    result
}