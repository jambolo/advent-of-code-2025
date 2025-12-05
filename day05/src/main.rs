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

    if cfg!(feature = "part2") {
        part2(&fresh_ranges);
    } else {
        part1(&fresh_ranges, &ingredient_ids);
    }
}

fn part1(fresh_ranges: &[(i64, i64)], ingredient_ids: &[i64]) {
    // Determine which ingredient IDs are fresh
    let fresh_ingredients: Vec<i64> = ingredient_ids
        .iter()
        .filter(|&id| {
            fresh_ranges.iter().any(|&(start, end)| *id >= start && *id <= end)
        })
        .cloned()
        .collect();
    // Output the result
    println!("Number of fresh ingredients: {}", fresh_ingredients.len());
}

fn part2(ranges: &[(i64, i64)]) {
    // Sort ranges by start value
    let mut sorted_ranges: Vec<(i64, i64)> = ranges.to_vec();
    sorted_ranges.sort_by_key(|&(start, _)| start);

    // Merge overlapping ranges
    let mut merged_ranges: Vec<(i64, i64)> = Vec::new();
    for &(start, end) in &sorted_ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if start <= last.1 {
                last.1 = last.1.max(end);
                continue;
            }
        }
        merged_ranges.push((start, end));
    }

    // Calculate total fresh ingredient count
    let total_fresh: i64 = merged_ranges.iter().map(|&(start, end)| end - start + 1).sum();

    // Output the result
    println!("Total number of fresh ingredients: {}", total_fresh);
}

fn parse_range(line: &str) -> (i64, i64) {
    let parts: Vec<&str> = line.split('-').collect();
    let start: i64 = parts[0].parse().unwrap();
    let end: i64 = parts[1].parse().unwrap();
    (start, end)
}
