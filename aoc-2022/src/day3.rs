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
fn part1(input: &[(HashSet<char>, HashSet<char>)]) -> u32 {
    input.iter().map(|b| {
        char_priority(b.0.intersection(&b.1).next().unwrap().to_owned())
    }).map(|v| v as u32).sum()
}

#[aoc(day3, part2)]
fn part2(input: &[(HashSet<char>, HashSet<char>)]) -> u32 {
    input.chunks(3).map(|b| {
        // iterate over every 3 lines and put all chars of that line into a hashset
        let mut sets = Vec::new();
        for i in b {
            let set: HashSet<char> = HashSet::from_iter(i.0.union(&i.1).map(|v| *v).collect::<Vec<char>>());
            sets.push(set);
        }

        // take the intersection of the three hashsets and return the priority value
        char_priority(*sets[1..].iter().fold(sets[0].clone(), |mut acc, set| {
            acc.retain(|v| set.contains(v));
            acc
        }).iter().next().unwrap())
    }).map(|v| v as u32).sum()
}

fn char_priority(c: char) -> u8 {
    let c = c as u8;
    if c.is_ascii_lowercase() {
        c - ('a' as u8 - 1)
    } else {
        (c - ('A' as u8 - 1)) + 26
    }
}