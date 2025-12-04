// Advent of Code 2025, Day 4

use common::load;

fn main() {
    println!("Day 4, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    let map = load::map();

    if cfg!(feature = "part2") {
        part2(&map);
    } else {
        part1(&map);
    }
}

fn part1(map: &Vec<Vec<char>>) {
    let height = map.len();
    let width = map[0].len();
    // Count the number of cells with less than 4 neighbors
    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == '@' {
                let neighbors = count_neighbors(&map, x, y);
                if neighbors < 4 {
                    count += 1;
                }
            }
        }
    }
    println!("Cells with less than 4 neighbors: {}", count);
}

fn count_neighbors(map: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut neighbors = 0;
    let height = map.len();
    let width = map[0].len();

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx != 0 || dy != 0 {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                    if map[ny as usize][nx as usize] == '@' {
                        neighbors += 1;
                    }
                }
            }
        }
    }
    neighbors
}

fn part2(map: &Vec<Vec<char>>) {
    let mut new_map = map.clone();
    let height = new_map.len();
    let width = new_map[0].len();
    // Let's try the naive approach
    let mut removed = 0;
    loop {
        let previous_removed = removed;
        for y in 0..height {
            for x in 0..width {
                if new_map[y][x] == '@' {
                    if count_neighbors(&new_map, x, y) < 4 {
                        new_map[y][x] = '.';
                        removed += 1;
                    }
                }
            }
        }
        if removed == previous_removed {
            break;
        }
    }
    println!("Total cells removed_this_pass: {}", removed);
}