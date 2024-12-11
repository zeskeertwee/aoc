use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap;

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Vec<u64> {
    input.split_whitespace().map(|v| v.parse::<u64>().unwrap()).collect()
}

#[aoc(day11, part1)]
fn part1(input: &[u64]) -> u64 {
    blink(input, 25)
}

#[aoc(day11, part2)]
fn part2(input: &[u64]) -> u64 {
    blink(input, 75)
}

fn blink(input: &[u64], times: usize) -> u64 {
    let mut map: FxHashMap<u64, u64> = FxHashMap::default();

    input.iter().for_each(|v| {
        *map.entry(*v).or_insert(0) += 1;
    });

    for _ in 0..times {
        let mut new_map = FxHashMap::default();
        for (k, v) in map.iter() {
            if *k == 0 {
                *new_map.entry(1).or_insert(0) += v;
            } else if is_even(digits(*k)) {
                let (a, b) = split_num(*k);
                *new_map.entry(a).or_insert(0) += v;
                *new_map.entry(b).or_insert(0) += v;
            } else {
                *new_map.entry(*k * 2024).or_insert(0) += v;
            }
        }
        map = new_map;
    }

    map.values().sum()
}

fn digits(n: u64) -> u64 {
    n.ilog10() as u64 + 1
}

fn is_even(n: u64) -> bool {
    n & 1 == 0
}

fn split_num(n: u64) -> (u64, u64) {
    let factor = 10u64.pow(digits(n) as u32 / 2);

    (n / factor, n % factor)
}