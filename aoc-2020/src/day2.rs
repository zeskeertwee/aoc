use reformation::Reformation;

#[derive(Reformation, Debug)]
#[reformation(r"{min}-{max} {chr}: {password}")]
pub struct Input {
    min: usize,
    max: usize,
    chr: char,
    password: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|line| Input::parse(line).unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Input]) -> usize {
    input.iter().map(|i| {
        let count = i.password.chars().map(|c| if c == i.chr { 1 } else { 0 }).fold(0, |acc, x| acc + x);
        if count >= i.min && count <= i.max {
            1
        } else {
            0
        }
    }).sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Input]) -> usize {
    input.iter().map(|i| {
        let chars: Vec<char> = i.password.chars().collect();
        if chars[i.min - 1] == i.chr && chars[i.max - 1] != i.chr {
            1
        } else if chars[i.min - 1] != i.chr && chars[i.max - 1] == i.chr {
            1
        } else {
            0
        }
    }).sum()
}