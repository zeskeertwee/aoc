use std::collections::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse_input_day3(input: &str) -> Vec<(HashSet<char>, HashSet<char>)> {
    input.lines().map(|l| {
        let (left, right) = l.split_at(l.len() / 2);
        let (mut lset, mut rset) = (HashSet::new(), HashSet::new());
        lset.extend(left.chars());
        rset.extend(right.chars());
        (lset, rset)
    }).collect()
}

#[aoc(day3, part1)]
fn part1(input: &[(HashSet<char>, HashSet<char>)]) -> i32 {
    input.iter().map(|b| {
        let c = b.0.intersection(&b.1).next().unwrap().to_owned() as u8;
        if c.is_ascii_lowercase() {
            c - ('b' as u8)
        } else {
            (c - ('B' as u8)) + 26
        }
    }).map(|v| v as i32).sum()
}