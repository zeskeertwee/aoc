use std::ops::RangeInclusive;
use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use rayon::prelude::*;

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<RangeInclusive<u64>> {
    input.split(',')
        .map(|l| {
            let range: Vec<u64> = l.split('-').map(|v| v.parse().unwrap()).collect();
            RangeInclusive::new(range[0], range[1])
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[RangeInclusive<u64>]) -> u64 {
    input.par_iter()
        .map(|r| r.to_owned().into_iter()
            .filter(|v| numlength(v) % 2 == 0) // cannot have a sequence with all digits repeating twice if the length is not divisible by two
            .filter(|v| {
                let l = numlength(v);
                let left = v / 10_u64.pow(l/2);
                let right = v - (left * 10_u64.pow(l/2));
                left == right
            })
            .sum::<u64>())
        .sum::<u64>()
}

#[aoc(day2, part2)]
fn part2(input: &[RangeInclusive<u64>]) -> u64 {
    dbg!(integer_factorize(24));

    input.par_iter()
        .map(|r| r.to_owned().into_iter()
            .filter(|v| {
                integer_factorize(numlength(v)).iter().any(|a| vector_single_unique_element(&extract_digits(v, *a)))
            })
            .sum::<u64>())
        .sum::<u64>()
}

fn numlength(n: &u64) -> u32 {
    n.ilog10() + 1
}

fn integer_factorize(n: u32) -> Vec<u32> {
    let mut v = vec![1];

    for i in 2..=n.isqrt() {
        if n % i == 0 {
            v.push(n / i);
            v.push(i);
        }
    }

    v
}

fn extract_digits(n: &u64, l: u32) -> Vec<u64> {
    let len = numlength(n);
    assert!(len % l == 0);

    let n_elements = len / l;
    let mut v: Vec<u64> = vec![];

    for i in (0..n_elements).rev() {
        let mut element = n / 10_u64.pow(l * i);

        for (j, val) in v.iter().enumerate() {
            let shift = (v.len() - j) as u32;
            element -= val * 10_u64.pow(l * shift);
        }

        v.push(element);
    }

    v
}

fn vector_single_unique_element(v: &[u64]) -> bool {
    for i in 1..v.len() {
        if v[0] != v[i] {
            return false;
        }
    }

    true
}

aoc_test!(test_day2, "../input/2025/day2.txt", 38310256125);