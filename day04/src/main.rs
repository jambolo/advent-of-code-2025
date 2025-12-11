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

fn part1(map: &[Vec<char>]) {
    let count = map.iter().enumerate()
        .map(|(y, row)| row.iter().enumerate()
            .filter(|(x, cell)| **cell == '@' && count_neighbors(map, *x, y) < 4)
            .count()
        )
        .sum::<usize>();
    println!("Cells with less than 4 neighbors: {}", count);
}

fn count_neighbors(map: &[Vec<char>], x: usize, y: usize) -> usize {
    neighboring_cells(map, x, y).filter(|&c| c == '@').count()
}

fn neighboring_cells(map: &[Vec<char>], x: usize, y: usize) -> impl Iterator<Item = char> + '_ {
    let height = map.len();
    let width = map[0].len();
    let x0 = x.saturating_sub(1);
    let y0 = y.saturating_sub(1);
    let x1 = (x + 1).min(width - 1);
    let y1 = (y + 1).min(height - 1);
    (y0..=y1).flat_map(move |ny| {
        (x0..=x1)
            .filter(move |&nx| nx != x || ny != y)
            .map(move |nx| map[ny][nx])
    })
}

fn part2(map: &[Vec<char>]) {
    let mut new_map = map.to_vec();
    let height = new_map.len();
    let width = new_map[0].len();
    // Let's try the naive approach
    let mut removed = 0;
    loop {
        let previous_removed = removed;
        for y in 0..height {
            for x in 0..width {
                if new_map[y][x] == '@' && count_neighbors(&new_map, x, y) < 4 {
                    new_map[y][x] = '.';
                    removed += 1;
                }
            }
        }
        if removed == previous_removed {
            break;
        }
    }
    println!("Total cells removed: {}", removed);
}