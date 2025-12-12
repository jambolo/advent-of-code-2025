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
    let count = ingredient_ids
        .iter()
        .filter(|&&id| is_fresh(fresh_ranges, id))
        .count();
    println!("Number of fresh ingredients: {}", count);
}

fn is_fresh(fresh_ranges: &[(i64, i64)], id: i64) -> bool {
    fresh_ranges.iter().any(|&(start, end)| id >= start && id <= end)
}

fn part2(ranges: &[(i64, i64)]) {
    let mut sorted = ranges.to_vec();
    sorted.sort_unstable_by_key(|&(start, _)| start);

    let mut merged: Vec<(i64, i64)> = Vec::new();
    for (start, end) in sorted {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 {
                last.1 = last.1.max(end);  // Merge overlapping ranges
                continue; // Don't push a new range
            }
            merged.push((start, end));
        }
    }
    let total_fresh: i64 = merged.iter().map(|&(start, end)| end - start + 1).sum();
    println!("Total number of fresh ingredients: {}", total_fresh);
}

fn parse_range(line: &str) -> (i64, i64) {
    let (start, end) = line.split_once('-').unwrap();
    (start.parse().unwrap(), end.parse().unwrap())
}
