use std::fs;

// From https://adventofcode.com/2025/day/1

// Standard input parsing
fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

// applying a rotation (positive and negative) with a module
fn change_position(value: u32, module: u32, delta: i32) -> (u32, u32) {

    // If positive, just add and do the module.
    if delta > 0 {
        let mut rotations = (value as i32 + delta) / module as i32;
        if (value as i32 + delta) % module as i32 == 0 {
            rotations -= 1;
        }
        return ((value + delta as u32) % module, rotations as u32);
    }

    // if negative, it's trickier. First subtract, if <0 it means there are crossing, take
    // the remaining part and calculate the module
    else if delta < 0 {
        if value as i32 + delta >= 0 {
            // If the sum is positive, we don't need to cross anything.
            return ((value as i32 + delta) as u32, 0); // We know it's positive anyways.
        }
        else {
            // Otherwise, there is across... But if the start point is zero, it is a false positive.
            let mut rotations: u32 = (value as i32 + delta).abs() as u32 / module;
            if value != 0 {
                rotations += 1;
            }
            if (value as i32 + delta) % module as i32 == 0 {
                rotations = std::cmp::max(rotations - 1, 0); // making sure itìs positive
            }
            let module_shift = module as i32 * (rotations as i32 + 1);
            return ((value as i32 + delta + module_shift) as u32 % module, rotations as u32);
        }
    }

    else { // delta == 0
        return (value, 0);
    }
}

fn part1(data: &[String]) -> u32 {
    let mut current_position = 50;
    let module = 100;

    // reading each line, checking if the first character is L or R (or none), then extracting the integer
    // from the rest of the line.
    let mut zero_counter = 0;
    for line in data {
        let direction = line.chars().next().unwrap();
        let steps: i32 = line[1..].parse::<i32>().unwrap();
        match direction {
            'L' => (current_position, _) = change_position(current_position, module, -steps),
            'R' => (current_position, _) = change_position(current_position, module, steps),
            _ => panic!("Invalid direction: {}", direction),
        }
        if current_position == 0 {
            zero_counter += 1;
        }
    }
    zero_counter
}

fn part2(data: &[String]) -> u32 {
    let mut current_position = 50;
    let module = 100;

    // reading each line, checking if the first character is L or R (or none), then extracting the integer
    // from the rest of the line.
    let mut zero_counter = 0;
    let mut zero_crossed: u32;
    for line in data {
        let direction = line.chars().next().unwrap();
        let steps: i32 = line[1..].parse::<i32>().unwrap();
        match direction {
            'L' => (current_position, zero_crossed) = change_position(current_position, module, -steps),
            'R' => (current_position, zero_crossed) = change_position(current_position, module, steps),
            _ => panic!("Invalid direction: {}", direction),
        }
        if current_position == 0 {
            zero_counter += 1;
        }

        // Adding the crossing too. Note that crossings because zero is the start point are ignored.
        zero_counter += zero_crossed;
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
        assert_eq!(part2(&data), 6);
    }
}