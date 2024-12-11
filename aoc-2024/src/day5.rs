use std::cmp::Ordering;
use std::iter::Iterator;
use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;

#[derive(Debug)]
struct Input {
    rules: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>
}

#[aoc_generator(day5)]
fn parse_day5(input: &str) -> Input {
    let split: Vec<&str> = input.split("\n\n").collect();

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
    input.updates.par_iter().map(|v| {
        if is_correct_order(&input.rules, v) {
            v[v.len() / 2]
        } else {
            0
        }
    }).sum()
}

fn is_correct_order(rules: &Vec<(u32, u32)>, update: &[u32]) -> bool {
    for (first, second) in rules {
        let mut found_second = false;

        for p in update {
            if p == first {
                if found_second == true {
                    // first after second
                    return false;
                }
            }

            if p == second {
                found_second = true;
                // either we found it after first, or we didn't find first at all
                // in both cases, good
            }
        }
    }

    true
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> u32 {
    let mut updates = input.updates.clone();
    updates.par_iter_mut()
        .filter(|v| !is_correct_order(&input.rules, v))
        .map(|v| {
            v.par_sort_by(|a, b| {
                match is_correct_order(&input.rules, &vec![*a, *b]) {
                    true => Ordering::Less,
                    false => Ordering::Greater
                }
            });

            v[v.len()/2]
        }).sum()
}