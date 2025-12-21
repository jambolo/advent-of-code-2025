// Advent of Code 2025, Day 9

use common::load;

fn main() {
    println!("Day 9, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    // Load the locations of the corners.
    let corners: Vec<(usize, usize)> = load::lines()
        .iter()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

        // List of edges of the region with normalized vertex order.
    let edges: Vec<_> = consecutive_pairs(&corners)
        .map(|(v0, v1)| if v0.0 < v1.0 || v0.1 < v1.1 { (*v0, *v1) } else { (*v1, *v0) })
        .collect();

    // List of horizontal edges with normalized vertex order.
    let horizontal_edges: Vec<_> = edges
        .iter()
        .filter(|(v0, v1)| v0.1 == v1.1)
        .copied()
        .collect();

    // Create a list of all possible rectangles and their areas.
    let mut rectangles = Vec::new();
    for i in 0..corners.len() - 1 {
        let (x0, y0) = corners[i];
        for j in i + 1..corners.len() {
            let (x1, y1) = corners[j];
            let area = ((x1 as i64 - x0 as i64).abs() + 1) * ((y1 as i64 - y0 as i64).abs() + 1);
            if cfg!(feature = "part2") {
                if fully_contained(&edges, &horizontal_edges, &(corners[i], corners[j])) {
                    rectangles.push(((i, j), area));
                }
            } else {
                rectangles.push(((i, j), area));
            }
        }
    }

    // Sort rectangles by area.
    rectangles.sort_by_key(|&(_, area)| area);

    // Output the area of the largest rectangle.
    println!("Largest area: {}", rectangles.last().unwrap().1);
}

fn crosses(rect: &((usize, usize), (usize, usize)), v0: &(usize, usize), v1: &(usize, usize)) -> bool {
    let (x0, y0) = *v0;
    let (x1, y1) = *v1;
    let ((cx0, cy0), (cx1, cy1)) = *rect;
    let (rx0, rx1) = (cx0.min(cx1), cx0.max(cx1));
    let (ry0, ry1) = (cy0.min(cy1), cy0.max(cy1));

    if x0 <= rx0 && rx0 <= x1 && ry0 < y0 && y0 < ry1 {
        return true;
    }
    if x0 <= rx1 && rx1 <= x1 && ry0 < y0 && y0 < ry1 {
        return true;
    }
    if y0 <= ry0 && ry0 <= y1 && rx0 < x0 && x0 < rx1 {
        return true;
    }
    if y0 <= ry1 && ry1 <= y1 && rx0 < x0 && x0 < rx1 {
        return true;
    }
    false
}

fn fully_contained(
    edges: &[((usize, usize), (usize, usize))],
    horizontal_edges: &[((usize, usize), (usize, usize))],
    rect: &((usize, usize), (usize, usize))
) -> bool {
    // Check that no region edges cross any of the rect edges.
    if edges.iter().any(|(v0, v1)| crosses(rect, v0, v1)) {
        return false;
    }

    // A single row or column is always contained.
    
    // Check that an interior point of the rectangle is inside the region.
    let test_x = rect.0.0.min(rect.1.0) as f64 + 0.5;
    let test_y = rect.0.1.min(rect.1.1) as f64 + 0.5;

    // Cast a vertical ray downward and count crossings with horizontal edges
    let crossings = horizontal_edges
        .iter()
        .filter(|(v0, v1)| {
            let edge_y = v0.1 as f64;
            let v0_x = v0.0 as f64;
            let v1_x = v1.0 as f64;
            // Check if the ray crosses this horizontal edge
            edge_y > test_y && v0_x < test_x && test_x < v1_x
        })
        .count();

    crossings % 2 == 1
}

fn consecutive_pairs<T>(slice: &[T]) -> impl Iterator<Item = (&T, &T)> {
    slice.iter().zip(slice.iter().cycle().skip(1)).take(slice.len())
}