// Advent of Code, 2025, Day 1

use common::load;

const STARTING_POSITION: i32 = 50;
const P: i32 = 100;

fn main() {
    println!("Day 1, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    // Load the data
    let lines = load::lines();

    let mut current_position = STARTING_POSITION;
    let mut password = 0;
    for line in lines {
        // Parse the direction and distance. Format is e.g. "R2", "L3".
        let (turn, distance_str) = line.split_at(1);
        let distance = distance_str.parse::<i32>().expect("Invalid distance");
        let full_turns = distance / P;
        let remainder = distance % P;

        // In part 2, count the number of times we pass position 0 (but not land on it)
        if cfg!(feature = "part2") {
            password += full_turns;
            match turn {
                "R" if current_position > P - remainder => password += 1,
                "L" if (0 < current_position) && (current_position < remainder) => password += 1,
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
            password += 1;
        }
    }
    println!("The password is: {}", password);
}
