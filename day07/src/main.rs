// Advent of Code 2025, Day 7

use common::load;

fn main() {
    #[cfg(not(feature = "instrumented"))]
    println!("Day 7, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    let map = load::map();

    if cfg!(feature = "part2") {
        part2(&map);
    } else {
        part1(&map);
    }
}

fn part1(map: &[Vec<char>]) {
    use std::collections::HashSet;

    let width = map[0].len();

    // List of columns with beams
    let mut beams = HashSet::new();

    // Number of splitters encountered
    let mut count: i64 = 0;

    // Beams move down the map
    for row in map {
        // Search for a source before doing anything else
        if beams.is_empty() {
            if let Some(x) = row.iter().position(|&c| c == 'S') {
                beams.insert(x);
            }
            continue;
        }

        for x in beams.iter().cloned().collect::<Vec<_>>() {
            if row[x] == '^' {
                count += 1;
                beams.remove(&x);
                if x > 0 {
                    beams.insert(x - 1);
                }
                if x + 1 < width {
                    beams.insert(x + 1);
                }
            }
        }
    }
    println!("Number of splitters encountered: {}", count);
}

fn part2(map: &[Vec<char>]) {
    use std::collections::HashMap;

    #[cfg(feature = "instrumented")]
    let mut inst = instrumentation::Instrumentation::new(map);

    let width = map[0].len();

    // List of timelines in the form of beams columns and their timeline counts
    let mut timelines: HashMap<usize, i64> = HashMap::new();

    // Beams move down the map
    for row in map {
        // Search for a source before doing anything else
        if timelines.is_empty() {
            if let Some(x) = row.iter().position(|&c| c == 'S') {
                timelines.insert(x, 1);
                #[cfg(feature = "instrumented")]
                inst.emit_initial_frame(x);
            }
            continue;
        }

        #[cfg(feature = "instrumented")]
        inst.begin_row();

        // Scan each beam column for splitters in this row
        for x in timelines.keys().cloned().collect::<Vec<_>>() {
            if row[x] == '^' {
                let count = timelines.remove(&x).expect("Timeline key not found during removal");
                #[cfg(feature = "instrumented")]
                inst.record_split(
                    x,
                    count,
                    if x > 0 { Some(x - 1) } else { None },
                    if x < width - 1 { Some(x + 1) } else { None },
                );

                if x > 0 {
                    *timelines.entry(x - 1).or_insert(0) += count;
                }
                if x < width - 1 {
                    *timelines.entry(x + 1).or_insert(0) += count;
                }
            }
        }

        #[cfg(feature = "instrumented")]
        inst.end_row(&timelines);
    }

    // Count the total number of timelines
    let final_timelines: i64 = timelines.values().sum();
    #[cfg(feature = "instrumented")]
    inst.emit_final_frame(&timelines, final_timelines);

    #[cfg(not(feature = "instrumented"))]
    println!("Total number of timelines: {}", final_timelines);

    #[cfg(feature = "instrumented")]
    inst.finalize_and_print();
}

#[cfg(feature = "instrumented")]
mod instrumentation {
    use serde::Serialize;
    use std::collections::HashMap;

    #[derive(Serialize)]
    pub struct BeamState {
        column: usize,
        timelines: i64,
    }

    #[derive(Serialize)]
    pub struct SplitEvent {
        column: usize,
        timelines_before: i64,
        left_column: usize,
        right_column: usize,
    }

    #[derive(Serialize)]
    pub struct Frame {
        frame_type: String,
        row: usize,
        beams: Vec<BeamState>,
        #[serde(skip_serializing_if = "Option::is_none")]
        splits: Option<Vec<SplitEvent>>,
        total_timelines: i64,
        splits_count: usize,
    }

    #[derive(Serialize)]
    pub struct LogOutput {
        width: usize,
        height: usize,
        source_column: usize,
        splitter_positions: Vec<(usize, usize)>,
        frames: Vec<Frame>,
        final_timelines: i64,
    }

    pub struct Instrumentation {
        width: usize,
        height: usize,
        source_column: usize,
        splitter_positions: Vec<(usize, usize)>,
        frames: Vec<Frame>,
        current_row: usize,
        current_splits: Vec<SplitEvent>,
        splits_count: usize,
        row_had_splits: bool,
    }

    impl Instrumentation {
        pub fn new(map: &[Vec<char>]) -> Self {
            let height = map.len();
            let width = if height > 0 { map[0].len() } else { 0 };

            let mut splitter_positions = Vec::new();
            for (row_idx, row) in map.iter().enumerate() {
                for (col_idx, &ch) in row.iter().enumerate() {
                    if ch == '^' {
                        splitter_positions.push((row_idx, col_idx));
                    }
                }
            }

            Self {
                width,
                height,
                source_column: 0,
                splitter_positions,
                frames: Vec::new(),
                current_row: 0,
                current_splits: Vec::new(),
                splits_count: 0,
                row_had_splits: false,
            }
        }

        pub fn emit_initial_frame(&mut self, source_col: usize) {
            self.source_column = source_col;
            self.frames.push(Frame {
                frame_type: "initial".to_string(),
                row: self.current_row,
                beams: vec![BeamState {
                    column: source_col,
                    timelines: 1,
                }],
                splits: None,
                total_timelines: 1,
                splits_count: 0,
            });
        }

        pub fn begin_row(&mut self) {
            self.current_splits.clear();
            self.row_had_splits = false;
        }

        pub fn record_split(&mut self, column: usize, timelines_before: i64, left_col: Option<usize>, right_col: Option<usize>) {
            self.splits_count += 1;
            self.row_had_splits = true;

            let left_column = left_col.unwrap_or(0);
            let right_column = right_col.unwrap_or(self.width.saturating_sub(1));

            self.current_splits.push(SplitEvent {
                column,
                timelines_before,
                left_column,
                right_column,
            });
        }

        pub fn end_row(&mut self, timelines: &HashMap<usize, i64>) {
            if self.row_had_splits {
                let mut beams: Vec<BeamState> = timelines
                    .iter()
                    .map(|(&col, &tl)| BeamState {
                        column: col,
                        timelines: tl,
                    })
                    .collect();
                beams.sort_by_key(|b| b.column);

                let total_timelines: i64 = timelines.values().sum();

                self.frames.push(Frame {
                    frame_type: "row_process".to_string(),
                    row: self.current_row,
                    beams,
                    splits: Some(std::mem::take(&mut self.current_splits)),
                    total_timelines,
                    splits_count: self.splits_count,
                });
            }
            self.current_row += 1;
        }

        pub fn emit_final_frame(&mut self, timelines: &HashMap<usize, i64>, final_timelines: i64) {
            let mut beams: Vec<BeamState> = timelines
                .iter()
                .map(|(&col, &tl)| BeamState {
                    column: col,
                    timelines: tl,
                })
                .collect();
            beams.sort_by_key(|b| b.column);

            self.frames.push(Frame {
                frame_type: "final".to_string(),
                row: self.height.saturating_sub(1),
                beams,
                splits: None,
                total_timelines: final_timelines,
                splits_count: self.splits_count,
            });
        }

        pub fn finalize_and_print(&self) {
            let output = LogOutput {
                width: self.width,
                height: self.height,
                source_column: self.source_column,
                splitter_positions: self.splitter_positions.clone(),
                frames: self
                    .frames
                    .iter()
                    .map(|f| Frame {
                        frame_type: f.frame_type.clone(),
                        row: f.row,
                        beams: f
                            .beams
                            .iter()
                            .map(|b| BeamState {
                                column: b.column,
                                timelines: b.timelines,
                            })
                            .collect(),
                        splits: f.splits.as_ref().map(|s| {
                            s.iter()
                                .map(|se| SplitEvent {
                                    column: se.column,
                                    timelines_before: se.timelines_before,
                                    left_column: se.left_column,
                                    right_column: se.right_column,
                                })
                                .collect()
                        }),
                        total_timelines: f.total_timelines,
                        splits_count: f.splits_count,
                    })
                    .collect(),
                final_timelines: self.frames.last().map(|f| f.total_timelines).unwrap_or(0),
            };

            println!("{}", serde_json::to_string(&output).expect("Failed to serialize JSON"));
        }
    }
}
