use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::grid::Grid;
use aoclib::vec2::{Direction, Vector2};
use rayon::prelude::*;

type Region = Vec<Vector2<usize>>;

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Grid<char> {
    Grid::parse(input, |c| c)
}

#[aoc(day12, part1)]
fn part1(input: &Grid<char>) -> usize {
    input.flood_fill()
        .par_iter()
        .map(|r| region_perimeter_length(input, &r) * r.len())
        .sum()
}

#[aoc(day12, part2)]
fn part2(input: &Grid<char>) -> usize {
    input.flood_fill()
        .into_par_iter()
        .map(|r| r.len() * region_corners(r))
        .sum()
}

fn region_perimeter_length(grid: &Grid<char>, region: &Region) -> usize {
    // for each square, each neighbouring square removes 1 from it's perimeter
    let mut square_perimiters = vec![4; region.len()];

    for (idx, i) in region.iter().enumerate() {
        for neighbor in grid.neighbour_squares(i) {
            if region.contains(&neighbor) {
                square_perimiters[idx] -= 1;
            }
        }
    }

    square_perimiters.iter().sum()
}

const CORNERS: [[Direction; 2]; 4] = [
    [Direction::Up, Direction::Left], // TOP LEFT
    [Direction::Left, Direction::Down], // BOT LEFT
    [Direction::Down, Direction::Right], // BOT RIGHT
    [Direction::Right, Direction::Up] // TOP RIGHT
];

fn region_corners(region: Region) -> usize {
    let mut corners = 0;

    for i in region.iter() {
        // for every corner of the cube, we check if we have 0, 1 or 2 neighbours in the UP/RIGHT/DOWN/lEFT directions, whichever are applicable
        // (i.e. for the top-right corner, check TOP/RIGHT.
        // case 1: 0 neighbours, 1 corner
        // case 2: 1 neighbour, 0 corners
        // case 3: 2 neighbours
        //   |-> case 3a: no diagonal neighbour, 1 corner
        //   |-> case 3b: diagonal neighbour, 0 corners
        for [a, b] in CORNERS {
            let va = a + *i;
            let vb = b + *i;
            let vdiag = a + (b + *i);

            let n_neighbours = region.contains(&va) as usize + region.contains(&vb) as usize;
            let ndiag = region.contains(&vdiag);

            corners += match (n_neighbours, ndiag) {
                (0, _) => 1,
                (1, _) => 0,
                (2, false) => 1,
                (2, true) => 0,
                _ => panic!("Unexpected neighbour counts!"),
            }
        }
    }

    corners
}