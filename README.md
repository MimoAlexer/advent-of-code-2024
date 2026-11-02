# Advent of Code 2024 - Rust

Solutions for all 25 days of [Advent of Code 2024](https://adventofcode.com/2024), covering both computational parts. Day 25's second star is awarded by the website after the other stars; it has no separate solver.

Requires a current stable Rust toolchain. No external crates, network access, account credentials, or input downloads are needed to build or run.

```sh
cargo run --release -- 1 1 inputs/day01.txt
cargo run --release -- 24 2 inputs/day24.txt
```

Arguments are `DAY PART [INPUT_FILE]`. Without a file, the program reads standard input:

```sh
cat inputs/day01.txt | cargo run --release -- 1 2
```

The answer is printed on its own line. The library exposes `advent_of_code_2024::solve(day, part, input) -> Result<String, String>`. Inputs should follow the puzzle format; malformed puzzle data can panic. Windows CRLF line endings are supported.

## Verification

```sh
cargo fmt -- --check
cargo test --locked
cargo build --release --locked
```

Tests cover every day with official examples and additional cases, including both warehouse widths, VM execution and the example quine, four swapped pairs in a generated ripple-carry adder, and CRLF input. Personal puzzle inputs and answer submissions have not been verified. Download your own input from Advent of Code and keep it under the ignored `inputs/` directory.

## Algorithms

| Days | Main techniques |
| --- | --- |
| 1-5 | Sorting, report validation, instruction scanning, directional search, topological ordering |
| 6-10 | Guard state tracking, equation search, antenna rays, disk compaction, ascending trail search |
| 11-15 | Stone frequency maps, region corners, integer linear equations, torus search, atomic box pushes |
| 16-20 | Dijkstra with optimal-path backtracking, VM and reverse octal search, BFS and binary search, towel dynamic programming, cheat endpoint distances |
| 21-25 | Memoised keypad transitions, secret sequence aggregation, maximum clique, structural adder repair, lock/key profiles |

Puzzle-specific choices:

- Day 14 uses a 101x103 torus, or 11x7 for the small example coordinates. Part 2 searches the entire torus period for minimum positional variance as a compactness heuristic for the visual Christmas tree. This is a candidate detector, not a general tree recogniser. Inspect its arrangement with `cargo run --release --example day14_render -- inputs/day14.txt`. Supply a second argument to render a different timestamp.
- Day 17 part 2 reverses the puzzle's octal-loop program. Each candidate suffix is checked with the actual VM, and the final answer must reproduce the whole program. Arbitrary VM programs without that loop structure are outside this search strategy.
- Day 18 uses a 71x71 grid and 1,024 bytes, or a 7x7 grid and 12 bytes for the small example coordinates.
- Day 20 counts cheats saving at least 100 picoseconds, with a maximum duration of 2 or 20.
- Day 24 part 2 expects a ripple-carry adder with four disjoint pairs of swapped gate outputs. It checks every adder identity before accepting a repair.

`src/lib.rs` contains days 1-13 and the public dispatcher; `src/late.rs` contains days 14-25. The binary is a thin CLI wrapper.

Puzzle descriptions and inputs belong to Advent of Code and are not redistributed. Tests include small example fixtures; no personal inputs are committed. Implementation generated with OpenAI Codex.
