use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::grid::Grid;
use aoclib::vec2::Vector2;

#[aoc_generator(day21)]
fn parse_input(input: &str) -> Vec<String> {
    input.lines().collect()
}

#[aoc(day21, part1)]
fn part1(input: &[String]) -> usize {
    0
}

const KEYPAD: Grid<char> = Grid {
    grid: vec!['7', '8', '9', '4', '5', '6', '1', '2', '3', '#', '0', 'A'],
    width: 3,
    height: 4,
};

const EMPTY_GRID: Grid<char> = Grid {
    grid: vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#', ',', '.'],
    width: 3,
    height: 4,
};

fn keypad_path(from: char, to: char) {
    let start = KEYPAD.find_first_occurance(&from).unwrap();
    let end = KEYPAD.find_first_occurance(&to).unwrap();

    // do the y-translation last, to ensure we don't cross the empty square
    let mut path = Vec::new();
    if start.x > end.x {
        (end.x..start.x).for_each(|x| path.push(Vector2::new(x, end.y)));
    } else {
        (start.x..end.x).for_each(|x| path.push(Vector2::new(x, start.y)));
    }

    if start.y > end.y {
        (end.y..start.y).for_each(|x| path.push(Vector2::new(x, end.x)));
    } else {
        (start.y..end.y).for_each(|x| path.push(Vector2::new(x, end.x)));
    }
}