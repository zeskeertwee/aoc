use std::mem;

#[derive(Default)]
pub struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Passport> {
    let mut passport = Passport::default();
    let mut result = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            result.push(mem::take(&mut passport));
        }

        for v in line.split_whitespace() {
            let split: Vec<&str> = v.split(":").collect();

            match split[0] {
                "byr" => passport.byr = Some(split[1].to_string()),
                "iyr" => passport.iyr = Some(split[1].to_string()),
                "eyr" => passport.eyr = Some(split[1].to_string()),
                "hgt" => passport.hgt = Some(split[1].to_string()),
                "hcl" => passport.hcl = Some(split[1].to_string()),
                "ecl" => passport.ecl = Some(split[1].to_string()),
                "pid" => passport.pid = Some(split[1].to_string()),
                "cid" => (),
                _ => panic!("Invalid key: {}", split[0]),
            }
        }
    }

    result
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Passport]) -> usize {
    input.iter().filter(|passport| {
        passport.byr.is_some()
            && passport.iyr.is_some()
            && passport.eyr.is_some()
            && passport.hgt.is_some()
            && passport.hcl.is_some()
            && passport.ecl.is_some()
            && passport.pid.is_some()
    }).count()
}