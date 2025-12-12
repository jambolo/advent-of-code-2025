// Advent of Code 2025, Day 11

use std::collections::HashMap;
use common::load;

fn main() {
    println!("Day 11, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    let lines = load::lines();

    // I am assuming the following:
    //   1. The input describes a DAG (no cycles).
    //   2. Each node is unique.
    //   3. The node named "out" is terminal node and it only exists as an output of other nodes.

    let mut dag: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let (node_str, outputs_str) = line.split_once(':').unwrap();
        let node = node_str.trim().to_string();
        let outputs: Vec<String> = outputs_str.trim().split_whitespace().map(|s| s.to_string()).collect();
        dag.insert(node, outputs);
    }

    if cfg!(feature = "part2") {
        part2(&dag);
    } else {
        part1(&dag);
    }
}

fn reduce(dag: &mut HashMap<String, Vec<String>>, excluded: &[&str]) {
    loop {
        let out = "out";
        let mut to_remove = Vec::new();
        for (node, outputs) in dag.iter() {
            if !outputs.is_empty()
                && !excluded.contains(&node.as_str())
                && outputs.iter().all(|o| o == out)
            {
                to_remove.push(node.clone());
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for node in &to_remove {
            dag.get_mut(node).unwrap().clear();
        }

        for outputs in dag.values_mut() {
            for output in outputs.iter_mut() {
                if to_remove.contains(output) {
                    *output = out.to_string();
                }
            }
        }
    }
}

fn part2(dag: &HashMap<String, Vec<String>>) {
    // Here is the plan.
    // Count the number of paths from "fft" to "dac".
    // If it is not zero, then
    //     Count the number of paths from "dac" to "out".
    //     Count the number of paths from "svr" to "fft".
    //     Return the product of the three counts.
    // Otherwise,
    //     Count the number of paths from "svr" to "dac".
    //     Count the number of paths from "dac" to "fft".
    //     Count the number of paths from "fft" to "out". Return the product.


    // Count the number of paths from "fft" to "dac".
    let fft_to_dac = {
        let mut dag = dag.clone();
        let excluded = vec!["fft", "dac"];
        reduce(&mut dag, &excluded);
        count_paths(&dag, "fft", "dac", "out")
    };

    if fft_to_dac > 0 {
        // Count the number of paths from "dac" to "out".
        let dac_to_out = count_paths(&dag, "dac", "out", "out");

        // Count the number of paths from "svr" to "fft".
        let svr_to_fft = {
            let mut dag = dag.clone();
            let excluded = vec!["svr", "fft"];
            reduce(&mut dag, &excluded);
            count_paths(&dag, "svr", "fft", "out")
        };
        println!("Result: {}", svr_to_fft * fft_to_dac * dac_to_out);
    } else {
        // Count the number of paths from "svr" to "dac".
        let svr_to_dac = {
            let mut dag = dag.clone();
            let excluded = vec!["svr", "dac"];
            reduce(&mut dag, &excluded);
            count_paths(&dag, "svr", "dac", "out")
        };

        // Count the number of paths from "dac" to "fft".
        let dac_to_fft = {
            let mut dag = dag.clone();
            let excluded = vec!["dac", "fft"];
            reduce(&mut dag, &excluded);
            count_paths(&dag, "dac", "fft", "out")
        };

        // Count the number of paths from "fft" to "out".
        let fft_to_out = count_paths(&dag, "fft", "out", "out");

        println!("Result: {}", svr_to_dac * dac_to_fft * fft_to_out);
    }
}

fn part1(dag: &HashMap<String, Vec<String>>) {
    // Recursively traverse the DAG from "you" to count all unique paths to "out" nodes.
    let result = count_paths(dag, "you", "out", "out");
    println!("Result: {}", result);
}

/// Count all unique paths from one node to another in a DAG.
/// Recursively traverses the DAG starting from `from` and counts all distinct paths that lead to `to`.
fn count_paths(dag: &HashMap<String, Vec<String>>, from: &str, to: &str, terminal: &str) -> usize {
    dag.get(from).map_or(0, |outputs| {
        outputs.iter().fold(0, |acc, output| {
            let rest = if output == to {
                1
            } else if output != terminal {
                count_paths(dag, output, to, terminal)
            } else {
                0
            };
            acc + rest
        })
    })
}
