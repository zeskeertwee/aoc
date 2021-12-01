use std::mem;

#[derive(Default)]
pub struct Passport {
    byr: Option<u64>,
    iyr: Option<u64>,
    eyr: Option<u64>,
    hgt: Option<Height>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<u64>,
}

pub enum Height {
    Cm(u16),
    In(u16)
}

// #[aoc_generator(day4)]
// pub fn input_generator(input: &str) -> Vec<Passport> {
//     let mut passport = Passport::default();
//     let mut result = Vec::new();
//
//     for line in input.lines() {
//         if line == "" {
//             result.push(mem::take(&mut passport));
//             continue
//         }
//
//         for v in line.split_whitespace() {
//             let split: Vec<&str> = v.split(":").collect();
//
//             match split[0] {
//                 "byr" => passport.byr = Some(split[1].parse().unwrap()),
//                 "iyr" => passport.iyr = Some(split[1].parse().unwrap()),
//                 "eyr" => passport.eyr = Some(split[1].parse().unwrap()),
//                 "hgt" => passport.hgt = match split[1].chars().rev().take(2).fold(String::new(), |mut acc, v| { acc.push(v); acc }).as_str() {
//                     "mc" => Some(Height::Cm(split[1].replace("cm", "").parse().unwrap())),
//                     "ni" => Some(Height::In(split[1].replace("in", "").parse().unwrap())),
//                     _ => None,
//                 },
//                 "hcl" => passport.hcl = Some(split[1].to_string()),
//                 "ecl" => passport.ecl = Some(split[1].to_string()),
//                 "pid" => passport.pid = Some(split[1].parse().unwrap()),
//                 "cid" => (),
//                 _ => panic!("Invalid key: {}", split[0]),
//             }
//         }
//     }
//
//     result
// }

const REQUIRED_KEYS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

#[aoc(day4, part1, oneliner)]
pub fn solve_part1_1(input: &str) -> usize {
    input.split("\n\n").map(|v| REQUIRED_KEYS.iter().map(|k| v.contains(k)).filter(|r| *r).count() == REQUIRED_KEYS.len()).filter(|r| *r).count()
}

// #[aoc(day4, part1)]
// pub fn solve_part1(input: &[Passport]) -> usize {
//     input.iter().filter(|passport| {
//         passport.byr.is_some()
//             && passport.iyr.is_some()
//             && passport.eyr.is_some()
//             && passport.hgt.is_some()
//             && passport.hcl.is_some()
//             && passport.ecl.is_some()
//             && passport.pid.is_some()
//     }).count()
// }

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> usize {
    // input.iter().map(|v: &Passport| {
    //     v.byr.unwrap() >= 1920 && v.byr.unwrap() < 2002 &&
    //         v.iyr.unwrap() >= 2010 && v.iyr.unwrap() < 2020 &&
    //         v.eyr.unwrap() >= 2020 && v.eyr.unwrap() < 2030 &&
    //         match v.hgt.unwrap()
    // }).sum()
    0
}