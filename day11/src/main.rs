// Advent of Code 2025, Day 11

use std::collections::HashMap;
use common::load;

fn main() {
    println!("Day 11, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    let lines = load::lines();

    // The input describes a DAG. Each node is described by a line in the input as follows: <node>":" <output> <output> ...
    // The root node is named "you". Nodes named "out" are terminal nodes.
    let dag = lines.iter().map(|line| {
        let (node_str, outputs_str) = line.split_once(':').unwrap();
        let node = node_str.trim().to_string();
        let outputs: Vec<String> = outputs_str.trim().split_whitespace().map(|s| s.to_string()).collect();
        (node, outputs)
    }).collect::<HashMap<String, Vec<String>>>();

    part1(&dag);
}

fn part1(dag: &HashMap<String, Vec<String>>) {
    // Recursively traverse the DAG from "you" to count all unique paths to "out" nodes.
    let result = count_paths(dag, "you");
    println!("Result: {}", result);
}

fn count_paths( dag: &HashMap<String, Vec<String>>, node: &str) -> usize {
    let mut total_paths = 0;
    if let Some(outputs) = dag.get(node) {
        for output in outputs {
            if output == "out" {
                total_paths += 1;
            } else {
                total_paths += count_paths(dag, output);
            }
        }
    }
    total_paths
}