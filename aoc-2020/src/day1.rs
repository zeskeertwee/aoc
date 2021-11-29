#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|line| line.parse::<u16>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u16]) -> u32 {
    for (idx, val) in input.iter().enumerate().skip(1) {
        dbg!(input[idx - 1]);
        dbg!(val + input[idx - 1]);
        if val + input[idx - 1] == 2020 {
            return *val as u32 * input[idx - 1] as u32;
        }
    }

    panic!("Couldn't find answer!");
}