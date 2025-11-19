use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use aoclib::vec2::Vector2;
use aoclib::grid::Grid;

struct Instruction {
    kind: InstructionKind,
    start: Vector2<usize>,
    end: Vector2<usize>
}

enum InstructionKind {
    On,
    Off,
    Toggle
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|l| {
            let split: Vec<Vector2<usize>> = l.split(" through ")
                .map(|s| {
                    let coords = s.split(",").map(|v| {
                        let num = v.chars().filter(|c| c.is_numeric()).collect::<String>();
                        num.parse().unwrap()
                    }).collect::<Vec<usize>>();
                    Vector2::new(coords[0], coords[1])
                }).collect();

            Instruction {
                start: split[0],
                end: split[1],
                kind: match l.chars().nth(6).unwrap() {
                    'n' => InstructionKind::On,
                    'f' => InstructionKind::Off,
                    ' ' => InstructionKind::Toggle,
                    _ => panic!()
                }
            }
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &[Instruction]) -> usize {
    let mut grid = Grid::fill(false, 1000, 1000);

    for i in input {
        for x in i.start.x..=i.end.x {
            for y in i.start.y..=i.end.y {
                let pos = Vector2::new(x, y);
                grid[&pos] = match i.kind {
                    InstructionKind::Off => false,
                    InstructionKind::On => true,
                    InstructionKind::Toggle => !grid[&pos]
                }
            }
        }
    }

    grid.grid.iter().map(|v| *v as usize).sum()
}

#[aoc(day6, part2)]
fn part2(input: &[Instruction]) -> usize {
    let mut grid = Grid::fill(0, 1000, 1000);

    for i in input {
        for x in i.start.x..=i.end.x {
            for y in i.start.y..=i.end.y {
                let pos = Vector2::new(x, y);
                grid[&pos] += match i.kind {
                    InstructionKind::Off => if grid[&pos] > 0 { -1 } else { 0 },
                    InstructionKind::On => 1,
                    InstructionKind::Toggle => 2
                }
            }
        }
    }

    grid.grid.iter().sum::<isize>() as usize
}

aoc_test!(test_day6, "../input/2015/day6.txt", 569999, 17836115);