// Advent of Code 2025, Day 7

use common::load;

fn main() {
    println!("Day 7, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    let map = load::map();

    if cfg!(feature = "part2") {
        part2(map);
    } else {
        part1(map);
    }
}

fn part1(map: Vec<Vec<char>>) {
    use std::collections::HashSet;

    let width = map[0].len();

    // List of columns with beams
    let mut beams = HashSet::new();

    // Number of splitters encountered
    let mut count: i64 = 0;

    // Beams move down the map
    for row in map {
        // Search for a source before doing anything else
        if beams.is_empty() {
            if let Some(x) = row.iter().position(|&c| c == 'S') {
                beams.insert(x);
            }
            continue;
        }

        for x in beams.iter().cloned().collect::<Vec<_>>() {
            if row[x] == '^' {
                count += 1;
                beams.remove(&x);
                if x > 0 {
                    beams.insert(x - 1);
                }
                if x + 1 < width {
                    beams.insert(x + 1);
                }
            }
        }
    }
    println!("Number of splitters encountered: {}", count);
}

fn part2(map: Vec<Vec<char>>) {
    use std::collections::HashMap;

    let width = map[0].len();

    // List of timelines in the form of beams columns and their timeline counts
    let mut timelines: HashMap<usize, i64> = HashMap::new();

    // Beams move down the map
    for row in map {
        // Search for a source before doing anything else
        if timelines.is_empty() {
            if let Some(x) = row.iter().position(|&c| c == 'S') {
                timelines.insert(x, 1);
            }
            continue;
        }

        // Scan each beam column for splitters in this row
        for x in timelines.keys().cloned().collect::<Vec<_>>() {
            if row[x] == '^' {
                let count = timelines.remove(&x).unwrap();
                if x > 0 {
                    *timelines.entry(x - 1).or_insert(0) += count;
                }
                if x < width - 1 {
                    *timelines.entry(x + 1).or_insert(0) += count;
                }
            }
        }
    }

    // Count the total number of timelines
    let final_timelines: i64 = timelines.values().sum();
    println!("Total number of timelines: {}", final_timelines);
}
