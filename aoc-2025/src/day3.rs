use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use rayon::prelude::*;

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|l| l.chars().map(|c| c as u8 - '0' as u8).collect())
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<u8>]) -> usize {
    input.iter()
        .map(|n| highest_number_n_digits::<2>(n))
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<u8>]) -> usize {
    input.par_iter()
        .map(|n| highest_number_n_digits::<12>(n))
        .sum()
}

/// returns the highest number possible from the given digits, using N digits in the order they appear
fn highest_number_n_digits<const N: usize>(digits: &Vec<u8>) -> usize {
    let mut highest_num: [u8; N] = [0; N];

    'outer: for (idx, i) in digits.iter().enumerate() {
        for j in 0..N {
            if i > &highest_num[j] && idx < digits.len() - N + 1 + j {
                highest_num[j] = *i;

                for k in (j+1)..N {
                    highest_num[k] = 0;
                }
                continue 'outer;
            }
        }
    }

    let mut n = 0;
    for i in 0..N {
        n += highest_num[N - i - 1] as usize * 10_usize.pow(i as _);
    }

    n
}

aoc_test!(test_day3, "../input/2025/day3.txt", 17435, 172886048065379);