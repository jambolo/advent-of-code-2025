// Advent of Code 2025, Day 5

use common::load;

fn main() {
    #[cfg(not(feature = "instrumented"))]
    println!(
        "Day 5, part {}",
        if cfg!(feature = "part2") { "2" } else { "1" }
    );

    let lines = load::lines();

    // Split the lines into two sections: fresh ranges and ingredient IDs
    let mut sections = lines.split(|line| line.is_empty());

    // Parse the fresh ingredient ranges
    let fresh_ranges: Vec<(i64, i64)> = sections
        .next()
        .expect("Missing fresh ingredient ranges section")
        .iter()
        .map(|line| parse_range(line))
        .collect();

    // Parse the ingredient IDs
    let ingredient_ids: Vec<i64> = sections
        .next()
        .expect("Missing ingredient IDs section")
        .iter()
        .map(|line| line.parse().expect("Failed to parse ingredient ID as i64"))
        .collect();

    if cfg!(feature = "part2") {
        part2(&fresh_ranges);
    } else {
        part1(&fresh_ranges, &ingredient_ids);
    }
}

fn part1(fresh_ranges: &[(i64, i64)], ingredient_ids: &[i64]) {
    let count = ingredient_ids
        .iter()
        .filter(|&&id| is_fresh(fresh_ranges, id))
        .count();
    println!("Number of fresh ingredients: {}", count);
}

fn is_fresh(fresh_ranges: &[(i64, i64)], id: i64) -> bool {
    fresh_ranges
        .iter()
        .any(|&(start, end)| id >= start && id <= end)
}

fn part2(ranges: &[(i64, i64)]) {
    #[cfg(feature = "instrumented")]
    let mut inst = instrumentation::Instrumentation::new(ranges);

    #[cfg(feature = "instrumented")]
    inst.emit_initial();

    let mut sorted = ranges.to_vec();
    #[cfg(feature = "instrumented")]
    inst.compute_sorted_indices(&sorted);

    sorted.sort_unstable_by_key(|&(start, _)| start);

    #[cfg(feature = "instrumented")]
    inst.emit_sorted();

    #[cfg(feature = "instrumented")]
    inst.start_merge();

    let mut merged: Vec<(i64, i64)> = Vec::new();
    for (start, end) in sorted {
        if let Some(last) = merged.last_mut()
            && start <= last.1 + 1
        {
            last.1 = last.1.max(end); // Merge overlapping ranges
            #[cfg(feature = "instrumented")]
            inst.emit_merge_step("merged");
            continue; // Don't push a new range
        }
        merged.push((start, end));
        #[cfg(feature = "instrumented")]
        inst.emit_merge_step("new");
    }

    let total_fresh: i64 = merged.iter().map(|&(start, end)| end - start + 1).sum();

    #[cfg(feature = "instrumented")]
    inst.emit_final(total_fresh);

    #[cfg(feature = "instrumented")]
    inst.finalize_and_print();

    #[cfg(not(feature = "instrumented"))]
    println!("Total number of fresh ingredients: {}", total_fresh);
}

fn parse_range(line: &str) -> (i64, i64) {
    let (start, end) = line
        .split_once('-')
        .expect("Failed to split range line on '-'");

    (
        start.parse().expect("Failed to parse range start as i64"),
        end.parse().expect("Failed to parse range end as i64"),
    )
}

#[cfg(feature = "instrumented")]
mod instrumentation {
    use serde::Serialize;

    #[derive(Serialize, Clone)]
    pub struct Range {
        start: String,
        end: String,
        original_index: usize,
    }

    #[derive(Serialize, Clone)]
    pub struct Frame {
        frame_type: String,
        step_index: usize,
        ranges: Vec<Range>,
        #[serde(skip_serializing_if = "Option::is_none")]
        current_index: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        action: Option<String>,
        merged_ranges: Vec<Range>,
        running_total: String,
    }

    #[derive(Serialize)]
    pub struct Output {
        day: u8,
        part: u8,
        range_count: usize,
        global_min: String,
        global_max: String,
        total_fresh: String,
        merged_count: usize,
        frames: Vec<Frame>,
    }

    pub struct Instrumentation {
        original_ranges: Vec<(i64, i64)>,
        global_min: i64,
        global_max: i64,
        frames: Vec<Frame>,
        sorted_indices: Vec<usize>,
        processed_count: usize,
    }

    impl Instrumentation {
        pub fn new(ranges: &[(i64, i64)]) -> Self {
            let global_min = ranges.iter().map(|&(s, _)| s).min().unwrap_or(0);
            let global_max = ranges.iter().map(|&(_, e)| e).max().unwrap_or(0);
            Self {
                original_ranges: ranges.to_vec(),
                global_min,
                global_max,
                frames: Vec::new(),
                sorted_indices: Vec::new(),
                processed_count: 0,
            }
        }

        pub fn compute_sorted_indices(&mut self, unsorted: &[(i64, i64)]) {
            let mut indexed: Vec<(usize, (i64, i64))> = unsorted.iter().cloned().enumerate().collect();
            indexed.sort_unstable_by_key(|&(_, (start, _))| start); // Duplicate sort to find original indices
            self.sorted_indices = indexed.iter().map(|&(i, _)| i).collect();
        }

        pub fn emit_initial(&mut self) {
            let ranges = self.ranges_to_json_with_indices(&self.original_ranges, &(0..self.original_ranges.len()).collect::<Vec<_>>());
            self.frames.push(Frame {
                frame_type: "initial".to_string(),
                step_index: 0,
                ranges,
                current_index: None,
                action: None,
                merged_ranges: Vec::new(),
                running_total: "0".to_string(),
            });
        }

        pub fn emit_sorted(&mut self) {
            let sorted_ranges: Vec<(i64, i64)> = self
                .sorted_indices
                .iter()
                .map(|&i| self.original_ranges[i])
                .collect();
            self.frames.push(Frame {
                frame_type: "sorted".to_string(),
                step_index: 0,
                ranges: self.ranges_to_json_with_indices(&sorted_ranges, &self.sorted_indices),
                current_index: None,
                action: None,
                merged_ranges: Vec::new(),
                running_total: "0".to_string(),
            });
        }

        pub fn start_merge(&mut self) {
            self.processed_count = 0;
        }

        pub fn emit_merge_step(&mut self, action: &str) {
            self.processed_count += 1;
            let sorted_ranges: Vec<(i64, i64)> = self.sorted_indices.iter().map(|&i| self.original_ranges[i]).collect();
            let merged_ranges = self.compute_merged_ranges();
            let running_total: i64 = merged_ranges.iter().map(|r| r.end.parse::<i64>().unwrap() - r.start.parse::<i64>().unwrap() + 1).sum();
            self.frames.push(Frame {
                frame_type: "merge_step".to_string(),
                step_index: self.processed_count,
                ranges: self.ranges_to_json_with_indices(&sorted_ranges, &self.sorted_indices),
                current_index: Some(self.processed_count - 1),
                action: Some(action.to_string()),
                merged_ranges,
                running_total: running_total.to_string(),
            });
        }

        pub fn emit_final(&mut self, total_fresh: i64) {
            self.processed_count = self.sorted_indices.len();
            let sorted_ranges: Vec<(i64, i64)> = self.sorted_indices.iter().map(|&i| self.original_ranges[i]).collect();
            let merged_ranges = self.compute_merged_ranges();
            self.frames.push(Frame {
                frame_type: "final".to_string(),
                step_index: self.sorted_indices.len() + 1,
                ranges: self.ranges_to_json_with_indices(&sorted_ranges, &self.sorted_indices),
                current_index: None,
                action: None,
                merged_ranges,
                running_total: total_fresh.to_string(),
            });
        }

        pub fn finalize_and_print(&self) {
            let last_frame = self.frames.last().unwrap();
            let output = Output {
                day: 5,
                part: 2,
                range_count: self.original_ranges.len(),
                global_min: self.global_min.to_string(),
                global_max: self.global_max.to_string(),
                total_fresh: last_frame.running_total.clone(),
                merged_count: last_frame.merged_ranges.len(),
                frames: self.frames.clone(),
            };
            println!("{}", serde_json::to_string(&output).unwrap());
        }

        fn compute_merged_ranges(&self) -> Vec<Range> {
            let mut merged: Vec<(i64, i64, usize)> = Vec::new();
            for (i, &orig_idx) in self.sorted_indices.iter().enumerate() {
                if i >= self.processed_count { break; }
                let (start, end) = self.original_ranges[orig_idx];
                if let Some(last) = merged.last_mut()
                    && start <= last.1 + 1
                {
                    last.1 = last.1.max(end);
                    continue;
                }
                merged.push((start, end, orig_idx));
            }
            merged.iter().map(|&(s, e, idx)| Range {
                start: s.to_string(),
                end: e.to_string(),
                original_index: idx,
            }).collect()
        }

        fn ranges_to_json_with_indices(&self, ranges: &[(i64, i64)], indices: &[usize]) -> Vec<Range> {
            ranges
                .iter()
                .zip(indices.iter())
                .map(|(&(start, end), &original_index)| Range {
                    start: start.to_string(),
                    end: end.to_string(),
                    original_index,
                })
                .collect()
        }
    }
}
