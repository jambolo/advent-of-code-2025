// Advent of Code 2025, Day 2

use common::load;

fn main() {
    println!("Day 2, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    // Load the data
    let input = load::string();

    // Parse the input into a vector of ranges.
    let ranges = parse_ranges(&input);

    // Gonna solve this the naive way. Iterate through all numbers in each range and look for doubled digits.
    let mut sum: i64 = 0;
    for (start, end) in ranges {
        for number in start..=end {
            let num_str = number.to_string();
            if cfg!(feature = "part2") {
                for n in 2..=num_str.len() {
                    if duplicated(&num_str, n) {
                        sum += number;
                        break;
                    }
                }
            } else {
                if duplicated(&num_str, 2) {
                    sum += number;
                }
            }
        }
    }
    println!("Sum: {}", sum);
}

/// Parses a string of comma-separated ranges into a vector of (start, end) tuples.
fn parse_ranges(input: &str) -> Vec<(i64, i64)> {
    let ranges = input
        .trim()
        .split(',')
        .map(|range_str| {
            let endpoints: Vec<&str> = range_str.split('-').collect();
            if endpoints.len() != 2 {
                panic!("Invalid range format: {}", range_str);
            }
            let start = endpoints[0].parse::<i64>().expect("Invalid start of range");
            let end = endpoints[1].parse::<i64>().expect("Invalid end of range");
            (start, end)
        })
        .collect();
    ranges
}

fn duplicated(s: &str, n: usize) -> bool {
    let len = s.len();
    // Must split evenly into n parts
    if len % n != 0 {
        return false;
    }
    let part_len = len / n;
    let first_part = &s[..part_len];
    for i in 1..n {
        let start = i * part_len;
        let end = start + part_len;
        if &s[start..end] != first_part {
            return false;
        }
    }
    true
}