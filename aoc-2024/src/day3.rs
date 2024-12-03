use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse_input_day3(input: &str) -> String {
    input.to_string()
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u64 {
    let mut count = 0;
    let mut index = 0;

    while index < input.len() {
        if let Some(i) = input[index..].find("mul(") {
            let arg1 = input[index + i + 4..].chars().take_while(|c| c.is_numeric()).collect::<String>();
            let arg2 = input[index + i + 4 + arg1.len() + 1..].chars().take_while(|c| c.is_numeric()).collect::<String>();
            if !arg1.is_empty() && !arg2.is_empty() && input.chars().nth(index + i + 4 + arg1.len() + 1 + arg2.len()).unwrap() == ')' {
                count += arg1.parse::<u64>().unwrap() * arg2.parse::<u64>().unwrap();
            }

            index += i + arg1.len() + 4 + arg2.len();
            continue;
        }

        break;
    }

    count
}