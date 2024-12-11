use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::grid::Grid;
use aoclib::vec2::Vector2;

#[aoc_generator(day4)]
fn parse_input_day4(input: &str) -> Grid<char> {
    Grid::parse(input, |c| c)
}

#[aoc(day4, part1)]
fn part1(grid: &Grid<char>) -> u32 {
    find_xmas(grid)
    + find_xmas(&grid.rotate())
    + find_xmas(&diag_grid(grid, true).rotate())
    + find_xmas(&diag_grid(grid, false).rotate())
}

#[aoc(day4, part1, rayon)]
fn part1_rayon(grid: &Grid<char>) -> u32 {
    let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);

    rayon::scope(|s| {
        s.spawn(|_| a = find_xmas(grid));
        s.spawn(|_| b = find_xmas(&grid.rotate()));
        s.spawn(|_| c = find_xmas(&diag_grid(grid, true).rotate()));
        s.spawn(|_| d = find_xmas(&diag_grid(grid, false).rotate()));
    });

    a + b + c + d
}

#[aoc(day4, part2)]
fn part2(grid: &Grid<char>) -> u32 {
    let mut count = 0;

    // iterate over whole grid, except outer sides because center of X can never be on the outer side
    for y in 1..grid.height - 1 {
        for x in 1..grid.width - 1 {
            let pos = Vector2::new(x, y);
            if *grid.index_unchecked(&pos) == 'A' {
                // can be the center of an X
                let top_left = grid.grid[y-1][x-1];
                let top_right = grid.grid[y-1][x+1];
                let bot_left = grid.grid[y+1][x-1];
                let bot_right = grid.grid[y+1][x+1];

                if ((top_left == 'M' && bot_right == 'S') || (top_left == 'S' && bot_right == 'M'))
                    && ((top_right == 'M' && bot_left == 'S') || (top_right == 'S' && bot_left == 'M')) {
                    count += 1;
                }
            }
        }
    }

    count
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

fn find_xmas(grid: &Grid<char>) -> u32 {
    grid.grid.iter()
        .map(|row| find_in_vector(row, "XMAS") + find_in_vector(row, "SAMX"))
        .sum()
}

fn diag_grid(grid: &Grid<char>, left: bool) -> Grid<char> {
    let width = (grid.width * 2) - 1; // the width of the diag. grid will be 2* the width of the original - 1 at the top/bottom
    let height = grid.height;
    let mut result: Vec<Vec<char>> = vec![Vec::new(); height];

    for y in 0..grid.height {
        if left {
            result[y].extend(&vec!['.'; y]); // pad left side
            result[y].extend(&grid.grid[y]); // add row, right side will pad at the end
        } else {
            result[y].extend(&vec!['.'; height - y - 1]);
            result[y].extend(&grid.grid[y]);
        }
        for _ in result[y].len()..width {
            result[y].push('.');
        }
    }

    Grid::from_vec(result)
}