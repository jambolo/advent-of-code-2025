// Advent of Code 2025, Day 9

use common::load;

fn main() {
    println!("Day 9, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    // Load the locations of the corners.
    let lines = load::lines();
    let corners: Vec<(i64, i64)> = lines.iter().map(|line| {
        let mut parts = line.split(',');
        let x: i64 = parts.next().unwrap().parse().unwrap();
        let y: i64 = parts.next().unwrap().parse().unwrap();
        (x, y)
    }).collect();

    // Create a list of all possible rectangles and their areas.
    let mut rectangles: Vec<((usize, usize), i64)> = Vec::new();
    for i in 0..corners.len() - 1 {
        let (x0, y0) = corners[i];
        for j in i + 1..corners.len() {
            let (x1, y1) = corners[j];
            let width = (x1 - x0).abs() + 1;
            let height = (y1 - y0).abs() + 1;
            let area = width * height;
            rectangles.push(((i, j), area));
        }
    }

    // Sort rectangles by area.
    rectangles.sort_by_key(|&(_, area)| area);

    // Output the area of the largest rectangle.
    let &((_, _), largest_area) = rectangles.last().unwrap();
    println!("Largest area: {}", largest_area);
}
