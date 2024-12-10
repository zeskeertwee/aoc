use fxhash::FxHashMap;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> (Vec<u32>, Vec<u32>) {
    input.lines()
        .map(|v| { v.split_ascii_whitespace().map(|v| v.parse().unwrap()).collect() })
        .fold((Vec::new(), Vec::new()), |(mut left, mut right), i: Vec<u32>| {
            left.push(i[0]);
            right.push(i[1]);
            (left, right)
        })
}

#[aoc(day1, part1)]
fn part1(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let (mut left, mut right) = input.clone();

    left.sort();
    right.sort();

    left.iter().zip(right).map(|(a, b)| a.abs_diff(b)).sum()
}

#[aoc(day1, part2)]
fn part2(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut map = FxHashMap::default();
    for i in input.1.iter() {
        *map.entry(i).or_insert(0) += 1;
    }

    input.0.iter().map(|v| map.get(v).unwrap_or(&0) * v).sum()
}