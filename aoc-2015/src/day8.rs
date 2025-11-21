use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<String> {
    input.lines()
        .map(|l| l.to_string())
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &[String]) -> usize {
    input.iter()
        .map(|l| {
            let mut num_bytes = 0;
            let chars: Vec<char> = l.chars().collect();
            let num_char = chars.len();

            let mut i = 0;
            while i < chars.len() - 1 {
                if i == 0{
                    // starting "
                    i += 1;
                    continue;
                }

                let c = chars[i];
                match c {
                    '\\' => if chars[i+1] == '\\' || chars[i+1] == '"' {
                        num_bytes += 1;
                        i += 1;
                    } else if chars[i+1] == 'x' {
                        num_bytes += 1;
                        i += 3;
                    },
                    _ => num_bytes += 1
                }

                i += 1;
            }

            num_char - num_bytes
        })
        .sum()
}

#[aoc(day8, part2)]
fn part2(input: &[String]) -> usize {
    input.iter()
        .map(|l| {
            let chars: Vec<char> = l.chars().collect();
            let num_char = chars.len();
            let mut encoded_num_char = 4 + num_char;

            let mut i = 0;
            while i < chars.len() - 1 {
                if i == 0{
                    // starting "
                    i += 1;
                    continue;
                }

                let c = chars[i];
                match c {
                    '\\' => if chars[i+1] == '\\' || chars[i+1] == '"' {
                        encoded_num_char += 2;
                        i += 1;
                    } else if chars[i+1] == 'x' {
                        encoded_num_char += 1;
                        i += 3;
                    },
                    _ => (),
                }

                i += 1;
            }

            encoded_num_char - num_char
        })
        .sum()
}

aoc_test!(test_day8, "../input/2015/day8.txt", 1371, 2117);