// Advent of Code 2025, Day 10, Part 2

use good_lp::{Expression, variables, coin_cbc, Solution, SolverModel};

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

        Machine {
            buttons,
            joltages,
        }
    }
}

fn parse_joltages(part: &str) -> Vec<i32> {
    part
        .trim_matches(&['{', '}'][..])
        .split(',')
        .map(|s| s.parse::<i32>().expect("Invalid joltage value"))
        .collect()
}

fn parse_buttons(parts: &[&str], len: usize) -> Vec<Vec<i32>> {
    parts.iter()
        .take_while(|p| p.starts_with('(') && p.ends_with(')'))
        .map(|p| parse_button(p, len))
        .collect()
}

fn parse_button(string: &&str, len: usize) -> Vec<i32> {
    let indexes = string.trim_matches(&['(', ')'][..])
    .split(',')
    .map(|s| s.parse::<i32>().expect("Invalid button index"))
    .collect::<Vec<i32>>();

    let mut button = vec![0; len];
    indexes.into_iter().for_each(|i| button[i as usize] = 1);

    button
}

pub fn part2(machines: Vec<Machine>) {
    let total = machines.iter().map(|machine| {
        let min_presses = solve_lp(&machine.buttons, &machine.joltages);
        min_presses.iter().sum::<i32>()
    }).sum::<i32>();
    println!("Total button presses: {}", total);
}


/// In short, find all solutions to the equation XB = J, where X is a vector of button presses, B is the button matrix, and J is
/// the joltage vector of size n, and return the minimum sum of elements in X.
/// - X is a row vector of size m.
/// - B is a matrix of size m x n with each row representing a button and each column representing whether or not the
///   button increments the corresponding joltage. Elements of B are either 0 or 1.
/// - J is a row vector of size n.
fn solve_lp(b_matrix: &[Vec<i32>], j_vector: &[i32]) -> Vec<i32>{
    let m = b_matrix.len();    // Number of rows
    let n = b_matrix[0].len(); // Number of columns

    assert!(j_vector.len() == n, "Joltage vector size must match number of columns in B");

    // Define the Variables
    let mut vars = variables!();
    // We create a vector of variables, each >= 0 and Integer
    let x: Vec<_> = (0..m)
        .map(|i| vars.add(good_lp::variable().min(0).integer().name(format!("x_{}", i))))
        .collect();

    // Define the Objective Function (Minimize the sum of X)
    let objective = x.iter().sum::<good_lp::Expression>();
    let mut problem = vars.minimise(objective).using(coin_cbc);
    problem.set_parameter("log", "0");

    // Add Constraints: XB = J
    // Each element J[k] is the sum of (X[i] * B[i][k]) for all i, B[i][k] is 0 or 1
    for k in 0..n {
        let col_expr: Expression = (0..m)
            .filter(|&i| b_matrix[i][k] != 0)
            .map(|i| x[i])
            .sum();
        problem.add_constraint(col_expr.eq(j_vector[k]));
    }

    // Solve the problem
    let solution = problem.solve().expect("Failed to solve LP problem");
    
    x.iter().map(|var| solution.value(*var) as i32).collect()
}

