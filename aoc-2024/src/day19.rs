use std::cmp::min;
use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use fxhash::{FxHashMap, FxHashSet};
use rayon::prelude::*;

#[derive(Debug)]
struct Input {
    patterns: FxHashSet<String>,
    designs: Vec<String>,
    longest_pattern: usize,
}

#[aoc_generator(day19)]
fn parse_input(input: &str) -> Input {
    let split: Vec<&str> = input.split("\n\n").collect();
    let mut longest_pattern = 0;
    let patterns = split[0].split(",").map(|v| v.trim().to_string()).fold(FxHashSet::default(), |mut acc, v| {
        if v.len() > longest_pattern {
            longest_pattern = v.len();
        }
        acc.insert(v);
        acc
    });
    let designs = split[1].lines().map(|l| l.to_owned()).collect();
    Input {
        patterns,
        longest_pattern,
        designs
    }
}

#[aoc(day19, part1)]
fn part1(input: &Input) -> usize {
    input.designs.par_iter().filter(|d| possible_combinations::<false>(&d, &input.patterns, input.longest_pattern, &mut FxHashMap::default()) > 0).count()
}

#[aoc(day19, part2)]
fn part2(input: &Input) -> usize {
    input.designs.par_iter().map(|d| possible_combinations::<true>(&d, &input.patterns, input.longest_pattern, &mut FxHashMap::default())).sum()
}

fn possible_combinations<const PART2: bool>(design: &str, patterns: &FxHashSet<String>, longest_pattern: usize, cache: &mut FxHashMap<String, usize>) -> usize {
    if design.len() == 0 {
        return 1;
    }

    if let Some(v) = cache.get(design) {
        return *v;
    }

    let mut ways = 0;

    for i in 1..min(longest_pattern, design.len()) + 1 {
        let pat = &design[0..i];
        if patterns.contains(pat) {
            let v = possible_combinations::<PART2>(&design[i..], patterns, longest_pattern, cache);
            if !PART2 {
                if v > 0 {
                    return v;
                }
            } else {
                ways += v;
            }
        }
    }

    cache.insert(design.to_owned(), ways);
    ways
}

aoc_test!(test_day19_sample, "../samples/day19.txt", 6, 16);
aoc_test!(test_day19, "../input/2024/day19.txt", 269, 758839075658876);