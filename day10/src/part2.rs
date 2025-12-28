// Advent of Code 2025, Day 10, Part 2

use good_lp::{Expression, Solution, SolverModel, coin_cbc, variables};

#[derive(Debug)]
pub struct Machine {
    pub buttons: Vec<Vec<i32>>, // Indexes of joltage values
    pub joltages: Vec<i32>,
}

impl Machine {
    pub fn new(line: &str) -> Machine {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let joltages = parse_joltages(parts.last().expect("No joltages provided"));
        let buttons = parse_buttons(&parts[1..], joltages.len());

        Machine { buttons, joltages }
    }
}

fn parse_joltages(part: &str) -> Vec<i32> {
    part.trim_matches(&['{', '}'][..])
        .split(',')
        .map(|s| s.parse::<i32>().expect("Invalid joltage value"))
        .collect()
}

fn parse_buttons(parts: &[&str], len: usize) -> Vec<Vec<i32>> {
    parts
        .iter()
        .take_while(|p| p.starts_with('(') && p.ends_with(')'))
        .map(|p| parse_button(p, len))
        .collect()
}

fn parse_button(string: &&str, len: usize) -> Vec<i32> {
    let indexes = string
        .trim_matches(&['(', ')'][..])
        .split(',')
        .map(|s| s.parse::<i32>().expect("Invalid button index"))
        .collect::<Vec<i32>>();

    let mut button = vec![0; len];
    indexes.into_iter().for_each(|i| button[i as usize] = 1);

    button
}

pub fn part2(machines: Vec<Machine>) {
    #[cfg(feature = "instrumented")]
    let mut inst = instrumentation::Instrumentation::new(machines.len());

    #[cfg(feature = "instrumented")]
    inst.emit_intro();

    let _total = machines
        .iter()
        .enumerate()
        .map(|(_idx, machine)| {
            #[cfg(feature = "instrumented")]
            inst.begin_machine(_idx, &machine.buttons, &machine.joltages);

            let min_presses = solve_lp(&machine.buttons, &machine.joltages);
            let sum = min_presses.iter().sum::<i32>();

            #[cfg(feature = "instrumented")]
            inst.solution_found(&min_presses, &machine.buttons);

            #[cfg(feature = "instrumented")]
            inst.machine_complete(sum);

            sum
        })
        .sum::<i32>();

    #[cfg(feature = "instrumented")]
    inst.finalize_and_print();

    #[cfg(not(feature = "instrumented"))]
    println!("Total button presses: {}", _total);
}

/// In short, find all solutions to the equation XB = J, where X is a vector of button presses, B is the button matrix, and J is
/// the joltage vector of size n, and return the minimum sum of elements in X.
/// - X is a row vector of size m.
/// - B is a matrix of size m x n with each row representing a button and each column representing whether or not the
///   button increments the corresponding joltage. Elements of B are either 0 or 1.
/// - J is a row vector of size n.
fn solve_lp(b_matrix: &[Vec<i32>], j_vector: &[i32]) -> Vec<i32> {
    let m = b_matrix.len(); // Number of rows
    let n = b_matrix[0].len(); // Number of columns

    assert!(
        j_vector.len() == n,
        "Joltage vector size must match number of columns in B"
    );

    // Define the Variables
    let mut vars = variables!();
    // We create a vector of variables, each >= 0 and Integer
    let x: Vec<_> = (0..m)
        .map(|i| {
            vars.add(
                good_lp::variable()
                    .min(0)
                    .integer()
                    .name(format!("x_{}", i)),
            )
        })
        .collect();

    // Define the Objective Function (Minimize the sum of X)
    let objective = x.iter().sum::<good_lp::Expression>();
    let mut problem = vars.minimise(objective).using(coin_cbc);
    problem.set_parameter("log", "0");

    // Add Constraints: XB = J
    // Each element J[k] is the sum of (X[i] * B[i][k]) for all i, B[i][k] is 0 or 1
    for k in 0..n {
        let col_expr: Expression = (0..m).filter(|&i| b_matrix[i][k] != 0).map(|i| x[i]).sum();
        problem.add_constraint(col_expr.eq(j_vector[k]));
    }

    // Solve the problem
    let solution = problem.solve().expect("Failed to solve LP problem");

    x.iter().map(|var| solution.value(*var) as i32).collect()
}

#[cfg(feature = "instrumented")]
mod instrumentation {
    use serde::Serialize;

    #[derive(Serialize, Clone)]
    pub struct Button {
        pub index: usize,
        #[serde(rename = "affectedCounters")]
        pub affected_counters: Vec<usize>,
        #[serde(rename = "pressCount")]
        pub press_count: i32,
    }

    #[derive(Serialize, Clone)]
    pub struct MachineData {
        pub id: usize,
        pub buttons: Vec<Button>,
        pub joltages: Vec<i32>,
        #[serde(rename = "currentValues")]
        pub current_values: Vec<i32>,
        #[serde(rename = "minPresses")]
        pub min_presses: i32,
        #[serde(rename = "isComplete")]
        pub is_complete: bool,
    }

    #[derive(Serialize, Clone)]
    pub struct Frame {
        #[serde(rename = "frameType")]
        pub frame_type: String,
        #[serde(rename = "machineIndex")]
        pub machine_index: usize,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub machine: Option<MachineData>,
        #[serde(rename = "activeButton", skip_serializing_if = "Option::is_none")]
        pub active_button: Option<usize>,
        #[serde(rename = "pressedButtons", skip_serializing_if = "Option::is_none")]
        pub pressed_buttons: Option<Vec<usize>>,
        #[serde(rename = "currentValues", skip_serializing_if = "Option::is_none")]
        pub current_values: Option<Vec<i32>>,
        #[serde(rename = "machinesSolved")]
        pub machines_solved: usize,
        #[serde(rename = "totalMachines")]
        pub total_machines: usize,
        #[serde(rename = "runningTotal")]
        pub running_total: i32,
        #[serde(rename = "finalAnswer", skip_serializing_if = "Option::is_none")]
        pub final_answer: Option<i32>,
    }

    #[derive(Serialize)]
    pub struct LogData {
        #[serde(rename = "puzzleDay")]
        pub puzzle_day: u32,
        #[serde(rename = "puzzleName")]
        pub puzzle_name: String,
        pub part: u32,
        pub frames: Vec<Frame>,
        #[serde(rename = "finalAnswer")]
        pub final_answer: i32,
        #[serde(rename = "totalMachines")]
        pub total_machines: usize,
    }

    pub struct Instrumentation {
        total_machines: usize,
        machines_solved: usize,
        running_total: i32,
        current_machine_index: usize,
        current_machine: Option<MachineData>,
        frames: Vec<Frame>,
    }

    impl Instrumentation {
        pub fn new(total_machines: usize) -> Self {
            Self {
                total_machines,
                machines_solved: 0,
                running_total: 0,
                current_machine_index: 0,
                current_machine: None,
                frames: Vec::new(),
            }
        }

        pub fn emit_intro(&mut self) {
            self.frames.push(Frame {
                frame_type: "intro".to_string(),
                machine_index: 0,
                machine: None,
                active_button: None,
                pressed_buttons: None,
                current_values: None,
                machines_solved: 0,
                total_machines: self.total_machines,
                running_total: 0,
                final_answer: None,
            });
        }

        pub fn begin_machine(
            &mut self,
            machine_index: usize,
            buttons_raw: &[Vec<i32>],
            joltages: &[i32],
        ) {
            self.current_machine_index = machine_index;

            let buttons: Vec<Button> = buttons_raw
                .iter()
                .enumerate()
                .map(|(i, b)| Button {
                    index: i,
                    affected_counters: b
                        .iter()
                        .enumerate()
                        .filter_map(|(j, &v)| if v != 0 { Some(j) } else { None })
                        .collect(),
                    press_count: 0,
                })
                .collect();

            self.current_machine = Some(MachineData {
                id: machine_index,
                buttons,
                joltages: joltages.to_vec(),
                current_values: vec![0; joltages.len()],
                min_presses: 0,
                is_complete: false,
            });

            // Emit machineStart frames
            for _ in 0..5 {
                self.frames.push(Frame {
                    frame_type: "machineStart".to_string(),
                    machine_index,
                    machine: self.current_machine.clone(),
                    active_button: None,
                    pressed_buttons: None,
                    current_values: None,
                    machines_solved: self.machines_solved,
                    total_machines: self.total_machines,
                    running_total: self.running_total,
                    final_answer: None,
                });
            }

            // Emit solving frames
            for _ in 0..3 {
                self.frames.push(Frame {
                    frame_type: "solving".to_string(),
                    machine_index,
                    machine: self.current_machine.clone(),
                    active_button: None,
                    pressed_buttons: None,
                    current_values: None,
                    machines_solved: self.machines_solved,
                    total_machines: self.total_machines,
                    running_total: self.running_total,
                    final_answer: None,
                });
            }
        }

        pub fn solution_found(&mut self, press_counts: &[i32], buttons_raw: &[Vec<i32>]) {
            let min_presses: i32 = press_counts.iter().sum();

            if let Some(ref mut machine) = self.current_machine {
                for (i, &count) in press_counts.iter().enumerate() {
                    machine.buttons[i].press_count = count;
                }
                machine.min_presses = min_presses;
            }

            // Emit solutionFound frames
            for _ in 0..5 {
                self.frames.push(Frame {
                    frame_type: "solutionFound".to_string(),
                    machine_index: self.current_machine_index,
                    machine: self.current_machine.clone(),
                    active_button: None,
                    pressed_buttons: None,
                    current_values: None,
                    machines_solved: self.machines_solved,
                    total_machines: self.total_machines,
                    running_total: self.running_total,
                    final_answer: None,
                });
            }

            // Emit buttonPress frames for each button with pressCount > 0
            let mut current_values =
                vec![0i32; self.current_machine.as_ref().unwrap().joltages.len()];
            let mut pressed_buttons: Vec<usize> = Vec::new();

            for (button_idx, &count) in press_counts.iter().enumerate() {
                if count > 0 {
                    // Update current values based on button effect
                    for (j, &effect) in buttons_raw[button_idx].iter().enumerate() {
                        current_values[j] += effect * count;
                    }
                    pressed_buttons.push(button_idx);

                    // Update machine state
                    if let Some(ref mut machine) = self.current_machine {
                        machine.current_values = current_values.clone();
                    }

                    // Emit 2 frames per button
                    for _ in 0..2 {
                        self.frames.push(Frame {
                            frame_type: "buttonPress".to_string(),
                            machine_index: self.current_machine_index,
                            machine: self.current_machine.clone(),
                            active_button: Some(button_idx),
                            pressed_buttons: Some(pressed_buttons.clone()),
                            current_values: Some(current_values.clone()),
                            machines_solved: self.machines_solved,
                            total_machines: self.total_machines,
                            running_total: self.running_total,
                            final_answer: None,
                        });
                    }
                }
            }
        }

        pub fn machine_complete(&mut self, min_presses: i32) {
            self.machines_solved += 1;
            self.running_total += min_presses;

            if let Some(ref mut machine) = self.current_machine {
                machine.current_values = machine.joltages.clone();
                machine.is_complete = true;
            }

            // Emit complete frames
            for _ in 0..5 {
                self.frames.push(Frame {
                    frame_type: "complete".to_string(),
                    machine_index: self.current_machine_index,
                    machine: self.current_machine.clone(),
                    active_button: None,
                    pressed_buttons: None,
                    current_values: None,
                    machines_solved: self.machines_solved,
                    total_machines: self.total_machines,
                    running_total: self.running_total,
                    final_answer: None,
                });
            }
        }

        pub fn finalize_and_print(mut self) {
            let final_answer = self.running_total;

            // Emit final frames
            for _ in 0..30 {
                self.frames.push(Frame {
                    frame_type: "final".to_string(),
                    machine_index: self.total_machines - 1,
                    machine: None,
                    active_button: None,
                    pressed_buttons: None,
                    current_values: None,
                    machines_solved: self.machines_solved,
                    total_machines: self.total_machines,
                    running_total: self.running_total,
                    final_answer: Some(final_answer),
                });
            }

            let log_data = LogData {
                puzzle_day: 10,
                puzzle_name: "Factory".to_string(),
                part: 2,
                frames: self.frames,
                final_answer,
                total_machines: self.total_machines,
            };

            println!("{}", serde_json::to_string(&log_data).unwrap());
        }
    }
}
