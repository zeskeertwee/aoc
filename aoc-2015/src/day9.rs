use aoc_runner_derive::{aoc, aoc_generator};
use aoclib::aoc_test;
use aoclib::graph::{Graph, Edge};

fn dest_to_id(ids: &mut Vec<String>, dst: &str) -> u8 {
    let dst = dst.to_string();
    if let Some(idx) = ids.iter().enumerate().find(|v| v.1 == &dst).map(|v| v.0) {
        idx as u8
    } else {
        ids.push(dst);
        ids.len() as u8 - 1
    }
}

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Graph<u8, u16> {
    let mut destination_ids: Vec<String> = vec![];
    let edges: Vec<Edge<u8, u16>> = input.lines()
        .map(|l| {
            let split: Vec<&str> = l.split_whitespace().collect();
            Edge {
                src: dest_to_id(&mut destination_ids, split[0]),
                dst: dest_to_id(&mut destination_ids, split[2]),
                cost: split[4].parse().unwrap()
            }
        })
        .collect();

    Graph::new_bidirectional_edges(edges)
}

#[aoc(day9, part1)]
fn part1(input: &Graph<u8, u16>) -> u16 {
    input.bfs_find_shortest_path_length(|path, graph| path.visited.len() + 1 == graph.nodes.len()).unwrap()
}

#[aoc(day9, part2)]
fn part2(input: &Graph<u8, u16>) -> u16 {
    input.bfs_find_longest_path_length(|path, graph| path.visited.len() + 1 == graph.nodes.len())
}

aoc_test!(test_day9, "../input/2015/day9.txt", 141, 736);
aoc_test!(test_day9_t, "../samples/day9_t.txt", 251, 898);