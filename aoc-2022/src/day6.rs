use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn parse_input_day6(input: &str) -> Vec<char> {
    input.chars().collect()
}

#[aoc(day6, part1)]
fn part1(input: &[char]) -> u32 {
    for (idx, window) in input.windows(4).enumerate() {
        if has_all_unique_chars(window) {
            // +4 for 0-based offset and character offset as window is 4 chars long and we need the
            // index of the last character
            return idx as u32 + 4;
        }
    }

    0
}

#[aoc(day6, part2)]
fn part2(input: &[char]) -> u32 {
    for (idx, window) in input.windows(14).enumerate() {
        if has_all_unique_chars(window) {
            // again, same offset, now just with a bigger window size
            return idx as u32 + 14;
        }
    }

    0
}

fn has_all_unique_chars(c: &[char]) -> bool {
    for i in 0..c.len() {
        for j in (i + 1)..c.len() {
            if c[i] == c[j] {
                return false;
            }
        }
    }

    true
}