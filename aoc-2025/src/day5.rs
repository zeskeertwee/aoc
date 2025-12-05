use std::ops::RangeInclusive;
use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use rayon::prelude::*;

struct Input {
    fresh_id_ranges: Vec<RangeInclusive<usize>>,
    ingredient_ids: Vec<usize>
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Input {
    let split: Vec<&str> = input.split("\n\n").collect();
    Input {
        fresh_id_ranges: split[0].lines().map(|l| {
            let range: Vec<usize> = l.split('-').map(|v| v.parse().unwrap()).collect();
            RangeInclusive::new(range[0], range[1])
        }).collect(),
        ingredient_ids: split[1].lines().map(|l| l.parse().unwrap()).collect()
    }
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> usize {
    input.ingredient_ids.par_iter()
        .filter(|id| input.fresh_id_ranges.iter().any(|r| r.contains(id)))
        .count()
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> usize {
    let mut n: Vec<usize> = input.fresh_id_ranges.iter()
        .flat_map(|r| [*r.start(), *r.end() + 1])
        .collect();
    n.sort();

    n.windows(2).map(|i| {
        if input.fresh_id_ranges.iter().any(|r| r.contains(&i[0])) {
            i[1] - i[0]
        } else {
            0
        }
    }).sum()
}

aoc_test!(test_day5, "../input/2025/day5.txt", 726, 354226555270043);