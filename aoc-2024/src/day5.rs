use std::iter::Iterator;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Input {
    rules: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>
}

#[aoc_generator(day5, part1)]
fn parse_day5(input: &str) -> Input {
    let mut split: Vec<&str> = input.split("\n\n").collect();

    Input {
        rules: split[0].lines().map(|v| {
            let split: Vec<&str> = v.split('|').collect();
            (split[0].parse().unwrap(), split[1].parse().unwrap())
        }).collect(),
        updates: split[1].lines().map(|v| {
            v.split(',').map(|i| i.parse().unwrap()).collect()
        }).collect()
    }
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> u32 {
    dbg!(&input);
    0
}