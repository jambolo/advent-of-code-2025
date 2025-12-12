// Advent of Code 2025, Day 1

use common::load;

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
    let mut history = instrumentation::Recording {
        frames: Vec::new(),
        rotation_number: 0,
        total_rotations: 0,
        progress: 0.0,
    };

    #[cfg(feature = "instrumented")]
    instrumentation::record_initial(&mut history, lines.len(), current_position, password);
    
    for line in lines {
        // Parse the direction and distance. Format is e.g. "R2", "L3".
        let (turn, distance_str) = line.split_at(1);
        let distance = distance_str.parse::<i32>().expect("Invalid distance");
        let full_turns = distance / P;
        let remainder = distance % P;

        #[cfg(feature = "instrumented")]
        instrumentation::record_rotation_start(
            &mut history,
            current_position,
            password,
            &line,
            distance,
            turn,
        );

        // In part 2, count the number of times we pass position 0 (but not land on it)
        if cfg!(feature = "part2") {
            #[cfg(feature = "instrumented")]
            instrumentation::record_zero_passes(&mut history, full_turns, password, &line, distance, turn);
            password += full_turns;
            match turn {
                "R" if current_position > P - remainder => {
                    #[cfg(feature = "instrumented")]
                    instrumentation::record_zero_passes(&mut history, 1, password, &line, distance, turn);
                    password += 1;
                },
                "L" if (0 < current_position) && (current_position < remainder) => {
                    #[cfg(feature = "instrumented")]
                    instrumentation::record_zero_passes(&mut history, 1, password, &line, distance, turn);
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
                    &mut history,
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
                &mut history,
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
    instrumentation::record_final(&mut history, current_position, password);

    #[cfg(not(feature = "instrumented"))]
    println!("The password is: {}", password);

    #[cfg(feature = "instrumented")]
    instrumentation::log_to_console(&history, password);
}

#[cfg(feature = "instrumented")]
mod instrumentation {
    use serde_json::json;

    pub struct Recording {
        pub frames: Vec<serde_json::Value>,
        pub rotation_number: usize,
        pub total_rotations: usize,
        pub progress: f64,
    }


    pub fn record_initial(history: &mut Recording, total_rotations: usize, position: i32, password: i32) {
        history.total_rotations = total_rotations;
        history.frames.push(json!({
            "frame_type": "initial",
            "rotation_number": history.rotation_number,
            "position": position,
            "password": password,
            "progress": history.progress
        }));
    }

    pub fn record_rotation_start(history: &mut Recording, position: i32, password: i32, instruction: &str, distance: i32, direction: &str) {
        history.rotation_number += 1;
        history.progress = (history.rotation_number as f64) / (history.total_rotations as f64);
        history.frames.push(json!({
            "frame_type": "rotation_start",
            "rotation_number": history.rotation_number,
            "position": position,
            "password": password,
            "instruction": instruction,
            "distance": distance,
            "direction": direction,
            "progress": history.progress
        }));
    }

    pub fn record_zero_passes(history: &mut Recording, n: i32, password: i32, instruction: &str, distance: i32, direction: &str) {
        for i in 0..n {
            history.frames.push(json!({
                "frame_type": "zero_pass",
                "rotation_number": history.rotation_number,
                "position": 0,
                "password": password + i + 1,
                "instruction": instruction,
                "distance": distance,
                "direction": direction,
                "progress": history.progress
            }));
        }
    }

    pub fn record_zero_land(
        history: &mut Recording,
        position: i32,
        password: i32,
        instruction: &str,
        distance: i32,
        direction: &str,
        passes_in_rotation: i32,
    ) {
        history.frames.push(json!({
            "frame_type": "zero_land",
            "rotation_number": history.rotation_number,
            "position": position,
            "password": password,
            "instruction": instruction,
            "distance": distance,
            "direction": direction,
            "lands_on_zero": true,
            "passes_in_rotation": passes_in_rotation,
            "progress": history.progress
        }));
    }

    pub fn record_rotation_end(
        history: &mut Recording,
        position: i32,
        password: i32,
        instruction: &str,
        distance: i32,
        direction: &str,
        passes_in_rotation: i32,
    ) {
        history.frames.push(json!({
            "frame_type": "rotation_end",
            "rotation_number": history.rotation_number,
            "position": position,
            "password": password,
            "instruction": instruction,
            "distance": distance,
            "direction": direction,
            "lands_on_zero": false,
            "passes_in_rotation": passes_in_rotation,
            "progress": history.progress
        }));
    }

    pub fn record_final(history: &mut Recording, position: i32, password: i32) {
        history.frames.push(json!({
            "frame_type": "final",
            "rotation_number": history.rotation_number,
            "position": position,
            "password": password,
            "progress": history.progress
        }));
    }

    pub fn log_to_console(history: &Recording, password: i32) {
        let log = json!({
            "frames": history.frames,
            "total_rotations": history.total_rotations,
            "final_password": password
        });
        println!("{}", log.to_string());
    }
}
