use std::collections::BinaryHeap;
use std::num::ParseIntError;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Result<Vec<Vec<i32>>, ParseIntError> {
    input.split("\n\n").map(|l| l.lines().map(|l| l.parse()).collect()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[Vec<i32>]) -> i32 {
    input.iter().map(|s| s.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
fn part2(input: &[Vec<i32>]) -> i32 {
    let data: BinaryHeap<i32> = input.iter().map(|s| s.iter().sum()).collect();
    data.into_sorted_vec().iter().rev().take(3).sum()
}