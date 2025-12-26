// Advent of Code 2025, Day 6

use common::load;

#[cfg(feature = "instrumented")]
use instrumentation::Instrumentation;

fn main() {
    #[cfg(not(feature = "instrumented"))]
    println!("Day 6, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    let lines = load::lines();

    if cfg!(feature = "part2") {
        #[cfg(feature = "instrumented")]
        let mut inst = Instrumentation::new(&lines);

        // Find the length of the longest line to determine the number of columns.
        let number_of_columns = lines.iter().take(lines.len() - 1).map(|line| line.len()).max().expect("No lines found");

        // For all but the last line, each column of text contains a number, one digit per line from highest
        // significance to lowest. Blanks are ignored. A column of all spaces (with the value 0) separates each list of
        // numbers.
        let mut columns: Vec<Vec<i64>> = Vec::new();
        let mut list: Vec<i64> = Vec::new();
        for c in (0..number_of_columns).rev() {
            let mut value: i64 = 0;
            for line in lines.iter().take(lines.len() - 1) {
                let ch = line.chars().nth(c).unwrap_or(' ');
                if ch != ' ' {
                    value = value * 10 + ch.to_digit(10).expect("Invalid digit") as i64;
                }
            }
            if value == 0 {
                // A column of all spaces indicates the end of a list.
                #[cfg(feature = "instrumented")]
                inst.end_problem(&list);
                columns.push(list);
                list = Vec::new();
            } else {
                #[cfg(feature = "instrumented")]
                inst.record_column(c);
                list.push(value);
            }
        }
        if !list.is_empty() {
            #[cfg(feature = "instrumented")]
            inst.end_problem(&list);
            columns.push(list);
        }

        // The last line contains the operation to perform on each column.
        // Since the columns were built in reverse order, reverse the operations as well.
        let mut operations = parse_operations(&lines[lines.len() - 1]);
        operations.reverse();

        #[cfg(feature = "instrumented")]
        inst.emit_initial();

        let mut sum: i64 = 0;
        for (i, column) in columns.iter().enumerate() {
            assert!(!column.is_empty(), "Column {} is empty", i);
            let result = match operations[i] {
                '+' => column.iter().sum::<i64>(),
                '*' => column.iter().product::<i64>(),
                _ => panic!("Unknown operation"),
            };
            sum += result;
            #[cfg(feature = "instrumented")]
            inst.emit_problem(i, column, operations[i], result, sum);
        }

        #[cfg(feature = "instrumented")]
        inst.finalize(sum);

        #[cfg(not(feature = "instrumented"))]
        println!("Sum: {}", sum);
    } else {
        // Each line consists of a list of numbers separated by one or more spaces.
        // For all but the last line, process the input data. A vector of vectors of numbers is created such that each
        // vector contains the numbers in the corresponding column.
        let mut columns: Vec<Vec<i64>> = Vec::new();
        for line in lines.iter().take(lines.len() - 1) {
            let numbers: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Invalid number"))
                .collect();
            for (i, &num) in numbers.iter().enumerate() {
                if columns.len() <= i {
                    columns.push(Vec::new());
                }
                columns[i].push(num);
            }
        }

        // The last line contains the operation to perform on each column.
        let operations = parse_operations(&lines[lines.len() - 1]);


        let sum = columns.iter().enumerate().map(|(i, column)| {
            match operations[i] {
                '+' => column.iter().sum::<i64>(),
                '*' => column.iter().product::<i64>(),
                _ => panic!("Unknown operation"),
            }
        }).sum::<i64>();
        println!("Sum: {}", sum);
    }
}

fn parse_operations(line: &str) -> Vec<char> {
    line
        .split_whitespace()
        .map(|s| s.chars().next().expect("No operation found"))
        .collect()
}

#[cfg(feature = "instrumented")]
mod instrumentation {
    use serde::Serialize;

    #[derive(Serialize, Clone)]
    struct Frame {
        frame_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        problem_index: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        column_start: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        column_end: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        numbers: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        operator: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        result: Option<String>,
        running_total: String,
    }

    #[derive(Serialize)]
    struct Output {
        day: u32,
        part: u32,
        grid: Vec<String>,
        rows: usize,
        cols: usize,
        problem_count: usize,
        grand_total: String,
        frames: Vec<Frame>,
    }

    pub struct Instrumentation {
        grid: Vec<String>,
        rows: usize,
        cols: usize,
        frames: Vec<Frame>,
        problem_index_counter: usize,
        column_ranges: Vec<(usize, usize)>,
        col_start: usize,
        col_end: Option<usize>,
    }

    impl Instrumentation {
        pub fn new(lines: &[String]) -> Self {
            let grid: Vec<String> = lines.to_vec();
            let rows = grid.len();
            let cols = grid.iter().map(|l| l.len()).max().unwrap_or(0);
            Self {
                grid,
                rows,
                cols,
                frames: Vec::new(),
                problem_index_counter: 0,
                column_ranges: Vec::new(),
                col_start: 0,
                col_end: None,
            }
        }

        pub fn record_column(&mut self, c: usize) {
            if self.col_end.is_none() {
                self.col_end = Some(c);
            }
            self.col_start = c;
        }

        pub fn end_problem(&mut self, list: &[i64]) {
            if !list.is_empty() {
                self.column_ranges.push((self.col_start, self.col_end.unwrap()));
                self.col_end = None;
            }
        }

        pub fn emit_initial(&mut self) {
            self.frames.push(Frame {
                frame_type: "initial".to_string(),
                problem_index: None,
                column_start: None,
                column_end: None,
                numbers: None,
                operator: None,
                result: None,
                running_total: "0".to_string(),
            });
        }

        pub fn emit_problem(
            &mut self,
            index: usize,
            numbers: &[i64],
            operator: char,
            result: i64,
            running_total: i64,
        ) {
            let (col_start, col_end) = self.column_ranges[index];
            self.frames.push(Frame {
                frame_type: "problem".to_string(),
                problem_index: Some(self.problem_index_counter),
                column_start: Some(col_start),
                column_end: Some(col_end),
                numbers: Some(numbers.iter().map(|n| n.to_string()).collect()),
                operator: Some(operator.to_string()),
                result: Some(result.to_string()),
                running_total: running_total.to_string(),
            });
            self.problem_index_counter += 1;
        }

        pub fn finalize(&self, grand_total: i64) {
            let mut frames = self.frames.clone();
            frames.push(Frame {
                frame_type: "final".to_string(),
                problem_index: None,
                column_start: None,
                column_end: None,
                numbers: None,
                operator: None,
                result: None,
                running_total: grand_total.to_string(),
            });

            let output = Output {
                day: 6,
                part: 2,
                grid: self.grid.clone(),
                rows: self.rows,
                cols: self.cols,
                problem_count: self.problem_index_counter,
                grand_total: grand_total.to_string(),
                frames,
            };

            println!("{}", serde_json::to_string(&output).unwrap());
        }
    }
}
