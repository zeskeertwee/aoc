#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    input.iter().enumerate().map(|(idx, val)| {
        input.iter().enumerate().map(|(idx2, val2)| {
            if idx != idx2 && val + val2 == 2020 {
                return Some(val * val2);
            }
            None
        }).find(|v| v.is_some()).unwrap_or(None)
    }).find(|v| v.is_some()).unwrap().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    input.iter().enumerate().map(|(idx, val)| {
        input.iter().enumerate().map(|(idx2, val2)| {
            input.iter().enumerate().map(|(idx3, val3)| {
                if idx != idx2 && idx2 != idx3 && idx != idx3 && val + val2 + val3 == 2020 {
                    return Some(val * val2 * val3);
                }
                None
            }).find(|v| v.is_some()).unwrap_or(None)
        }).find(|v| v.is_some()).unwrap_or(None)
    }).find(|v| v.is_some()).unwrap().unwrap()
}