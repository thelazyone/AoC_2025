# Advent of Code 2025 - Rust Solutions

This repository contains solutions for Advent of Code 2025 written in Rust.
It's a low-effort attempt, using Cursors as IDE without agent calls (other than setting up the folder structure initially).

## Project Structure

Each day has its own folder with the following files:
- `main.rs` - The solution code
- `input.txt` - The puzzle input
- `sample.txt` - The example input (to use in tests)

## Usage
- `cargo run --bin day<01-12>` to run a specific day
- `cargo test --bin day<01-12>` to test a specific day
- `cargo test` to runn all the days implemented so far

## Adding a New Day
Copy the structure of the current folders, and possibly use the template.rs 
Note that the `Cargo.toml` file has explicit definition for each day, as I haven't found a procedural way.