// Advent of Code 2025, Day 5

use common::load;

fn main() {
    println!("Day 5, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    let lines = load::lines();

    // Split the lines into two sections: fresh ranges and ingredient IDs
    let mut sections = lines.split(|line| line.is_empty());

    // Parse the fresh ingredient ranges
    let fresh_ranges: Vec<(i64, i64)> = sections
        .next()
        .unwrap()
        .iter()
        .map(|line| parse_range(line))
        .collect();

    // Parse the ingredient IDs
    let ingredient_ids: Vec<i64> = sections
        .next()
        .unwrap()
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();

    // Determine which ingredient IDs are fresh
    let fresh_ingredients: Vec<i64> = ingredient_ids
        .into_iter()
        .filter(|&id| {
            fresh_ranges.iter().any(|&(start, end)| id >= start && id <= end)
        })
        .collect();

    // Output the result
    println!("Number of fresh ingredients: {}", fresh_ingredients.len());
}

fn parse_range(line: &str) -> (i64, i64) {
    let parts: Vec<&str> = line.split('-').collect();
    let start: i64 = parts[0].parse().unwrap();
    let end: i64 = parts[1].parse().unwrap();
    (start, end)
}
