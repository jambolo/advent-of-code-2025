// Advent of Code 2025, Day 7

use common::load;

fn main() {
    println!("Day 7, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    let map = load::map();
    let height = map.len();
    let width = map[0].len();

    if cfg!(feature = "part2") {
        use std::collections::HashMap;

        // List of timelines in the form of beams columns and their timeline counts
        let mut timelines: HashMap<usize, i64> = HashMap::new();

        // Beams move down the map
        for y in 0..height {
            // Search for a source before doing anything else
            if timelines.is_empty() {
                for x in 0..width {
                    if map[y][x] == 'S' {
                        timelines.insert(x, 1);
                        break;
                    }
                }
                continue;
            }

            // Scan each beam column for a splitter
            let timeline_columns: Vec<usize> = timelines.keys().cloned().collect();
            for x in timeline_columns {
                if map[y][x] == '^' {
                    let count = timelines[&x];
                    // Split the timeline
                    timelines.remove(&x);
                    if x > 0 {
                        *timelines.entry(x - 1).or_insert(0) += count;                    }
                    if x + 1 < width {
                        *timelines.entry(x + 1).or_insert(0) += count;                    }
                }
            }
        }

        // Count the total number of timelines
        let final_timelines: i64 = timelines.values().sum();
        println!("Total number of timelines: {}", final_timelines);
    } else {
        use std::collections::HashSet;

        // List of columns with beams
        let mut beams = HashSet::new();

        // Number of splitters encountered
        let mut count: i64 = 0;

        // Beams move down the map
        for y in 0..height {
            // Search for a source before doing anything else
            if beams.is_empty() {
                for x in 0..width {
                    if map[y][x] == 'S' {
                        beams.insert(x);
                        break;
                    }
                }
                continue;
            }

            // Scan each beam column for a splitter
            let beam_columns: Vec<usize> = beams.iter().cloned().collect();
            for x in beam_columns {
                if map[y][x] == '^' {
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
}
