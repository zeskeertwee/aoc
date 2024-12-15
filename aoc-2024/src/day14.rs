use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use aoclib::vec2::Vector2;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Robot {
    position: Vector2<i32>,
    velocity: Vector2<i32>,
}

#[aoc_generator(day14)]
fn parse_input(input: &str) -> Vec<Robot> {
    input.lines()
        .map(|l| {
            let split: Vec<&str> = l.split_whitespace().collect();
            let pos: Vec<i32> = split[0].replace("p=", "").split(',').map(|v| v.parse::<i32>().unwrap()).collect();
            let vel: Vec<i32> = split[1].replace("v=", "").split(',').map(|v| v.parse::<i32>().unwrap()).collect();

            Robot {
                position: Vector2::new(pos[0], pos[1]),
                velocity: Vector2::new(vel[0], vel[1]),
            }
        })
        .collect()
}

#[aoc(day14, part1)]
fn part1(input: &[Robot]) -> usize {
    let mut robots = input.to_owned();
    let size = if input.len() < 20 {
        Vector2::new(11, 7)
    } else {
        Vector2::new(101, 103)
    };
    step_robots(&mut robots, 100, &size);

    safety_factor(&robots, &size)
}

#[aoc(day14, part2)]
fn part2(input: &[Robot]) -> usize {
    let mut robots = input.to_owned();
    let size = if input.len() < 20 {
        Vector2::new(11, 7)
    } else {
        Vector2::new(101, 103)
    };

    // assuming the christmas tree is centered, the safety factor should be the lowest
    // TODO: checking for clustered robots on the x and y axis and then finding the LCM of those times should be much faster
    (1..10000usize)
        .map(|t| {
            step_robots(&mut robots, 1, &size);
            (safety_factor(&robots, &size), t)
        })
        .min_by_key(|(s, _)| *s)
        .unwrap().1
}

fn step_robots(robots: &mut Vec<Robot>, t: i32, size: &Vector2<i32>) {
    for robot in robots.iter_mut() {
        robot.position.x = (robot.position.x + t * robot.velocity.x).rem_euclid(size.x);
        robot.position.y = (robot.position.y + t * robot.velocity.y).rem_euclid(size.y);
    }
}

fn safety_factor(robots: &[Robot], size: &Vector2<i32>) -> usize {
    let mut quadrant_scores = [0; 4];

    for robot in robots.iter() {
        let left = robot.position.x < (size.x / 2);
        let right = robot.position.x > (size.x / 2);
        let top = robot.position.y < (size.y / 2);
        let bottom = robot.position.y > (size.y / 2);
        match (left, right, top, bottom) {
            (true, false, true, false) => quadrant_scores[0] += 1,
            (false, true, true, false) => quadrant_scores[1] += 1,
            (false, true, false, true) => quadrant_scores[2] += 1,
            (true, false, false, true) => quadrant_scores[3] += 1,
            _ => (),
        }
    }

    quadrant_scores.iter().product()
}

// part 2 for the sample doesn't actually make sense to check
aoc_test!(test_day14_sample, "../samples/day14.txt", 12);
aoc_test!(test_day14, "../input/2024/day14.txt", 218619324, 6446);