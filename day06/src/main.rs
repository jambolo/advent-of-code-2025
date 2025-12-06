// Advent of Code 2025, Day 6

use common::load;

fn main() {
    println!("Day 6, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    let lines = load::lines();

    let mut columns: Vec<Vec<i64>> = Vec::new();
    if cfg!(feature = "part2") {
        // Find the length of the longest line to determine the number of columns.
        let number_of_columns = lines.iter().take(lines.len() - 1).map(|line| line.len()).max().unwrap();

        // For all but the last line, each column of text contains a number, one digit per line from highest
        // significance to lowest. Blanks are ignored. A column of all spaces (with the value 0) separates each list of
        // numbers.
        let mut list: Vec<i64> = Vec::new();
        for c in 0..number_of_columns {
            let mut value: i64 = 0;
            for line in lines.iter().take(lines.len() - 1) {
                let ch = line.chars().nth(c).unwrap_or(' ');
                if ch != ' ' {
                    value = value * 10 + ch.to_digit(10).unwrap() as i64;                
                }
            }
            if value == 0 {
                // A column of all spaces indicates the end of a list.
                columns.push(list);
                list = Vec::new();
            } else {
                list.push(value);
            }
        }
        if !list.is_empty() {
            columns.push(list);
        }
    } else {
        // Each line consists of a list of numbers separated by one or more spaces.
        // For all but the last line, process the input data. A vector of vectors of numbers is created such that each
        // vector contains the numbers in the corresponding column.
        for line in lines.iter().take(lines.len() - 1) {
            let numbers: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            for (i, &num) in numbers.iter().enumerate() {
                if columns.len() <= i {
                    columns.push(Vec::new());
                }
                columns[i].push(num);
            }
        }
    }

    // The last line contains the operation to perform on each column.
    let operations: &Vec<char> = &lines[lines.len() - 1]
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();

    let mut sum: i64 = 0;
    for (i, column) in columns.iter().enumerate() {
        let result:i64 = match operations[i] {
            '+' => column.iter().sum(),
            '*' => column.iter().product(),
            _ => panic!("Unknown operation"),
        };
        sum += result;
    }
    println!("Sum: {}", sum);
}
