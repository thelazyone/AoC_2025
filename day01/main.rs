use std::fs;

// From https://adventofcode.com/2025/day/1

// Standard input parsing
fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

// applying a rotation (positive and negative) with a module
fn change_position(value: i32, module: i32, delta: i32) -> (i32, i32) {

    // If positive, just add and do the module.
    if delta > 0 {
        let mut rotations = (value + delta) / module;
        if (value + delta) % module == 0 {
            rotations -= 1;
        }
        println!("delta is positive, value+delta is {} and crossings are {}", value + delta, rotations);
        return ((value + delta) % module, rotations);
    }

    // if negative, it's trickier. First subtract, if <0 it means there are crossing, take
    // the remaining part and calculate the module
    else if delta < 0 {
        if value + delta >= 0 {
            println!("delta is negative but no crossings");
            return (value + delta, 0);
        }
        else {
            let mut rotations = (value + delta).abs() / module;
            if value != 0 {
                rotations += 1;
            }
            if value + delta % module == 0 {
                rotations -= 1;
            }
            println!("delta is negative, crossings are {} and value + delta is {}", rotations, value + delta);
            return ((value + delta + module * (rotations + 1)) % module, rotations);
        }
    }

    else { // delta == 0
        return (value, 0);
    }
}

fn part1(data: &[String]) -> i32 {
    let mut current_position = 50;
    let module = 100;

    // reading each line, checking if the first character is L or R (or none), then extracting the integer
    // from the rest of the line.
    let mut zero_counter = 0;
    for line in data {
        let direction = line.chars().next().unwrap();
        let steps: i32 = line[1..].parse::<i32>().unwrap();
        match direction {
            'L' => (current_position, _) = change_position(current_position, module, steps * -1),
            'R' => (current_position, _) = change_position(current_position, module, steps),
            _ => panic!("Invalid direction: {}", direction),
        }
        if current_position == 0 {
            zero_counter += 1;
        }
        println!("current_position: {}", current_position);
    }
    zero_counter
}

fn part2(data: &[String]) -> i32 {
    let mut current_position = 50;
    let module = 100;

    // reading each line, checking if the first character is L or R (or none), then extracting the integer
    // from the rest of the line.
    let mut zero_counter = 0;
    let mut zero_crossed: i32;
    for line in data {
        let direction = line.chars().next().unwrap();
        let steps: i32 = line[1..].parse::<i32>().unwrap();
        match direction {
            'L' => (current_position, zero_crossed) = change_position(current_position, module, steps * -1),
            'R' => (current_position, zero_crossed) = change_position(current_position, module, steps),
            _ => panic!("Invalid direction: {}", direction),
        }
        if current_position == 0 {
            zero_counter += 1;
        }
        zero_counter += zero_crossed;
        println!("current_position: {}, counter: {}", current_position, zero_counter);
    }
    zero_counter
}

fn main() {
    let input = fs::read_to_string("day01/input.txt")
        .expect("Failed to read input file");
    
    let data = parse_input(&input);
    
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let sample = fs::read_to_string("day01/sample.txt")
            .expect("Failed to read sample.txt");
        let data = parse_input(&sample);
        assert_eq!(part1(&data), 3);
    }

    #[test]
    fn test_part2() {
        let sample = fs::read_to_string("day01/sample.txt")
            .expect("Failed to read sample.txt");
        let data = parse_input(&sample);
        assert_eq!(part2(&data), 6); // TODO: Update expected value
    }
}