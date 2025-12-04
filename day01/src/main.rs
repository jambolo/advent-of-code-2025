// Day 1
//

use common::load;

const STARTING_POSITION: i32 = 50;
const P: i32 = 100;

fn main() {
    println!("Day 1, part {}", if cfg!(feature="part2") { "2" } else { "1" });

    // Load the data
    let lines = load::lines();
    println!("Loaded {} lines of data.", lines.len());

    let mut current_position = STARTING_POSITION;
    let mut password = 0;
    for line in lines {
        // Parse the direction and distance. Format is e.g. "R2", "L3".
        let (turn, distance_str) = line.split_at(1);
        let distance: i32 = distance_str.parse::<i32>().expect("Invalid distance") % P;

        match turn {
            "R" => current_position = (current_position + distance) % P,
            "L" => current_position = (current_position + P - distance) % P,
            _ => panic!("Invalid turn direction: {}", turn),
        }

        // Count the number of times the position is 0
        if current_position == 0 {
            password += 1;
        }

        // Otherwise, part 2
    }
    println!("The password is: {}", password);
}