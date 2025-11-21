use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use aoclib::graph::{Graph, Edge};

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Graph<String, u16> {
    let edges: Vec<Edge<String, u16>> = input.lines()
        .map(|l| {
            let split: Vec<&str> = l.split_whitespace().collect();
            Edge {
                src: split[0].to_string(),
                dst: split[2].to_string(),
                cost: split[4].parse().unwrap()
            }
        })
        .collect();

    Graph::new_bidirectional_edges(edges)
}

#[aoc(day9, part1)]
fn part1(input: &Graph<String, u16>) -> u16 {
    input.bfs_find_shortest_path_length(|path, graph| path.visited.len() + 1 == graph.nodes.len()).unwrap()
}

#[aoc(day9, part2)]
fn part2(input: &Graph<String, u16>) -> u16 {
    input.find_longest_path_visiting_all()
}

aoc_test!(test_day9, "../input/2015/day9.txt", 141, 736);