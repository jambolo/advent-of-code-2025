// Advent of Code 2025, Day 11

use common::load;
use std::collections::HashMap;

#[cfg(feature = "instrumented")]
use instrumentation::Instrumentation;

fn main() {
    #[cfg(not(feature = "instrumented"))]
    println!("Day 11, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    let lines = load::lines();

    // I am assuming the following:
    //   1. The input describes a DAG (no cycles).
    //   2. Each node is unique.
    //   3. The node named "out" is terminal node and it only exists as an output of other nodes.

    let mut dag: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let (node_str, outputs_str) = line
            .split_once(':')
            .expect("Failed to split line on ':' for node and outputs");
        let node = node_str.trim().to_string();
        let outputs: Vec<String> = outputs_str.split_whitespace().map(|s| s.to_string()).collect();
        dag.insert(node, outputs);
    }

    if cfg!(feature = "part2") {
        part2(&dag);
    } else {
        part1(&dag);
    }
}

fn reduce(dag: &mut HashMap<String, Vec<String>>, excluded: &[&str], #[cfg(feature = "instrumented")] inst: &mut Instrumentation) {
    loop {
        let out = "out";
        let mut to_remove = Vec::new();
        for (node, outputs) in dag.iter() {
            if !outputs.is_empty() && !excluded.contains(&node.as_str()) && outputs.iter().all(|o| o == out) {
                to_remove.push(node.clone());
            }
        }

        if to_remove.is_empty() {
            break;
        }

        #[cfg(feature = "instrumented")]
        inst.record_reduction_batch(&to_remove, dag);

        for node in &to_remove {
            dag.get_mut(node).expect("Node to remove not found in dag").clear();
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
    #[cfg(feature = "instrumented")]
    let mut inst = Instrumentation::new(&dag);
    #[cfg(feature = "instrumented")]
    inst.emit_intro_frames();
    #[cfg(feature = "instrumented")]
    inst.emit_graph_display_frames();

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

        #[cfg(feature = "instrumented")]
        inst.begin_segment(1, "fft", "dac");

        reduce(
            &mut dag,
            &excluded,
            #[cfg(feature = "instrumented")]
            &mut inst,
        );
        count_paths(
            &dag,
            "fft",
            "dac",
            "out",
            #[cfg(feature = "instrumented")]
            &mut inst,
        )
    };

    if fft_to_dac > 0 {
        #[cfg(feature = "instrumented")]
        inst.complete_segment(1, fft_to_dac);

        #[cfg(feature = "instrumented")]
        inst.begin_segment(2, "dac", "out");

        // Count the number of paths from "dac" to "out".
        let dac_to_out = count_paths(
            dag,
            "dac",
            "out",
            "out",
            #[cfg(feature = "instrumented")]
            &mut inst,
        );

        #[cfg(feature = "instrumented")]
        inst.complete_segment(2, dac_to_out);

        // Count the number of paths from "svr" to "fft".
        let svr_to_fft = {
            let mut dag = dag.clone();
            let excluded = vec!["svr", "fft"];

            #[cfg(feature = "instrumented")]
            inst.begin_segment(0, "svr", "fft");

            reduce(
                &mut dag,
                &excluded,
                #[cfg(feature = "instrumented")]
                &mut inst,
            );
            count_paths(
                &dag,
                "svr",
                "fft",
                "out",
                #[cfg(feature = "instrumented")]
                &mut inst,
            )
        };

        #[cfg(feature = "instrumented")]
        inst.complete_segment(0, svr_to_fft);

        let result = svr_to_fft * fft_to_dac * dac_to_out;

        #[cfg(not(feature = "instrumented"))]
        println!("Result: {}", result);

        #[cfg(feature = "instrumented")]
        inst.set_final_answer(result);
    } else {
        // Count the number of paths from "svr" to "dac".
        let svr_to_dac = {
            let mut dag = dag.clone();
            let excluded = vec!["svr", "dac"];

            #[cfg(feature = "instrumented")]
            inst.begin_segment(0, "svr", "dac");

            reduce(
                &mut dag,
                &excluded,
                #[cfg(feature = "instrumented")]
                &mut inst,
            );
            count_paths(
                &dag,
                "svr",
                "dac",
                "out",
                #[cfg(feature = "instrumented")]
                &mut inst,
            )
        };

        #[cfg(feature = "instrumented")]
        inst.complete_segment(0, svr_to_dac);

        // Count the number of paths from "dac" to "fft".
        let dac_to_fft = {
            let mut dag = dag.clone();
            let excluded = vec!["dac", "fft"];

            #[cfg(feature = "instrumented")]
            inst.begin_segment(1, "dac", "fft");

            reduce(
                &mut dag,
                &excluded,
                #[cfg(feature = "instrumented")]
                &mut inst,
            );
            count_paths(
                &dag,
                "dac",
                "fft",
                "out",
                #[cfg(feature = "instrumented")]
                &mut inst,
            )
        };
        #[cfg(feature = "instrumented")]
        inst.complete_segment(1, dac_to_fft);

        #[cfg(feature = "instrumented")]
        inst.begin_segment(2, "fft", "out");

        // Count the number of paths from "fft" to "out".
        let fft_to_out = count_paths(
            dag,
            "fft",
            "out",
            "out",
            #[cfg(feature = "instrumented")]
            &mut inst,
        );

        #[cfg(feature = "instrumented")]
        inst.complete_segment(2, fft_to_out);

        let result = svr_to_dac * dac_to_fft * fft_to_out;

        #[cfg(not(feature = "instrumented"))]
        println!("Result: {}", result);

        #[cfg(feature = "instrumented")]
        inst.set_final_answer(result);
    }

    #[cfg(feature = "instrumented")]
    inst.emit_multiply_frames();
    #[cfg(feature = "instrumented")]
    inst.emit_final_frames();
    #[cfg(feature = "instrumented")]
    inst.finalize_and_print();
}

fn part1(dag: &HashMap<String, Vec<String>>) {
    // Recursively traverse the DAG from "you" to count all unique paths to "out" nodes.
    let result = count_paths(
        dag,
        "you",
        "out",
        "out",
        #[cfg(feature = "instrumented")]
        &mut Instrumentation::new(dag),
    );
    println!("Result: {}", result);
}

/// Count all unique paths from one node to another in a DAG.
/// Recursively traverses the DAG starting from `from` and counts all distinct paths that lead to `to`.
fn count_paths(
    dag: &HashMap<String, Vec<String>>,
    from: &str,
    to: &str,
    terminal: &str,
    #[cfg(feature = "instrumented")] inst: &mut Instrumentation,
) -> usize {
    dag.get(from).map_or(0, |outputs| {
        outputs.iter().fold(0, |acc, output| {
            #[cfg(feature = "instrumented")]
            inst.record_edge_enter(from, output);

            let rest = if output == to {
                1
            } else if output != terminal {
                count_paths(
                    dag,
                    output,
                    to,
                    terminal,
                    #[cfg(feature = "instrumented")]
                    inst,
                )
            } else {
                0
            };

            #[cfg(feature = "instrumented")]
            inst.record_edge_exit();

            acc + rest
        })
    })
}

#[cfg(feature = "instrumented")]
mod instrumentation {
    use serde::Serialize;
    use std::collections::HashMap;

    #[derive(Serialize, Clone)]
    pub struct Edge {
        from: String,
        to: String,
    }

    #[derive(Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Frame {
        frame_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        active_nodes: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        active_edges: Option<Vec<(String, String)>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pruned_nodes: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pruned_edges: Option<Vec<(String, String)>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        current_node: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        active_segment: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        segment_counts: Option<[usize; 3]>,
        #[serde(skip_serializing_if = "Option::is_none")]
        segment_complete: Option<[bool; 3]>,
        #[serde(skip_serializing_if = "Option::is_none")]
        segment_labels: Option<[String; 3]>,
        #[serde(skip_serializing_if = "Option::is_none")]
        message: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        final_answer: Option<usize>,
    }

    #[derive(Serialize, Clone)]
    pub struct Graph {
        nodes: Vec<String>,
        edges: Vec<Edge>,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Output {
        puzzle_day: u32,
        puzzle_name: String,
        part: u32,
        graph: Graph,
        frames: Vec<Frame>,
        final_answer: usize,
    }

    pub struct Instrumentation {
        output: Output,
        all_edges: Vec<(String, String)>,
        pruned_nodes: Vec<String>,
        pruned_edges: Vec<(String, String)>,
        active_segment_label: Option<String>,
        edge_visit_counter: usize,
        traversal_stack: Vec<(String, String)>,
        segment_start_node: Option<String>,
        segment_labels: [String; 3],
        segment_counts: [usize; 3],
        segment_complete: [bool; 3],
    }

    impl Instrumentation {
        pub fn new(dag: &HashMap<String, Vec<String>>) -> Self {
            let mut all_nodes: Vec<String> = dag.keys().cloned().collect();
            if !all_nodes.contains(&"out".to_string()) {
                all_nodes.push("out".to_string());
            }
            all_nodes.sort();

            let mut all_edges = Vec::new();
            for (from, outputs) in dag {
                for to in outputs {
                    all_edges.push((from.clone(), to.clone()));
                }
            }
            all_edges.sort();

            let graph = Graph {
                nodes: all_nodes,
                edges: all_edges
                    .iter()
                    .map(|(f, t)| Edge {
                        from: f.clone(),
                        to: t.clone(),
                    })
                    .collect(),
            };

            Self {
                output: Output {
                    puzzle_day: 11,
                    puzzle_name: "Reactor".to_string(),
                    part: 2,
                    graph,
                    frames: Vec::new(),
                    final_answer: 0,
                },
                all_edges,
                pruned_nodes: Vec::new(),
                pruned_edges: Vec::new(),
                active_segment_label: None,
                edge_visit_counter: 0,
                traversal_stack: Vec::new(),
                segment_start_node: None,
                segment_labels: [String::new(), String::new(), String::new()],
                segment_counts: [0, 0, 0],
                segment_complete: [false, false, false],
            }
        }

        pub fn emit_intro_frames(&mut self) {
            let frame = Frame {
                frame_type: "intro".to_string(),
                active_nodes: None,
                active_edges: None,
                pruned_nodes: None,
                pruned_edges: None,
                current_node: None,
                active_segment: None,
                segment_counts: Some(self.segment_counts),
                segment_complete: Some(self.segment_complete),
                segment_labels: Some(self.segment_labels.clone()),
                message: None,
                final_answer: None,
            };
            self.output.frames.push(frame);
        }

        pub fn emit_graph_display_frames(&mut self) {
            let frame = Frame {
                frame_type: "graphDisplay".to_string(),
                active_nodes: None,
                active_edges: None,
                pruned_nodes: None,
                pruned_edges: None,
                current_node: None,
                active_segment: None,
                segment_counts: None,
                segment_complete: None,
                segment_labels: None,
                message: None,
                final_answer: None,
            };
            self.output.frames.push(frame);
        }

        pub fn begin_segment(&mut self, segment_idx: usize, from: &str, to: &str) {
            let label = format!("{}→{}", from, to);
            self.active_segment_label = Some(label.clone());
            self.segment_labels[segment_idx] = label;
            self.pruned_nodes.clear();
            self.pruned_edges.clear();
            self.edge_visit_counter = 0;
            self.traversal_stack.clear();
            self.segment_start_node = Some(from.to_string());

            let frame = Frame {
                frame_type: "segmentStart".to_string(),
                active_nodes: None,
                active_edges: None,
                pruned_nodes: None,
                pruned_edges: None,
                current_node: None,
                active_segment: self.active_segment_label.clone(),
                segment_counts: None,
                segment_complete: None,
                segment_labels: Some(self.segment_labels.clone()),
                message: Some(format!("Starting segment: {} → {}", from, to)),
                final_answer: None,
            };
            self.output.frames.push(frame);
        }

        pub fn record_reduction_batch(&mut self, pruned: &[String], _dag: &HashMap<String, Vec<String>>) {
            let mut new_pruned_edges = Vec::new();
            for node in pruned {
                if !self.pruned_nodes.contains(node) {
                    self.pruned_nodes.push(node.clone());
                    for (from, to) in &self.all_edges {
                        if (from == node || to == node) && !self.pruned_edges.contains(&(from.clone(), to.clone())) {
                            self.pruned_edges.push((from.clone(), to.clone()));
                            new_pruned_edges.push((from.clone(), to.clone()));
                        }
                    }
                }
            }

            let frame = Frame {
                frame_type: "reduction".to_string(),
                active_nodes: None,
                active_edges: None,
                pruned_nodes: Some(self.pruned_nodes.clone()),
                pruned_edges: Some(self.pruned_edges.clone()),
                current_node: None,
                active_segment: self.active_segment_label.clone(),
                segment_counts: None,
                segment_complete: None,
                segment_labels: None,
                message: Some(format!("Pruning {} nodes", pruned.len())),
                final_answer: None,
            };
            self.output.frames.push(frame);
        }

        pub fn record_edge_enter(&mut self, from: &str, to: &str) {
            self.traversal_stack.push((from.to_string(), to.to_string()));
            self.edge_visit_counter += 1;

            if self.edge_visit_counter % 10000 == 0 {
                // Collect all nodes in the current traversal path
                let mut active_nodes: Vec<String> = Vec::new();
                if let Some(ref start) = self.segment_start_node {
                    active_nodes.push(start.clone());
                }
                for (_, node_to) in &self.traversal_stack {
                    if !active_nodes.contains(node_to) {
                        active_nodes.push(node_to.clone());
                    }
                }

                // Use all edges in the traversal stack as active edges
                let active_edges: Vec<(String, String)> = self.traversal_stack.clone();

                // Current node is the last node we're visiting (the 'to' of the last edge)
                let current_node = to.to_string();

                let frame = Frame {
                    frame_type: "counting".to_string(),
                    active_nodes: if active_nodes.is_empty() { None } else { Some(active_nodes) },
                    active_edges: if active_edges.is_empty() { None } else { Some(active_edges) },
                    pruned_nodes: None,
                    pruned_edges: None,
                    current_node: Some(current_node),
                    active_segment: self.active_segment_label.clone(),
                    segment_counts: Some(self.segment_counts),
                    segment_complete: None,
                    segment_labels: None,
                    message: None,
                    final_answer: None,
                };
                self.output.frames.push(frame);
            }
        }

        pub fn record_edge_exit(&mut self) {
            self.traversal_stack.pop();
        }

        pub fn complete_segment(&mut self, segment_idx: usize, count: usize) {
            self.segment_counts[segment_idx] = count;
            self.segment_complete[segment_idx] = true;

            let frame = Frame {
                frame_type: "segmentComplete".to_string(),
                active_nodes: None,
                active_edges: None,
                pruned_nodes: None,
                pruned_edges: None,
                current_node: None,
                active_segment: None,
                segment_counts: Some(self.segment_counts),
                segment_complete: Some(self.segment_complete),
                segment_labels: None,
                message: None,
                final_answer: None,
            };
            self.output.frames.push(frame);

            self.active_segment_label = None;
        }

        pub fn set_final_answer(&mut self, answer: usize) {
            self.output.final_answer = answer;
        }

        pub fn emit_multiply_frames(&mut self) {
            let frame = Frame {
                frame_type: "multiply".to_string(),
                active_nodes: None,
                active_edges: None,
                pruned_nodes: None,
                pruned_edges: None,
                current_node: None,
                active_segment: None,
                segment_counts: Some(self.segment_counts),
                segment_complete: None,
                segment_labels: None,
                message: None,
                final_answer: Some(self.output.final_answer),
            };
            self.output.frames.push(frame);
        }

        pub fn emit_final_frames(&mut self) {
            let frame = Frame {
                frame_type: "final".to_string(),
                active_nodes: None,
                active_edges: None,
                pruned_nodes: None,
                pruned_edges: None,
                current_node: None,
                active_segment: None,
                segment_counts: Some(self.segment_counts),
                segment_complete: Some(self.segment_complete),
                segment_labels: None,
                message: None,
                final_answer: Some(self.output.final_answer),
            };
            self.output.frames.push(frame);
        }

        pub fn finalize_and_print(&self) {
            let json = serde_json::to_string(&self.output).expect("Failed to serialize output");
            println!("{}", json);
        }
    }
}
