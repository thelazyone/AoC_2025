use std::fs;

// From https://adventofcode.com/2025/day/2

// Parses comma-separated pairs of "X-Y" into a Vec of (i64, i64) tuples
fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .trim()
        .split(',')
        .map(|pair| {
            let mut parts = pair.trim().split('-');
            let a: i64 = parts.next().unwrap().parse().unwrap();
            let b: i64 = parts.next().unwrap().parse().unwrap();
            (a, b)
        })
        .collect()
}

fn part1(data: &[(i64, i64)]) -> i64 {
    let mut all_faulty_values : Vec<i64> = Vec::new();

    for (a, b) in data {

        let mut current_pair = (*a, *b);
        
        // First of all, faulty IDs must have an even number of digits.
        // This means that in an id range, if the starting ID has an odd number of digits, 
        // we can consider from the first value after it that has even numbers.
        if current_pair.0.to_string().len() % 2 == 1 {

            // The smaller number with the given digits is a 1 followed by a.to_string().len() zeros.
            let smallest_number = 10_i64.pow(current_pair.0.to_string().len() as u32);

            // If the second number is smaller than this, no faulty can happen.
            if current_pair.1 < smallest_number {
                continue;
            }
            current_pair.0 = smallest_number;

            
            // Now checking if the input sizes are compatible.
            // It's fair to assume that they are for now, otherwise I'll change the logic later on.
            if current_pair.0.to_string().len() != current_pair.1.to_string().len() {
                panic!("Input sizes are not compatible ({} and {})", current_pair.0, current_pair.1);
            }
        }

        // Now if the second number is bigger in digits:
        if current_pair.1.to_string().len() % 2 == 1 {

            // Returning panic if the size of the second is greater than the first by more than 1.
            if current_pair.1.to_string().len() > current_pair.0.to_string().len() + 1 {
                panic!("Input sizes are not compatible ({} and {})", current_pair.0, current_pair.1);
            }
            
            // Otherwise, setting the max to the largest possible number with the right amount of digits
            // (a number of given lenght made all out of nines)
            current_pair.1 = "9".repeat(current_pair.1.to_string().len() - 1).parse::<i64>().unwrap();
        }

        // Finally, splitting each number into two values, converting to i64.
        // Doing that with mod with 10^len/2
        let len = current_pair.0.to_string().len() as u32;
        let start_b = current_pair.0 % 10_i64.pow(len / 2);
        let start_a = current_pair.0 / 10_i64.pow(len / 2);
        let end_b = current_pair.1 % 10_i64.pow(len / 2);
        let end_a = current_pair.1 / 10_i64.pow(len / 2);
        

        // If start_b is less or equal to start_a, there is a double.
        if start_b <= start_a{
            let candidate = start_a * 10_i64.pow(len / 2) + start_a;
            if candidate <= end_a * 10_i64.pow(len / 2) + end_b {
                all_faulty_values.push(candidate);
            }
        }

        // If end_b is greater than end_a, there is a double.
        if  end_b >= end_a && start_a != end_a{
            all_faulty_values.push(end_a * 10_i64.pow(len / 2) + end_a);
        }

        // Every value in between has exactly one value.
        for i in start_a + 1..end_a {
            all_faulty_values.push(i * 10_i64.pow(len / 2) + i);
        }


    }

    // returning the sum of the faulty values
    all_faulty_values.iter().sum()
}

fn part2(data: &[(i64, i64)]) -> i64 {
    // Brute forcing for part 2
    let mut all_faulty_values : Vec<i64> = Vec::new();

    // println!("data: {:?}", data);

    for (a, b) in data {
        // println!("checking: {:?}-{:?}", a, b);

        // Iterating every value in the range, applying checks.
        for i in *a..=*b {

            let len = i.to_string().len();

            // First checking size, iterating for each value less than size, and see
            // if len mod that value is 0.
            for sub_len in 1..len {

                let sections_number = len / sub_len;

                // Check if the j is a division of len
                if len % sub_len != 0 {
                    continue;
                }

                // Extracting the first section from the integer:
                let mask = 10_i64.pow((sub_len * (sections_number - 1)) as u32);
                let first_piece = i / mask;
                let mut remainder = i % mask;

                // For each other segment of lenght sub_len, check if they're identical to the first piece.
                // The amount of segments is len / sub_len.
                let mut all_identical = true;
                for section_i in 1..sections_number {
                    let submask = 10_i64.pow((sub_len * (sections_number - section_i - 1)) as u32);
                    let other_piece = remainder / submask;
                    remainder = remainder % submask;
                    if first_piece != other_piece {
                        all_identical = false;  
                        break;
                    }
                }

                if all_identical {
                    all_faulty_values.push(i);
                    break;
                }

            }
        }
    }
    // returning the sum of the faulty values
    all_faulty_values.iter().sum()
}

fn main() {
    let input = fs::read_to_string("day02/input.txt")
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
        let sample = fs::read_to_string("day02/sample.txt")
            .expect("Failed to read sample.txt");
        let data = parse_input(&sample);
        assert_eq!(part1(&data), 1227775554);
    }

    #[test]
    fn test_part2() {
        let sample = fs::read_to_string("day02/sample.txt")
            .expect("Failed to read sample.txt");
        let data = parse_input(&sample);
        assert_eq!(part2(&data), 4174379265); // TODO: Update expected value   
    }
}