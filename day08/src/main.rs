// Advent of Code 2025, Day 8

use common::load;

#[cfg(feature = "instrumented")]
use instrumentation::Instrumentation;

fn main() {
    #[cfg(not(feature = "instrumented"))]
    println!(
        "Day 8, part {}",
        if cfg!(feature = "part2") { "2" } else { "1" }
    );

    // Load the locations from the input file.
    let lines = load::lines();
    let locations: Vec<(i64, i64, i64)> = lines
        .iter()
        .map(|line| {
            let mut parts = line.split(',');
            let x: i64 = parts
                .next()
                .expect("Missing x coordinate")
                .parse()
                .expect("Failed to parse x coordinate as i64");
            let y: i64 = parts
                .next()
                .expect("Missing y coordinate")
                .parse()
                .expect("Failed to parse y coordinate as i64");
            let z: i64 = parts
                .next()
                .expect("Missing z coordinate")
                .parse()
                .expect("Failed to parse z coordinate as i64");
            (x, y, z)
        })
        .collect();

    // Compute the distances between all pairs of locations.
    let mut distances: Vec<((usize, usize), f64)> = Vec::new();
    for i in 0..locations.len() - 1 {
        let (x1, y1, z1) = locations[i];
        for (j, (x2, y2, z2)) in locations.iter().enumerate().skip(i + 1) {
            let dx = (*x2 - x1) as f64;
            let dy = (*y2 - y1) as f64;
            let dz = (*z2 - z1) as f64;
            let distance = (dx * dx + dy * dy + dz * dz).sqrt();
            distances.push(((i, j), distance));
        }
    }

    // Sort distances.
    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).expect("Failed to compare distances"));

    // Create the initial list of circuits.
    let mut circuits: Vec<Vec<usize>> = Vec::new();
    for i in 0..locations.len() {
        circuits.push(vec![i]);
    }

    if cfg!(feature = "part2") {
        #[cfg(feature = "instrumented")]
        let mut inst = Instrumentation::new(&locations);

        // Connect junction boxes until all are connected
        let mut index = 0;
        while circuits.len() > 1 {
            let merged = connect(&mut circuits, distances[index].0);
            #[cfg(not(feature = "instrumented"))]
            let _ = merged; // Suppress unused variable warning

            #[cfg(feature = "instrumented")]
            if merged {
                inst.record_connection(
                    distances[index].0,
                    distances[index].1,
                    &circuits,
                    circuits.len() == 1,
                );
            }

            index += 1;
        }
        // Print the product of the x coordinates of the last connected connection
        let connection = distances[index - 1].0;
        let from = connection.0;
        let to = connection.1;
        let result = locations[from].0 * locations[to].0;
        #[cfg(not(feature = "instrumented"))]
        println!("Result: {}", result);

        #[cfg(feature = "instrumented")]
        inst.finalize(from, to, locations[from].0, locations[to].0, result);
    } else {
        // Connect the closest N junction boxes
        let n = 1000; // Number of boxes to connect
        for distance in distances.iter().take(n) {
            connect(&mut circuits, distance.0);
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
}

fn connect(circuits: &mut Vec<Vec<usize>>, connection: (usize, usize)) -> bool {
    let from = connection.0;
    let to = connection.1;
    let cf = containing_circuit(circuits, from).expect("From junction not found in any circuit");
    let ct = containing_circuit(circuits, to).expect("To junction not found in any circuit");
    // If junctions are in different circuits, then merge the circuits. Otherwise, do nothing.
    if cf != ct {
        // Merge circuits
        let mut ct_clone = circuits[ct].clone();
        circuits[cf].append(&mut ct_clone);
        circuits.remove(ct);
        true
    } else {
        false
    }
}

fn containing_circuit(circuits: &[Vec<usize>], junction: usize) -> Option<usize> {
    circuits.iter().position(|c| c.contains(&junction))
}

#[cfg(feature = "instrumented")]
mod instrumentation {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct JunctionBox {
        x: i64,
        y: i64,
        z: i64,
    }

    #[derive(Serialize)]
    pub struct Frame {
        frame_type: String,
        connection_index: usize,
        from_idx: usize,
        to_idx: usize,
        distance: f64,
        circuits_remaining: usize,
        circuit_assignments: Vec<usize>,
    }

    #[derive(Serialize)]
    struct LogData {
        boxes: Vec<JunctionBox>,
        total_connections_needed: usize,
        frames: Vec<Frame>,
        final_from_idx: usize,
        final_to_idx: usize,
        final_from_x: i64,
        final_to_x: i64,
        answer: i64,
    }

    pub struct Instrumentation {
        boxes: Vec<JunctionBox>,
        frames: Vec<Frame>,
        num_boxes: usize,
        connection_count: usize,
    }

    impl Instrumentation {
        pub fn new(locations: &[(i64, i64, i64)]) -> Self {
            let boxes: Vec<JunctionBox> = locations
                .iter()
                .map(|(x, y, z)| JunctionBox {
                    x: *x,
                    y: *y,
                    z: *z,
                })
                .collect();
            let num_boxes = boxes.len();

            // Create initial circuit assignments (each box in its own circuit)
            let initial_assignments: Vec<usize> = (0..num_boxes).collect();

            let initial_frame = Frame {
                frame_type: "initial".to_string(),
                connection_index: 0,
                from_idx: 0,
                to_idx: 0,
                distance: 0.0,
                circuits_remaining: num_boxes,
                circuit_assignments: initial_assignments,
            };

            Self {
                boxes,
                frames: vec![initial_frame],
                num_boxes,
                connection_count: 0,
            }
        }

        pub fn record_connection(
            &mut self,
            connection: (usize, usize),
            distance: f64,
            circuits: &[Vec<usize>],
            is_final: bool,
        ) {
            self.connection_count += 1;

            // Build circuit assignments array
            let mut circuit_assignments = vec![0; self.num_boxes];
            for (circuit_id, circuit) in circuits.iter().enumerate() {
                for &box_idx in circuit {
                    circuit_assignments[box_idx] = circuit_id;
                }
            }

            let frame_type = if is_final { "final" } else { "connection" };

            let frame = Frame {
                frame_type: frame_type.to_string(),
                connection_index: self.connection_count,
                from_idx: connection.0,
                to_idx: connection.1,
                distance,
                circuits_remaining: circuits.len(),
                circuit_assignments,
            };

            self.frames.push(frame);
        }

        pub fn finalize(
            self,
            final_from_idx: usize,
            final_to_idx: usize,
            final_from_x: i64,
            final_to_x: i64,
            answer: i64,
        ) {
            let log_data = LogData {
                total_connections_needed: self.num_boxes - 1,
                boxes: self.boxes,
                frames: self.frames,
                final_from_idx,
                final_to_idx,
                final_from_x,
                final_to_x,
                answer,
            };

            println!("{}", serde_json::to_string(&log_data).unwrap());
        }
    }
}
