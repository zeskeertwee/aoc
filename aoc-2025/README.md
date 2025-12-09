## Advent of Code 2024
The goal is to solve all AoC 2025 exersises (both parts) in <500ms.

| Day | Part 1  | Part 2  | Libraries      |
|-----|---------|---------|----------------|
| 1   | 8.274μs | 11.81μs |                |
| 2   | 2.601ms | 128.9ms | Rayon          |
| 3   |         |         | Rayon (part 2) |
| 4   | 1.066ms | 23.99ms |                |
| 5   | 69.86μs | 38.04μs | Rayon (part 2) |
| 6   |         |         |                |
| 7   |         |         |                |
| 8   |         |         |                |
| 9   |         |         |                |
| 10  |         |         |                |
| 11  |         |         |                |
| 12  |         |         |                |

\* = Part 1/2 solution require (nearly) the same computation, can both be solved in this time

All benchmarked on `Ryzen 7 3700X, 48GB (2 x 8GB, 2 x 16GB) 3200MT/s` desktop using `cargo aoc bench -d [day]`.
