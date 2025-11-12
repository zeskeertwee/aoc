use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> String {
    input.to_string()
}

#[aoc(day1, part1)]
fn part1(input: &str) -> i64 {
    input.chars()
        .fold(0, |acc, c| {
            match c {
                '(' => acc + 1,
                ')' => acc - 1,
                _ => panic!()
            }
        })
}

#[aoc(day1, part2)]
fn part2(input: &str) -> usize {
    let mut floor = 0;

    for (idx, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!()
        };

        if floor == -1 {
            return idx + 1;
        }
    }

    return 0;
}

aoc_test!(test_day1, "../input/2015/day1.txt", 232, 1783);