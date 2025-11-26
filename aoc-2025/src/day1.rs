use std::cmp::max;
use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<i16> {
    input.lines()
        .map(|l| match l.chars().nth(0).unwrap() {
            'L' => -1,
            'R' => 1,
            _ => panic!()
        } * l[1..].parse::<i16>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[i16]) -> usize {
    input.iter()
        .fold((50, 0), |(mut dial, mut password), rot| {
            dial = increment_dial(dial, rot);

            if dial == 0 {
                password += 1;
            }

            (dial, password)
        })
        .1
}

#[aoc(day1, part2)]
fn part2(input: &[i16]) -> usize {
    input.iter()
        .fold((50, 0), |(mut dial, mut password), rot| {
            // For passing 0 on the mod 100 part of the rotation
            password += if ((rot % 100) + dial < 0 || (rot % 100) + dial > 100) && dial != 0 { 1 } else { 0 };
            // For amount of total rotations
            password += max(rot.abs() / 100, 0) as usize;

            dial = increment_dial(dial, rot);
            if dial == 0 {
                password += 1;
            }

            (dial, password)
        })
        .1
}

fn increment_dial(mut dial: i16, rot: &i16) -> i16 {
    dial += rot;

    if dial >= 100 {
        dial = dial % 100;
    } else if dial < 0 {
        // It's negative, wrap it
        dial = (100 + (dial % 100)) % 100;
    }

    dial
}

aoc_test!(test_day1, "../input/2025/day1.txt", 984, 5657);