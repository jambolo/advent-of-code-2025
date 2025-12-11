// Advent of Code 2025, Day 2

use common::load;

fn main() {
    #[cfg(not(feature = "instrumented"))]
    println!("Day 2, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    // Load the data
    let input = load::string();

    // Parse the input into a vector of ranges.
    let ranges = parse_ranges(&input);

    if cfg!(feature = "part2")
    {
        part2(&ranges);
    } else {
        part1(&ranges);
    }
}

fn part2(ranges: &[(i64, i64)]) {
    #[cfg(feature = "instrumented")]
    let mut instrumentation = instrumentation::Instrumentation::new();
    #[cfg(feature = "instrumented")]
    instrumentation.set_ranges(ranges);

    let mut sum: i64 = 0;
    let mut last_range: Option<(usize, i64, i64)> = None;

    for (range_index, (start, end)) in ranges.iter().copied().enumerate() {
        last_range = Some((range_index, start, end));

        #[cfg(feature = "instrumented")]
        instrumentation.record_range_start(range_index, start, end, sum);

        for number in start..=end {
            #[cfg(feature = "instrumented")]
            instrumentation.increment_inspected();
            let num_str = number.to_string();
            let len = num_str.len();

            #[cfg(feature = "instrumented")]
            instrumentation.maybe_scan_tick(
                range_index,
                start,
                end,
                number,
                &num_str,
                sum,
            );

            for repeat_count in 2..=len {
                if len % repeat_count != 0 {
                    continue;
                }

                let candidate_length = len / repeat_count;
                let matched = duplicated(&num_str, candidate_length);

//                #[cfg(feature = "instrumented")]
//                instrumentation.record_pattern_check(
//                    range_index,
//                    start,
//                    end,
//                    number,
//                    &num_str,
//                    sum,
//                    repeat_count,
//                    candidate_length,
//                    &num_str[..candidate_length],
//                    matched.is_some(),
//                );

                if let Some(candidate) = matched {
                    sum += number;
                    instrumentation.increment_range_invalids();
                    instrumentation.increment_global_invalids();

                    #[cfg(feature = "instrumented")]
                    instrumentation.record_invalid_hit(
                        range_index,
                        start,
                        end,
                        number,
                        &num_str,
                        sum,
                        repeat_count,
                        candidate_length,
                        &candidate,
                    );
                    break;
                }
            }
        }

        #[cfg(feature = "instrumented")]
        instrumentation.record_range_end(
            range_index,
            start,
            end,
            sum,
        );
    }

    #[cfg(not(feature = "instrumented"))]
    println!("Sum: {}", sum);

    #[cfg(feature = "instrumented")]
    if let Some((range_index, start, end)) = last_range {
        instrumentation.record_final_summary(range_index, start, end, sum);
        instrumentation.finalize_and_print();
    }
}

fn part1(ranges: &[(i64, i64)]) {
    let mut sum: i64 = 0;
    for (start, end) in ranges {
        for number in *start..=*end {
            let num_str = number.to_string();
            if duplicated(&num_str,num_str.len() / 2).is_some() {
                sum += number;
            }
        }
    }
    println!("Sum: {}", sum);
}

/// Parses a string of comma-separated ranges into a vector of (start, end) tuples.
fn parse_ranges(input: &str) -> Vec<(i64, i64)> {
    input
        .trim()
        .split(',')
        .map(|range_str| {
            let endpoints: Vec<&str> = range_str.split('-').collect();
            if endpoints.len() != 2 {
                panic!("Invalid range format: {}", range_str);
            }
            let start = endpoints[0].parse::<i64>().expect("Invalid start of range");
            let end = endpoints[1].parse::<i64>().expect("Invalid end of range");
            (start, end)
        })
        .collect()
}

/// Returns true if all parts of size n are the same
fn duplicated(s: &str, n: usize) -> Option<String> {
    let first_chunk = &s.as_bytes()[..n];
    let yes = s.as_bytes()
        .chunks(n)
        .all(|chunk| chunk == first_chunk);
    if yes {
        Some(s[..n].to_string())
    } else {
        None
    }
}

#[cfg(feature = "instrumented")]
mod instrumentation {
    use serde::Serialize;

    pub const SAMPLING_STRIDE: usize = 500;

    #[derive(Serialize)]
    pub struct Instrumentation {
        puzzle_day: u8,
        part: u8,
        #[serde(skip_serializing_if = "Option::is_none")]
        sampling_stride: Option<usize>,
        ranges: Vec<RangeDescriptor>,
        frames: Vec<Frame>,
        final_sum: i64,
        range_invalids: usize,
        global_invalids: usize,
        inspected: usize,
    }

    impl Instrumentation {
        pub fn new() -> Self {
            Self {
                puzzle_day: 2,
                part: 2,
                sampling_stride: Some(SAMPLING_STRIDE),
                ranges: Vec::new(),
                frames: Vec::new(),
                final_sum: 0,
                range_invalids: 0,
                global_invalids: 0,
                inspected: 0,
            }
        }

        pub fn set_ranges(&mut self, ranges: &[(i64, i64)]) {
            self.ranges = ranges
                .iter()
                .enumerate()
                .map(|(index, (start, end))| RangeDescriptor {
                    index,
                    start: *start,
                    end: *end,
                    label: None,
                })
                .collect();
        }

        pub fn increment_global_invalids(&mut self) {
            self.global_invalids += 1;
        }

        pub fn increment_range_invalids(&mut self) {
            self.range_invalids += 1;
        }

        pub fn increment_inspected(&mut self) {
            self.inspected += 1;
        }

        fn compute_range_progress(&self, number: usize, start: i64, end: i64) -> f64 {
            if start == end {
                return 1.0;
            }
            let traveled = (number as i64 - start) as f64;
            let span = (end - start) as f64;
            (traveled / span).clamp(0.0, 1.0)
        }

        pub fn maybe_scan_tick(
            &mut self,
            range_index: usize,
            range_start: i64,
            range_end: i64,
            number: i64,
            digits: &str,
            global_sum: i64,
        ) {
            if (self.inspected + SAMPLING_STRIDE - 1) % SAMPLING_STRIDE != 0 {
                return;
            }

            let range_progress = self.compute_range_progress(number as usize, range_start, range_end);
            self.frames.push(Frame {
                frame_type: "scan_tick",
                range_index,
                range_start,
                range_end,
                range_progress,
                global_sum,
                global_invalids: self.global_invalids,
                range_invalids: Some(self.range_invalids),
                inspected: Some(self.inspected),
                message: None,
                number: Some(number),
                digits: Some(digits.to_string()),
                repeat_count: None,
                chunk_length: None,
                candidate_chunk: None,
                r#match: None,
            });
        }

        pub fn record_range_start(
            &mut self,
            range_index: usize,
            range_start: i64,
            range_end: i64,
            global_sum: i64,
        ) {
            self.frames.push(Frame {
                frame_type: "range_start",
                range_index,
                range_start,
                range_end,
                range_progress: 0.0,
                global_sum,
                global_invalids: self.global_invalids,
                range_invalids: Some(0),
                inspected: Some(0),
                message: None,
                number: None,
                digits: None,
                repeat_count: None,
                chunk_length: None,
                candidate_chunk: None,
                r#match: None,
            });
        }

//        pub fn record_pattern_check(
//            &mut self,
//            range_index: usize,
//            range_start: i64,
//            range_end: i64,
//            number: i64,
//            digits: &str,
//            global_sum: i64,
//            repeat_count: usize,
//            candidate_length: usize,
//            candidate: &str,
//            is_match: bool,
//        ) {
//            let range_progress = self.compute_range_progress(range_index, range_start, range_end);
//            self.frames.push(Frame {
//                frame_type: "pattern_check",
//                range_index,
//                range_start,
//                range_end,
//                range_progress,
//                global_sum,
//                global_invalids: self.global_invalids,
//                range_invalids: Some(self.range_invalids),
//                inspected: Some(self.inspected),
//                message: None,
//                number: Some(number),
//                digits: Some(digits.to_string()),
//                repeat_count: Some(repeat_count),
//                chunk_length: Some(candidate_length),
//                candidate_chunk: Some(candidate.to_string()),
//                r#match: Some(is_match),
//            });
//        }

        pub fn record_invalid_hit(
            &mut self,
            range_index: usize,
            range_start: i64,
            range_end: i64,
            number: i64,
            digits: &str,
            global_sum: i64,
            repeat_count: usize,
            candidate_length: usize,
            candidate: &str,
        ) {
            self.final_sum = global_sum;

            let range_progress = self.compute_range_progress(range_index, range_start, range_end);
            self.frames.push(Frame {
                frame_type: "invalid_hit",
                range_index,
                range_start,
                range_end,
                range_progress,
                global_sum,
                global_invalids: self.global_invalids,
                range_invalids: Some(self.range_invalids),
                inspected: Some(self.inspected),
                message: None,
                number: Some(number),
                digits: Some(digits.to_string()),
                repeat_count: Some(repeat_count),
                chunk_length: Some(candidate_length),
                candidate_chunk: Some(candidate.to_string()),
                r#match: Some(true),
            });
        }

        pub fn record_range_end(
            &mut self,
            range_index: usize,
            range_start: i64,
            range_end: i64,
            global_sum: i64,
        ) {
            self.frames.push(Frame {
                frame_type: "range_end",
                range_index,
                range_start,
                range_end,
                range_progress: 1.0,
                global_sum,
                global_invalids: self.global_invalids,
                range_invalids: Some(self.range_invalids),
                inspected: Some(self.inspected),
                message: None,
                number: None,
                digits: None,
                repeat_count: None,
                chunk_length: None,
                candidate_chunk: None,
                r#match: None,
            });
        }

        pub fn record_final_summary(
            &mut self,
            range_index: usize,
            range_start: i64,
            range_end: i64,
            final_sum: i64,
        ) {
            self.final_sum = final_sum;

            self.frames.push(Frame {
                frame_type: "final_summary",
                range_index,
                range_start,
                range_end,
                range_progress: 1.0,
                global_sum: final_sum,
                global_invalids: self.global_invalids,
                range_invalids: None,
                inspected: None,
                message: None,
                number: None,
                digits: None,
                repeat_count: None,
                chunk_length: None,
                candidate_chunk: None,
                r#match: None,
            });
        }

        pub fn finalize_and_print(&self) {
            let json = serde_json::to_string(self).expect("Failed to serialize instrumentation");
            println!("{}", json);
        }
    }

    #[derive(Serialize)]
    struct RangeDescriptor {
        index: usize,
        start: i64,
        end: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
    }

    #[derive(Serialize)]
    struct Frame {
        frame_type: &'static str,
        range_index: usize,
        range_start: i64,
        range_end: i64,
        range_progress: f64,
        global_sum: i64,
        global_invalids: usize,
        #[serde(skip_serializing_if = "Option::is_none")]
        range_invalids: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        inspected: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        message: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        number: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        digits: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        repeat_count: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        chunk_length: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        candidate_chunk: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        r#match: Option<bool>,
    }
}
