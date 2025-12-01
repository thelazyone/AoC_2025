use std::fs;

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

fn part1(data: &[String]) -> i32 {
    // TODO: Implement part 1
    0
}

fn part2(data: &[String]) -> i32 {
    // TODO: Implement part 2
    0
}

fn main() {
    let input = fs::read_to_string("dayXX/input.txt")
        .expect("Failed to read input file");
    
    let data = parse_input(&input);
    
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Paste the sample input from the problem description here
    const SAMPLE: &str = "\
example line 1
example line 2";

    #[test]
    fn test_part1() {
        let data = parse_input(SAMPLE);
        assert_eq!(part1(&data), 0); // TODO: Update expected value
    }

    #[test]
    fn test_part2() {
        let data = parse_input(SAMPLE);
        assert_eq!(part2(&data), 0); // TODO: Update expected value
    }
}

