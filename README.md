[![AoC Tests](https://github.com/zeskeertwee/aoc/actions/workflows/test.yaml/badge.svg)](https://github.com/zeskeertwee/aoc/actions/workflows/test.yaml)
## Advent of code solutions in Rust
I try to use as little libraries as reasonably possible to solve the problems.
I only use:
- `rayon` for paralellization
- `fxhash` for faster `HashMap` and `HashSet` implementations
- `aoclib` is my own library containing implementations of useful things for advent of code such as a `Grid`, `Vector2`, `Direction`, etc.
- `bitvec` in my own `aoclib` for faster boolean arrays

Benchmarks and library usage per day for each year:
- [2015 Solutions](./aoc-2015/README.md)
- [2024 Solutions](./aoc-2024/README.md)
