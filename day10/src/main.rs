// Advent of Code 2025, Day 10

use common::load;
use itertools::Itertools;

#[cfg(not(feature = "part2"))]
#[derive(Debug)]
struct Machine {
    final_state: u64,   // binary representation of final state
    buttons: Vec<u64>,       // binary representation of button effects
}

#[cfg(feature = "part2")]
#[derive(Debug)]
struct Machine {
    buttons: Vec<Vec<usize>>, // Indexes of joltage values
    joltages: Vec<i64>,
}

fn main() {
    println!("Day 10, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    let lines = load::lines();
    let machines: Vec<Machine> = lines.iter().map(|line| parse_machine(line)).collect();

    #[cfg(feature = "part2")]
    part2(machines);

    #[cfg(not(feature = "part2"))]
    part1(machines);
}

#[cfg(feature = "part2")]
fn part2(machines: Vec<Machine>) {
    let mut sum: i64 = 0;

    for machine in machines {
        let max_presses = machine.joltages.iter().sum::<i64>();
        let min_presses = *machine.joltages.iter().max().unwrap();
        let j = machine.joltages.len();
        let mut found = false;

        // Try all combinations of 1..=n buttons. Stop when the first successful combination is found.
        for k in min_presses..=max_presses {
            let pool = create_button_press_pool(&machine.joltages, &machine.buttons);
            let combinations = pool.iter().combinations(k as usize);
            for combo in combinations {
//                println!("  Trying combination: {:?}", combo);
                let mut result = vec![0i64; j];
                for b in combo {
                    increase_joltages(&mut result, &machine.buttons[*b]);
                }
                if result == machine.joltages {
                    sum += k as i64;
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        if !found {
            panic!("No valid combination of buttons found for machine: {:?}", machine);
        }
    }
    println!("Sum: {}", sum);
}

// Create a vector consisting of each button index repeated for the maximum number of presses
#[cfg(feature = "part2")]
fn create_button_press_pool(joltages: &[i64], buttons: &[Vec<usize>]) -> Vec<usize> {
    let mut pool = Vec::new();
    for (i, b) in buttons.iter().enumerate() {
        // Determine the maximum number of times this button can be pressed without exceeding any joltage
        let button_max = b.iter().map(|&index| joltages[index] as usize).min().unwrap();

        // Add the button index to the pool that many times
        for _ in 0..button_max {
            pool.push(i);
        }
    }
    pool
}

#[cfg(not(feature = "part2"))]
fn part1(machines: Vec<Machine>) {
    let mut sum: i64 = 0;

    for machine in &machines {
        let mut found = false;
        // Try all combinations of 1..=n buttons. Stop when the first successful combination is found.
        for k in 1..=machine.buttons.len() {
            let combinations = machine.buttons.iter().combinations(k);
            for combo in combinations {
                let result = combo.iter().fold(0, |state, &button| {
                    state ^ *button
                });
                if result == machine.final_state {
                    found = true;
                    break;
                }
            }
            if found {
                sum += k as i64;
                break;
            }
        }
        if !found {
            panic!("No combination of buttons found for machine: {:?}", machine);
        }
    }
    println!("Sum: {}", sum);
}

fn parse_machine(line: &str) -> Machine {
    let parts: Vec<&str> = line.split_whitespace().collect();

    #[cfg(not(feature = "part2"))]
    let final_state = parse_final_state(parts[0]);

    let mut buttons = Vec::new();
    #[cfg(feature = "part2")]
    let mut joltages = Vec::new();

    for part in &parts[1..] {
        if part.starts_with('(') && part.ends_with(')') {
            buttons.push(parse_button(part));
        } else {
            #[cfg(feature = "part2")]
            if part.starts_with('{') && part.ends_with('}') {
                joltages = parse_joltages(part);
            }
        }
    }
    Machine {
        #[cfg(not(feature = "part2"))]
        final_state,
        buttons,
        #[cfg(feature = "part2")]
        joltages,
    }
}

// Parse final state
#[cfg(not(feature = "part2"))]
fn parse_final_state(s: &str) -> u64 {
    let trimmed = s.trim_matches(&['[', ']'][..]);
    trimmed.chars().rev().fold(0, |state, c| {
        (state << 1) | if c == '#' {1} else {0}
    })
}

// Parse button
#[cfg(not(feature = "part2"))]
fn parse_button(s: &str) -> u64 {
    let trimmed = s.trim_matches(&['(', ')'][..]);
    trimmed.split(',').map(|num| num.parse::<u64>().unwrap()).fold(0, |state, num| {
        state | (1 << num)
    })
}

#[cfg(feature = "part2")]
fn parse_button(s: &str) -> Vec<usize> {
    let trimmed = s.trim_matches(&['(', ')'][..]);
    trimmed.split(',').map(|num| num.parse::<usize>().unwrap()).collect()
}

// Parse joltages
#[cfg(feature = "part2")]
fn parse_joltages(s: &str) -> Vec<i64> {
    let trimmed = s.trim_matches(&['{', '}'][..]);
    trimmed.split(',').map(|num| num.parse().unwrap()).collect()
}

// #[cfg(feature = "part2")]
// fn joltages_equal<T: PartialOrd>(a: &[T], b: &[T]) -> bool {
//     a.iter().zip(b.iter()).all(|(x, y)| x == y)
// }
// 
// #[cfg(feature = "part2")]
// fn joltages_less_than<T: PartialOrd>(a: &[T], b: &[T]) -> bool {
//     a.iter().zip(b.iter()).any(|(x, y)| y > x)
// }

// Increase elements of the joltages vector by one according to the button vector where each element is an index
#[cfg(feature = "part2")]
fn increase_joltages(joltages: &mut [i64], button: &[usize]) {
    for &index in button {
        joltages[index] += 1;
    }
}
