use aoc_2024::day16::Day16;
use aoclib::AocDay;
use rayon;

fn main() {
    println!("Initializing rayon");
    rayon::ThreadPoolBuilder::new().build_global().unwrap();

    Day16::run(include_str!("../input/2024/day16.txt"));
}