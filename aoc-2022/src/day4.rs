use std::ops::RangeInclusive;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse_input_day4(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    input.lines().map(|l| {
        let split: Vec<&str> = l.split(',').collect();

        assert!(split.len() == 2);

        let ranges: Vec<RangeInclusive<u32>> = split.iter().map(|v| {
            let values: Vec<u32> = v.split('-').map(|v| v.parse().unwrap()).collect();
            RangeInclusive::new(values[0], values[1])
        }).collect();
        (ranges[0].clone(), ranges[1].clone())
    }).collect()
}

#[aoc(day4, part1)]
fn part1(input: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> u32 {
    input
        .iter()
        .map(|v| range_falls_inside_other(&v.0, &v.1) as u32)
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> u32 {
    input
        .iter()
        .map(|v| range_overlaps_other(&v.0, &v.1) as u32)
        .sum()
}
// checks wether or not one of the two ranges completely fits inside the other
fn range_falls_inside_other(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    (a.start() <= b.start() && a.end() >= b.end()) || // b inside of a
        (b.start() <= a.start() && b.end() >= a.end()) // a inside of b
}

// checks wether or not the ranges overlap
fn range_overlaps_other(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    b.start() <= a.end() && !(b.end() < a.start())
}