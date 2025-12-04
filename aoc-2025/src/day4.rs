use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use aoclib::grid::Grid;
use aoclib::vec2::Vector2;

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Grid<char> {
    Grid::parse(input, |c| c)
}

#[aoc(day4, part1)]
fn part1(input: &Grid<char>) -> usize {
    input.iter_squares()
        .filter(|(v, pos)| {
            **v == '@' &&
            input.adjacent_neighbour_squares(pos).iter().filter(|c| input[c] == '@').count() < 4
        })
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &Grid<char>) -> usize {
    let mut removed = 0;
    let mut grid = input.clone();

    loop {
        let p: Vec<Vector2<usize>> = grid.iter_squares()
            .filter(|(v, pos)| {
                **v == '@' &&
                    grid.adjacent_neighbour_squares(pos).iter().filter(|c| grid[c] == '@').count() < 4
            })
            .map(|v| v.1)
            .collect();

        if p.len() == 0 {
            break;
        }

        removed += p.len();

        for i in p {
            grid[&i] = '.';
        }
    }

    removed
}

aoc_test!(test_day4, "../input/2025/day4.txt", 1493, 9194);