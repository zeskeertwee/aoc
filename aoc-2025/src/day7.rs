use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use fxhash::FxHashSet;
use aoclib::grid::Grid;
use aoclib::memoizer::Memoizer;
use aoclib::vec2::{Direction, Vector2};

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Grid<char> {
    Grid::parse(input, |c| c)
}

#[aoc(day7, part1)]
fn part1(input: &Grid<char>) -> usize {
    let mut beams = FxHashSet::from_iter([input.find_first_occurance(&'S').unwrap()]);
    let mut y = 0;

    let mut split = 0;

    while y < input.height - 1 {
        let mut new_beams = FxHashSet::default();
        for beam in beams {
            let new_beam =  Direction::Down + beam;

            if input[&new_beam] == '^' {
                // split the beam
                split += 1;
                new_beams.insert(Direction::Left + new_beam);
                new_beams.insert(Direction::Right + new_beam);
            } else {
                new_beams.insert(new_beam);
            }
        }

        beams = new_beams;

        y += 1;
    }

    split
}

#[aoc(day7, part2)]
fn part2(input: &Grid<char>) -> usize {
    let mem = Memoizer::new(Box::new(split_beam), input.clone());
    mem.call(input.find_first_occurance(&'S').unwrap())
}

fn split_beam(p: &Vector2<usize>, input: &Grid<char>, mem: &Memoizer<Vector2<usize>, Grid<char>, usize>) -> usize {
    let mut pos = *p;

    while pos.y < input.height - 1 {
        if input[&pos] == '^' {
            return mem.call(Direction::Left + pos) + mem.call(Direction::Right + pos)
        }

        pos = Direction::Down + pos;
    }

    1
}

aoc_test!(test_day7, "../input/2025/day7.txt", 1541, 80158285728929);