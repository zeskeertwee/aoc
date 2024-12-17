use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use aoclib::grid::Grid;
use aoclib::vec2::{Direction, Vector2, DIRECTIONS};
use fxhash::{FxHashMap, FxHashSet};

struct Input {
    grid: Grid<char>,
    starting_pos: Vector2<usize>,
    target_pos: Vector2<usize>
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Reindeer {
    position: Vector2<usize>,
    direction: Direction,
    path: Vec<Vector2<usize>>,
    cost: usize
}

impl Ord for Reindeer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Input {
    let grid = Grid::parse(input, |c| c);
    let starting_pos = grid.find_first_occurance(&'S').unwrap();
    let target_pos = grid.find_first_occurance(&'E').unwrap();

    Input {
        grid, starting_pos, target_pos
    }
}

#[aoc(day16, part1)]
fn part1(input: &Input) -> usize {
    bfs_maze2(&input.grid, input.starting_pos, Direction::Right,input.target_pos, false)[0].cost
}

#[aoc(day16, part2)]
fn part2(input: &Input) -> usize {
    let paths = bfs_maze2(&input.grid, input.starting_pos, Direction::Right, input.target_pos, true);
    let min_cost = paths[0].cost; // assume first path is shortest

    paths.iter().filter(|p| p.cost == min_cost).fold(FxHashSet::default(), |mut acc, p| {
        for i in &p.path {
            acc.insert(i);
        }
        acc
    }).len()
}

fn bfs_maze2(grid: &Grid<char>, starting_pos: Vector2<usize>, starting_dir: Direction, target_pos: Vector2<usize>, find_multiple: bool) -> Vec<Reindeer> {
    let mut result = Vec::new();
    let mut shortest_len = None;
    let mut queue: BinaryHeap<Reverse<Reindeer>> = BinaryHeap::new();
    // stores cost to reach the node
    let mut visited: FxHashMap<(Vector2<usize>, Direction), usize> = FxHashMap::default();
    queue.push(Reverse(Reindeer {
        position: starting_pos,
        direction: starting_dir,
        path: vec![starting_pos],
        cost: 0
    }));

    while !queue.is_empty() {
        let reindeer = queue.pop().unwrap().0;
        if reindeer.position == target_pos {
            if reindeer.cost < shortest_len.unwrap_or(usize::MAX) {
                shortest_len = Some(reindeer.cost);
            }

            result.push(reindeer);

            if !find_multiple {
                 return result;
            }
            continue;
        }

        if let Some(c) = visited.get(&(reindeer.position, reindeer.direction)) {
            if c < &reindeer.cost || reindeer.cost > shortest_len.unwrap_or(usize::MAX) {
                // stop with this reindeer, we have already found a shorter path
                continue;
            }
        }

        visited.insert((reindeer.position, reindeer.direction), reindeer.cost);

        for dir in DIRECTIONS {
            if dir.inv() == reindeer.direction {
                // don't walk back
                continue;
            }

            let rotate = dir != reindeer.direction;
            let new_cost = reindeer.cost + if !rotate { 1 } else { 1001 };
            let i = dir + reindeer.position;

            if grid[&i] != '#' && !visited.contains_key(&(i, dir)) {
                // we can make the step
                let mut path = reindeer.path.clone();
                path.push(i);
                queue.push(Reverse(Reindeer {
                    direction: dir,
                    position: i,
                    cost: new_cost,
                    path
                }));
            }
        }
    }

    result
}

aoc_test!(test_day16_sample1, "../samples/day16-1.txt", 7036, 45);
aoc_test!(test_day16_sample2, "../samples/day16-2.txt", 11048, 64);
aoc_test!(test_day16, "../input/2024/day16.txt", 109516, 568);