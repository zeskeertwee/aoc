use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .map(|l| l.split('x').map(|v| v.parse().unwrap()).collect())
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Vec<u32>]) -> u32 {
    input.iter()
        .map(|package| {
            let sides = [package[0] * package[1], package[0] * package[2], package[1] * package[2]];
            2 * sides.iter().sum::<u32>() + sides.iter().min().unwrap()
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Vec<u32>]) -> u32 {
    input.iter()
        .map(|package| {
            let wrapping_ribbon = 2 * (package.iter().sum::<u32>() - package.iter().max().unwrap());
            let volume = package[0] * package[1] * package[2];

            wrapping_ribbon + volume
        })
        .sum::<u32>()
}

aoc_test!(test_day2, "../input/2015/day2.txt", 1586300, 3737498);