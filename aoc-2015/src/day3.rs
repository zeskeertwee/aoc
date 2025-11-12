use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use aoclib::vec2::{Direction, Vector2};
use fxhash::FxHashSet;

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<Direction> {
    input.chars().map(|c| match c {
        '>' => Direction::Right,
        '^' => Direction::Up,
        '<' => Direction::Left,
        'v' => Direction::Down,
        _ => panic!()
    })
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Direction]) -> usize {
    let mut visited: FxHashSet<Vector2<isize>> = FxHashSet::default();
    visited.insert(Vector2::new(0, 0));

    input.iter()
        .fold(Vector2::new(0, 0), |pos, dir| {
            let new_pos = *dir + pos;
            visited.insert(new_pos);
            new_pos
        });

    visited.len()
}

#[aoc(day3, part2)]
fn part2(input: &[Direction]) -> usize {
    let mut visited: FxHashSet<Vector2<isize>> = FxHashSet::default();
    visited.insert(Vector2::new(0, 0));

    input.iter()
        .array_chunks::<2>()
        .fold((Vector2::new(0, 0), Vector2::new(0, 0)), |pos, dir| {
            let new_pos = (*dir[0] + pos.0, *dir[1] + pos.1);
            visited.insert(new_pos.0);
            visited.insert(new_pos.1);
            new_pos
        });

    visited.len()
}

aoc_test!(test_day3, "../input/2015/day3.txt", 2572, 2631);