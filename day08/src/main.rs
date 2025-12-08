// Advent of Code 2025, Day 8

use common::{load};

fn main() {
    println!("Day 8, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    // Load the locations from the input file.
    let lines = load::lines();
    let locations: Vec<(i64, i64, i64)> = lines.iter().map(|line| {
        let mut parts = line.split(',');
        let x: i64 = parts.next().unwrap().parse().unwrap();
        let y: i64 = parts.next().unwrap().parse().unwrap();
        let z: i64 = parts.next().unwrap().parse().unwrap();
        (x, y, z)
    }).collect();

    // Compute the distances between all pairs of locations.
    let mut distances: Vec<((usize, usize), f64)> = Vec::new();
    for i in 0..locations.len() - 1 {
        let (x1, y1, z1) = locations[i];
        for j in i + 1..locations.len() {
            let (x2, y2, z2) = locations[j];
            let dx = (x2 - x1) as f64;
            let dy = (y2 - y1) as f64;
            let dz = (z2 - z1) as f64;
            let distance = (dx * dx + dy * dy + dz * dz).sqrt();
            distances.push(((i, j), distance));
        }
    }

    // Sort distances.
    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // Create the initial list of circuits.
    let mut circuits: Vec<Vec<usize>> = Vec::new();
    for i in 0..locations.len() {
        circuits.push(vec![i]);
    }

    // Connect the closest N junction boxes
    let n = 1000; // Number of boxes to connect
    for i in 0..n {
        connect(&mut circuits, distances[i].0);
    }

    // Create a sorted list of circuit sizes
    let mut circuit_sizes: Vec<usize> = Vec::new();
    for c in circuits.iter() {
        circuit_sizes.push(c.len());
    }
    circuit_sizes.sort();

    // Print the product of the sizes of the three largest circuits
    let m = circuit_sizes.len();
    let product = circuit_sizes[m - 1] * circuit_sizes[m - 2] * circuit_sizes[m - 3];
    println!("Result: {}", product);
}

fn connect(circuits: &mut Vec<Vec<usize>>, pair: (usize, usize)) {
    let from = pair.0;
    let to = pair.1;
    let cf = containing_circuit(circuits, from).unwrap();
    let ct = containing_circuit(circuits, to).unwrap();
    // If junctions are in different circuits, then merge the circuits. Otherwise, do nothing.
    if cf != ct {
        // Merge circuits
        let mut ct_clone = circuits[ct].clone();
        circuits[cf].append(&mut ct_clone);
        circuits.remove(ct);
    }
}

fn containing_circuit(circuits: &Vec<Vec<usize>>, junction: usize) -> Option<usize> {
    for (i, c) in circuits.iter().enumerate() {
        if c.contains(&junction) {
            return Some(i);
        }
    }
    None
}