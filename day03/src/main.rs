// Advent of Code 2025, Day 3

use common::load;

fn main() {
    println!("Day 3, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    let banks = load::lines();

    let mut joltage = 0;

    for bank in banks {
        let numbers: Vec<u32> = bank.chars().filter_map(|c| c.to_digit(10)).collect();

        // Find the first number such that none of the following numbers are greater. Exclude the last number.
        let (first_i, &first_v) = numbers
            .iter()
            .enumerate()
            .take(numbers.len() - 1)
            .fold(
                (0, &numbers[0]),
                |(max_i, max_v), (i, v)| {
                    if v > max_v {
                        (i, v)
                    } else {
                        (max_i, max_v)
                    }
                },
            );

        // Find the highest number after that position.
        let second_v = numbers[first_i + 1..].iter().max().unwrap();
    
        let sorted_bank = {
            let mut sb = numbers.clone();
            sb.sort_unstable();
            sb.iter().map(|n| n.to_string()).collect::<String>()
        };
        println!("Bank (sorted): {} joltage:{}{}", sorted_bank, first_v, second_v);
        joltage += first_v * 10 + second_v;
    }

    println!("Total joltage: {}", joltage);
}