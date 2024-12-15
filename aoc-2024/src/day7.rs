use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use rayon::prelude::*;

pub struct Equation {
    test: u64,
    numbers: Vec<u64>,
}

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Vec<Equation> {
    input.lines().map(|l| {
        let split: Vec<&str> = l.split(':').collect();

        Equation {
            test: split[0].parse().unwrap(),
            numbers: split[1].trim().split(' ').map(|n| n.parse().unwrap()).collect()
        }
    }).collect()
}

#[aoc(day7, part1)]
fn part1(input: &[Equation]) -> u64 {
    input.par_iter().map(|eq| {
        try_operators::<false>(eq.test, eq.numbers[0], &eq.numbers[1..]) as u64 * eq.test
    }).sum()
}

#[aoc(day7, part2)]
fn part2(input: &[Equation]) -> u64 {
    input.par_iter().map(|eq| {
        try_operators::<true>(eq.test, eq.numbers[0], &eq.numbers[1..]) as u64 * eq.test
    }).sum()
}

fn try_operators<const PART2: bool>(test: u64, n0: u64, numbers: &[u64]) -> bool {
    if numbers.len() == 0 {
        return n0 == test;
    }

    if numbers.len() == 1 {
        if n0 + numbers[0] == test || n0 * numbers[0] == test {
            return true;
        } else if !PART2 {
            return false;
        }
        
        return concat(n0, numbers[0]) == test;
    }

    if !PART2 {
        try_operators::<PART2>(test, n0 + numbers[0], &numbers[1..])
        || try_operators::<PART2>(test, n0 * numbers[0], &numbers[1..])
    } else {
        try_operators::<PART2>(test, n0 + numbers[0], &numbers[1..])
        || try_operators::<PART2>(test, n0 * numbers[0], &numbers[1..])
        || try_operators::<PART2>(test, concat(n0, numbers[0]), &numbers[1..])
    }
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

aoc_test!(test_day7_sample, "../samples/day7.txt", 3749, 11387);
aoc_test!(test_day7, "../input/2024/day7.txt", 1298103531759, 140575048428831);