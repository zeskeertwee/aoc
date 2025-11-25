use aoc_runner_derive::{aoc_generator, aoc};
use aoclib::aoc_test;

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Vec<char> {
    input.chars().collect()
}

#[aoc(day11, part1)]
fn part1(input: &[char]) -> String {
    let mut password = input.to_vec();
    while !satisfies_requirements(&password) {
        password = increment_password(password, input.len() - 1);
    }

    String::from_iter(password)
}

#[aoc(day11, part2)]
fn part2(input: &[char]) -> String {
    let mut password = input.to_vec();
    while !satisfies_requirements(&password) {
        password = increment_password(password, input.len() - 1);
    }
    password = increment_password(password, input.len() - 1);
    while !satisfies_requirements(&password) {
        password = increment_password(password, input.len() - 1);
    }

    String::from_iter(password)
}

fn increment_password(mut pwd: Vec<char>, pos: usize) -> Vec<char> {
    if pwd[pos] == 'z' {
        pwd[pos] = 'a';
        pwd = increment_password(pwd, pos - 1);
    } else {
        pwd[pos] = (pwd[pos] as u8 + 1) as char;
    }

    pwd
}

fn satisfies_requirements(pwd: &[char]) -> bool {
    let pairs: Vec<usize> = pwd.windows(2)
            .enumerate()
            .filter(|&(_, c)| c[0] == c[1])
            .map(|(i, _)| i)
            .collect();

    pairs.len() >= 2 && pairs[0] + 1 < pairs[1] &&
    pwd.iter()
        .map(|c| *c as u8)
        .map_windows(|[a, b, c]| (*a + 1 == *b) && (*a + 2 == *c))
        .any(|v| v)
    && pwd.iter()
        .find(|c| **c == 'i' || **c == 'o' || **c == 'l')
        .is_none()
}

aoc_test!(test_day11, "../input/2015/day11.txt", "cqjxxyzz", "cqkaabcc");