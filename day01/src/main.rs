// Advent of Code 2025, Day 1

use common::load;
use serde_json::json;

const STARTING_POSITION: i32 = 50;
const P: i32 = 100;

fn main() {
    #[cfg(not(feature = "instrumented"))]
    println!("Day 1, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    // Load the data
    let lines = load::lines();

    let mut current_position = STARTING_POSITION;
    let mut password = 0;

    #[cfg(feature = "instrumented")]
    instrumentation::record_initial(lines.len(), current_position, password);
    
    for line in lines {
        // Parse the direction and distance. Format is e.g. "R2", "L3".
        let (turn, distance_str) = line.split_at(1);
        let distance = distance_str.parse::<i32>().expect("Invalid distance");
        let full_turns = distance / P;
        let remainder = distance % P;

        #[cfg(feature = "instrumented")]
        instrumentation::record_rotation_start(
            current_position,
            password,
            &line,
            distance,
            turn,
        );

        // In part 2, count the number of times we pass position 0 (but not land on it)
        if cfg!(feature = "part2") {
            #[cfg(feature = "instrumented")]
            instrumentation::record_zero_passes(full_turns, password, &line, distance, turn);
            password += full_turns;
            match turn {
                "R" if current_position > P - remainder => {
                    #[cfg(feature = "instrumented")]
                    instrumentation::record_zero_passes(1, password, &line, distance, turn);
                    password += 1;
                },
                "L" if (0 < current_position) && (current_position < remainder) => {
                    #[cfg(feature = "instrumented")]
                    instrumentation::record_zero_passes(1, password, &line, distance, turn);
                    password += 1;
                },
                "R" | "L" => {},
                _ => panic!("Invalid turn direction: {}", turn),
            }
        }

        current_position = match turn {
            "R" => (current_position + remainder) % P,
            "L" => (current_position + P - remainder) % P,
            _ => panic!("Invalid turn direction: {}", turn),
        };

        // Count the number of times the position is 0
        if current_position == 0 {
                #[cfg(feature = "instrumented")]
                instrumentation::record_zero_land(
                    current_position,
                    password,
                    &line,
                    distance,
                    turn,
                    full_turns + 1,
                );
            password += 1;
        } else {
            #[cfg(feature = "instrumented")]
            instrumentation::record_rotation_end(
                current_position,
                password,
                &line,
                distance,
                turn,
                full_turns,
            );
        }
    }

    #[cfg(feature = "instrumented")]
    instrumentation::record_final(current_position, password);

    #[cfg(not(feature = "instrumented"))]
    println!("The password is: {}", password);

    #[cfg(feature = "instrumented")]
    instrumentation::log_to_console(password);
}

#[cfg(feature = "instrumented")]
mod instrumentation {

    static mut frames: Vec<serde_json::Value> = Vec::new();
    static mut rotation_number: usize = 0;
    static mut total_rotations: usize = 1;
    static mut progress: f64 = 0.0;

    pub fn record_initial(max_rotations: usize, position: i32, password: i32) {
        unsafe {
            total_rotations = max_rotations;
            frames.push(json!({
                "frame_type": "initial",
                "rotation_number": rotation_number,
                "position": position,
                "password": password,
                "progress": progress
            }));
        }
    }

    pub fn record_rotation_start(position: i32, password: i32, instruction: &str, distance: i32, direction: &str) {
        unsafe {
            rotation_number += 1;
            progress = (rotation_number as f64) / (total_rotations as f64);
            frames.push(json!({
                "frame_type": "rotation_start",
                "rotation_number": rotation_number,
                "position": position,
                "password": password,
                "instruction": instruction,
                "distance": distance,
                "direction": direction,
                "progress": progress
            }));
        }
    }

    pub fn record_zero_passes(n: i32, password: i32, instruction: &str, distance: i32, direction: &str) {
        unsafe {
            for i in 0..n {
                frames.push(json!({
                    "frame_type": "zero_pass",
                    "position": 0,
                    "password": password + i + 1,
                    "instruction": instruction,
                    "distance": distance,
                    "direction": direction,
                    "progress": progress
                }));
            }
        }
    }

    pub fn record_zero_land(
        position: i32,
        password: i32,
        instruction: &str,
        distance: i32,
        direction: &str,
        passes_in_rotation: i32,
    ) {
        unsafe {
            frames.push(json!({
                "frame_type": "zero_land",
                "rotation_number": rotation_number,
                "position": position,
                "password": password,
                "instruction": instruction,
                "distance": distance,
                "direction": direction,
                "lands_on_zero": true,
                "passes_in_rotation": passes_in_rotation,
                "progress": progress
            }));
        }
    }

    pub fn record_rotation_end(
        position: i32,
        password: i32,
        instruction: &str,
        distance: i32,
        direction: &str,
        passes_in_rotation: i32,
    ) {
        unsafe {
            frames.push(json!({
                "frame_type": "rotation_end",
                "rotation_number": rotation_number,
                "position": position,
                "password": password,
                "instruction": instruction,
                "distance": distance,
                "direction": direction,
                "lands_on_zero": false,
                "passes_in_rotation": passes_in_rotation,
                "progress": progress
            }));
        }
    }

    pub fn record_final(position: i32, password: i32) {
        unsafe {
            frames.push(json!({
                "frame_type": "final",
                "rotation_number": rotation_number,
                "position": position,
                "password": password,
                "progress": progress
            }));
        }
    }

    pub fn log_to_console(password: i32) {
        unsafe {
            let log = json!({
                "frames": frames,
                "total_rotations": rotation_number,
                "final_password": password
            });
            println!("{}", log.to_string());
        }
    }
}