# Advent of Code 2025 - Rust Solutions

This repository contains solutions for Advent of Code 2025 written in Rust.

## Project Structure

Each day has its own folder with the following files:
- `main.rs` - The solution code
- `input.txt` - Your personal puzzle input (paste from AoC website)

```
AoC_2025/
├── Cargo.toml        # Global cargo configuration with all days as binaries
├── template.rs       # Template file to copy for new days
├── day01/
│   ├── main.rs      # Solution code
│   └── input.txt    # Puzzle input
├── day02/
│   ├── main.rs
│   └── input.txt
...
```

## Usage

### Running a specific day

```bash
cargo run --bin day01
```

### Testing a specific day

```bash
cargo test --bin day01
```

### Running tests with output

```bash
cargo test --bin day01 -- --nocapture
```

### Building all solutions

```bash
cargo build --release
```

## Adding a New Day

1. Create a new folder: `dayXX/` (e.g., `day04/`)
2. Copy `template.rs` to `dayXX/main.rs`
3. Update the path in the `main.rs` file from `dayXX` to your day number
4. Create an `input.txt` file in the folder
5. The day should already be configured in `Cargo.toml`

## Template Structure

Each day's solution includes:
- **`parse_input`**: Parses the input string into a usable data structure
- **`part1`**: Solution for part 1
- **`part2`**: Solution for part 2
- **`main`**: Reads input and runs both parts
- **Tests**: Uses sample input from the problem description (embedded in the `SAMPLE` const)

### Workflow for Each Day

1. Read the problem on [Advent of Code](https://adventofcode.com/2025)
2. Copy the sample input and paste it into the `SAMPLE` const in the test module
3. Update the expected test values
4. Implement the solution for part 1
5. Run tests: `cargo test --bin dayXX`
6. Paste your actual puzzle input into `dayXX/input.txt`
7. Run your solution: `cargo run --bin dayXX`
8. After solving part 1, implement part 2 and repeat

## Tips

- The template uses a generic `Vec<String>` for parsed data. Modify `parse_input` to return the appropriate data structure for your problem.
- Tests use embedded sample input (in the `SAMPLE` const) which is convenient and keeps everything in one file.
- The solution automatically runs from the project root, so input paths are relative to the workspace root.
- Use `cargo run --release --bin dayXX` for performance-critical problems.

## Happy Coding! 🎄

