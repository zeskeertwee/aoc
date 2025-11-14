use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use md5::{Md5, Digest};
use std::ops::Fn;
use rayon::prelude::*;

#[aoc_generator(day4)]
fn parse_input(input: &str) -> String {
    input.to_string()
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    find_hash_pattern(input, |hash| hash[0] == 0 && hash[1] == 0 && hash[2] & 0b11110000 == 0)
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    find_hash_pattern(input, |hash| hash[0] == 0 && hash[1] == 0 && hash[2] == 0)
}

fn find_hash_pattern<F: Fn([u8; 16]) -> bool + Sync>(input: &str, cond: F) -> usize {
    let mut base_hasher = Md5::new();
    base_hasher.update(input.to_string().as_bytes());

    (0..usize::MAX).into_par_iter()
        .find_first(|n| {
            let mut hasher = Md5::new();
            base_hasher.clone_into(&mut hasher);
            hasher.update(n.to_string().as_bytes());
            let result: [u8; 16] = hasher.finalize().into();

            cond(result)
        })
        .unwrap()
}

aoc_test!(test_day4, "../input/2015/day4.txt", 254575, 1038736);