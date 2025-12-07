// Advent of Code 2025, Day 7

use std::collections::HashSet;
use common::load;

fn main() {
    println!("Day 7, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    let map = load::map();
    let height = map.len();
    let width = map[0].len();

    // List of columns with beams
    let mut beams = HashSet::new();

    // Number of splitters encountered
    let mut count = 0;

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

        // Scan each beam column for count
        let mut beams_copy = beams.clone();
        for x in beams {
            if map[y][x] == '^' {
                count += 1;
                beams_copy.remove(&x);
                if x > 0 {
                    beams_copy.insert(x - 1);
                }
                if x + 1 < width {
                    beams_copy.insert(x + 1);
                }
            }
        }
        beams = beams_copy;
    }

    println!("Number of splitters encountered: {}", count);
}
