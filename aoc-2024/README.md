## Advent of Code 2024
The goal is to solve all AoC 2024 exersises (both parts) in <1000ms. Preferably <500ms.

| Day | Part 1   | Part 2   | Libraries              |
|-----|----------|----------|------------------------|
| 1   | 14.61μs  | 12.35μs  | FxHash (part 2)        |
| 2   | 3.294μs  | 79.75μs  |                        |
| 3   | 88.47μs  | 142.9μs  |                        |
| 4   | 685.7μs  | 155.8μs  | Rayon (part 1)         |
| 5   | 188.80μs | 408.6μs  | Rayon                  |
| 6   | 145.5μs  | 5.100ms  | FxHash, Rayon (part 2) |
| 7   | 98.35μs  | 2.635ms  | Rayon                  |
| 8   | 11.71μs  | 30.16μs  | FxHash                 |
| 9   | 359.6μs  | 941.8μs  |                        |
| 10  | 109.6μs  | 111.7μs  | FxHash, Rayon          |
| 11  | 121.0μs  | 4.681ms  | FxHash                 |
| 12  | 2.157ms  | 3.025ms  | FxHash, Rayon          |
| 13  | 6.890μs  | 52.64μs  |                        |
| 14  | 4.807μs  | 6.700ms  | Rayon (part 2)         |
| 15  | 226.5μs  | 459.0μs  | FxHash                 |
| 16  | 39.59ms* | 38.76ms* | FxHash                 |
| 17  | 509.4ns  | 58.28μs  |                        |
| 18  | 229.7μs  | 392.2μs  |                        |
| 19  | 68.86μs  | 412.7μs  | FxHash, Rayon          |
| 20  | 2.042ms  | 14.14ms  | Rayon                  |
| 21  |          |          |                        |
| 22  | 665.6μs  |          | Rayon (part 1)         |
| 23  |          |          |                        |
| 24  |          |          |                        |
| 25  |          |          |                        |

\* = Part 1/2 solution require (nearly) the same computation, can both be solved in this time

All benchmarked on `Ryzen 7 3700X, 48GB (2 x 8GB, 2 x 16GB) 3200MT/s` desktop using `cargo aoc bench -d [day]`.
