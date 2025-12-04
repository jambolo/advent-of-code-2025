// Advent of Code 2025, Day 3

use common::load;

fn main() {
    println!("Day 3, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    let banks = load::lines();

    let mut joltage: u64 = 0;

    for bank in banks {
        let numbers: Vec<u32> = bank.chars().filter_map(|c| c.to_digit(10)).collect();
    
        let count = if cfg!(feature = "part2") { 12 } else { 2 };
        let mut next_i = 0;
        let mut j : u64 = 0;
        for c in 0..count {
            let (i, v) = next_digit(&numbers[next_i..], count - c - 1);
            next_i += i + 1;
            j = j * 10 + v as u64;
        }
        joltage += j;
    }

    println!("Total joltage: {}", joltage);
}

// Find the first number such that none of the following numbers are greater. Exclude the last n numbers.
fn next_digit(numbers: &[u32], n: usize) -> (usize, u32) {
    let mut best_i = 0;
    let mut best_v = numbers[0];
    for i in 1..numbers.len() - n {
        if numbers[i] > best_v {
            best_i = i;
            best_v = numbers[i];
        }
    }
    (best_i, best_v)
}

