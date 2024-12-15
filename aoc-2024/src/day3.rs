use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;

#[aoc_generator(day3)]
fn parse_input(input: &str) -> String {
    input.to_string()
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u64 {
    let mut count = 0;
    let mut index = 0;

    while index < input.len() {
        match find_next_mul(&input[index..]) {
            (0, 0) => (),
            (value, offset) => {
                index += offset;
                count += value;
                continue;
            }
        }

        break;
    }

    count
}

// returns mul value, offset at which it was found
fn find_next_mul(string: &str) -> (u64, usize) {
    let mut count = 0;
    let mut index = 0;

    if let Some(i) = string.find("mul(") {
        let arg1 = string[i + 4..].chars().take_while(|c| c.is_numeric()).collect::<String>();
        let arg2 = string[i + 4 + arg1.len() + 1..].chars().take_while(|c| c.is_numeric()).collect::<String>();

        count = if !arg1.is_empty() && !arg2.is_empty() && string.chars().nth(i + 4 + arg1.len() + 1 + arg2.len()).unwrap() == ')' {
            arg1.parse::<u64>().unwrap() * arg2.parse::<u64>().unwrap()
        } else { 0 };

        index += i + arg1.len() + 4 + arg2.len();
    }

    (count, index)
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u64 {
    let mut on = true;
    let mut count = 0;
    let mut index = 0;

    while index < input.len() {
        if !on {
            // if off, find next point to turn on at
            match input[index..].find("do()") {
                Some(offset) => {
                    on = true;
                    index += offset + 4;
                    continue;
                },
                None => {
                    return count;
                }
            }
        }

        let next_off = input[index..].find("don't()");

        match find_next_mul(&input[index..]) {
            (0, 0) => break,
            (value, offset) => {
                if let Some(next_off) = next_off && index + offset > index + next_off {
                    // if mul beyond next off, turn off and jump to next off.
                    on = false;
                    index += next_off + 4;
                } else {
                    // if mul is before next off, add it and jump to it
                    index += offset;
                    count += value;
                }
            }
        }
    }

    count

}

aoc_test!(test_day3_sample, "../samples/day3.txt", 161, 48);
aoc_test!(test_day3, "../input/2024/day3.txt", 155955228, 100189366);