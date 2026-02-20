use itertools::Itertools;

#[derive(Debug)]
pub struct Machine {
    pub final_state: u64,  // binary representation of final state
    pub buttons: Vec<u64>, // binary representation of button effects
}

pub fn part1(machines: Vec<Machine>) {
    let mut sum: i64 = 0;

    for machine in &machines {
        let mut found = false;
        // Try all combinations of 1..=n buttons. Stop when the first successful combination is found.
        for k in 1..=machine.buttons.len() {
            let combinations = machine.buttons.iter().combinations(k);
            for combo in combinations {
                let result = combo.iter().fold(0, |state, &button| state ^ *button);
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

pub fn parse_machine(line: &str) -> Machine {
    let parts: Vec<&str> = line.split_whitespace().collect();

    let final_state = parse_final_state(parts[0]);

    let mut buttons = Vec::new();

    for part in &parts[1..] {
        if part.starts_with('(') && part.ends_with(')') {
            buttons.push(parse_button(part));
        }
    }
    Machine { final_state, buttons }
}

// Parse final state
fn parse_final_state(s: &str) -> u64 {
    let trimmed = s.trim_matches(&['[', ']'][..]);
    trimmed
        .chars()
        .rev()
        .fold(0, |state, c| (state << 1) | if c == '#' { 1 } else { 0 })
}

// Parse button
fn parse_button(s: &str) -> u64 {
    let trimmed = s.trim_matches(&['(', ')'][..]);
    trimmed
        .split(',')
        .map(|num| num.parse::<u64>().expect("Failed to parse button index as u64"))
        .fold(0, |state, num| state | (1 << num))
}
