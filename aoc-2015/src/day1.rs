use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<i32> {
    input.chars().map(|c| if c == '(' { 1 } else { -1 }).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[i32]) -> i32 {
    input.iter()
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &[i32]) -> usize {
    let mut floor = 0;

    input.iter()
        .position(|v| {
            floor += v;
            floor < 0
        })
        .map(|f| f + 1)
        .unwrap()
}

aoc_test!(test_day1, "../input/2015/day1.txt", 232, 1783);