use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl TryFrom<Option<char>> for Shape {
    type Error = ();

    fn try_from(value: Option<char>) -> Result<Self, Self::Error> {
        match value {
            Some('A') | Some('X') => Ok(Shape::Rock),
            Some('B') | Some('Y') => Ok(Shape::Paper),
            Some('C') | Some('Z') => Ok(Shape::Scissors),
            _ => Err(())
        }
    }
}

impl Shape {
    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn outcome_score(&self, other: &Shape) -> i32 {
        match (self, other) {
            (Shape::Rock, Shape::Rock) => 3,
            (Shape::Paper, Shape::Paper) => 3,
            (Shape::Scissors, Shape::Scissors) => 3,

            (Shape::Rock, Shape::Scissors) => 6,
            (Shape::Rock, Shape::Paper) => 0,

            (Shape::Paper, Shape::Rock) => 6,
            (Shape::Paper, Shape::Scissors) => 0,

            (Shape::Scissors, Shape::Rock) => 0,
            (Shape::Scissors, Shape::Paper) => 6
        }
    }

    fn part2_get_shape(&self, other: &Shape) -> Shape {
        match (self, other) {
            // we need to lose
            (Shape::Rock, Shape::Rock) => Shape::Scissors,
            (Shape::Rock, Shape::Paper) => Shape::Rock,
            (Shape::Rock, Shape::Scissors) => Shape::Paper,

            // we need to draw
            (Shape::Paper, s) => s.clone(),

            // we need to win
            (Shape::Scissors, Shape::Rock) => Shape::Paper,
            (Shape::Scissors, Shape::Paper) => Shape::Scissors,
            (Shape::Scissors, Shape::Scissors) => Shape::Rock,
        }

    }
}

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Vec<(Shape, Shape)> {
    input.lines().map(|l| {
        let line: Vec<&str> = l.split_whitespace().collect();
        (line[0].chars().next().try_into().unwrap(), line[1].chars().next().try_into().unwrap())
    }).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(Shape, Shape)]) -> i32 {
    input.iter().map(|l| l.1.outcome_score(&l.0) + l.1.score()).sum()
}

#[aoc(day2, part2)]
fn part2(input: &[(Shape, Shape)]) -> i32 {
    input.iter().map(|l| {
        let shape = l.1.part2_get_shape(&l.0);
        shape.outcome_score(&l.0) + shape.score()
    }).sum()
}