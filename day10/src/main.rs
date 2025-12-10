// Advent of Code 2025, Day 10

use common::load;

#[cfg(not(feature = "part2"))]
mod part1;

#[cfg(feature = "part2")]
mod part2;

fn main() {
    println!("Day 10, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    let lines = load::lines();

    #[cfg(feature = "part2")]
    {
        let machines: Vec<part2::Machine> = lines.iter().map(|line| part2::Machine::new(line)).collect();
        part2::part2(machines);
    }

    #[cfg(not(feature = "part2"))]
    {
        let machines: Vec<part1::Machine> = lines.iter().map(|line| part1::parse_machine(line)).collect();
        part1::part1(machines);
    }
}
