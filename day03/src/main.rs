// Advent of Code 2025, Day 3

use common::load;

fn main() {
    #[cfg(not(feature = "instrumented"))]
    println!("Day 3, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    let banks = load::lines();

    #[cfg(feature = "instrumented")]
    let mut instrumentation = instrumentation::Instrumentation::new(banks.len());

    let mut joltage: u64 = 0;

    for (bank_index, bank) in banks.into_iter().enumerate() {
        #[cfg(not(feature = "instrumented"))]
        let _ = bank_index;

        let numbers: Vec<u32> = bank.chars().filter_map(|c| c.to_digit(10)).collect();

        let count = if cfg!(feature = "part2") { 12 } else { 2 };

        #[cfg(feature = "instrumented")]
        instrumentation.start_bank(bank_index, &bank, numbers.len(), joltage);

        let mut start = 0; // Current search start index
        let mut j: u64 = 0;
        for c in 0..count {
            let remaining_picks = count - c - 1;
            let (i, v) = next_digit(&numbers[start..], remaining_picks);

            #[cfg(feature = "instrumented")]
            instrumentation.set_window(numbers.len(), start, remaining_picks);

            start += i + 1;
            #[cfg(feature = "instrumented")]
            instrumentation.record_skip(bank_index, &bank, remaining_picks, joltage + j, i);
            j = j * 10 + v as u64;
            #[cfg(feature = "instrumented")]
            {
                let pick_index = start + i;
                instrumentation.record_pick(bank_index, &bank, pick_index, v, remaining_picks - 1, joltage + j);

                instrumentation.record_scan_window(bank_index, &bank, remaining_picks - 1, joltage + j);
            }
        }
        let bank_value = j;
        let bank_total = joltage + bank_value;

        #[cfg(feature = "instrumented")]
        instrumentation.complete_bank(bank_index, &bank, bank_value, bank_total);

        joltage = bank_total;
    }

    #[cfg(not(feature = "instrumented"))]
    println!("Total joltage: {}", joltage);

    #[cfg(feature = "instrumented")]
    instrumentation.finalize(joltage);
}

// Find the first number such that none of the following numbers are greater. Exclude the last n numbers.
fn next_digit(numbers: &[u32], n: usize) -> (usize, u32) {
    numbers
        .iter()
        .take(numbers.len().saturating_sub(n))
        .enumerate()
        .fold(
            (0, numbers[0]),
            |(max_i, max_v), (i, &v)| {
                if v > max_v { (i, v) } else { (max_i, max_v) }
            },
        )
}

#[cfg(feature = "instrumented")]
mod instrumentation {
    use serde::Serialize;

    const DIGITS_TO_PICK: usize = 12;
    const FRAME_CAP: usize = 6000;

    #[derive(Clone, Serialize)]
    #[serde(rename_all = "snake_case")]
    enum FrameType {
        BankStart,
        ScanWindow,
        Pick,
        Skip,
        BankComplete,
        Final,
    }

    #[derive(Clone, Serialize)]
    struct Frame {
        frame_type: FrameType,
        bank_index: usize,
        bank_digits: String,
        window_start: usize,
        window_end: usize,
        cursor: usize,
        remaining_picks: usize,
        chosen_so_far: String,
        locked_indices: Vec<usize>,
        running_total: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        bank_value: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        bank_output: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        banks_completed: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        skip_count: Option<usize>,
    }

    #[derive(Clone, Serialize)]
    struct Output {
        day: u8,
        part: u8,
        digits_to_pick: usize,
        total_banks: usize,
        final_total: u64,
        frames: Vec<Frame>,
    }
    pub struct Instrumentation {
        output: Output,
        frame_cap: usize,
        skip_frames_this_bank: usize,
        max_skip_frames_this_bank: usize,
        last_bank_complete: Option<Frame>,
        locked_indices: Vec<usize>,
        chosen_so_far: String,
        last_window_start: usize,
        last_window_end: usize,
        last_pick_index: usize,
        window_start: usize,
        window_end: usize,
    }

    impl Instrumentation {
        pub fn new(total_banks: usize) -> Self {
            Self {
                output: Output {
                    day: 3,
                    part: 2,
                    digits_to_pick: DIGITS_TO_PICK,
                    total_banks,
                    final_total: 0,
                    frames: Vec::new(),
                },
                frame_cap: FRAME_CAP,
                skip_frames_this_bank: 0,
                max_skip_frames_this_bank: 6,
                last_bank_complete: None,
                locked_indices: Vec::new(),
                chosen_so_far: String::new(),
                last_window_start: 0,
                last_window_end: 0,
                last_pick_index: 0,
                window_start: 0,
                window_end: 0,
            }
        }

        pub fn set_window(&mut self, bank_len: usize, window_start: usize, remaining: usize) {
            self.window_start = window_start;
            self.window_end = bank_len.saturating_sub(remaining).max(window_start);
        }

        pub fn start_bank(&mut self, bank_index: usize, bank_digits: &str, bank_len: usize, running_total: u64) {
            // Reset per-bank state
            self.max_skip_frames_this_bank = if bank_len > 200 { 4 } else { 6 };
            self.skip_frames_this_bank = 0;
            self.locked_indices.clear();
            self.chosen_so_far.clear();
            self.last_window_start = 0;
            self.last_window_end = 0;
            self.last_pick_index = 0;

            let window_end = bank_len.saturating_sub(DIGITS_TO_PICK);
            self.push_frame(
                Frame {
                    frame_type: FrameType::BankStart,
                    bank_index,
                    bank_digits: bank_digits.to_string(),
                    window_start: 0,
                    window_end,
                    cursor: window_end,
                    remaining_picks: DIGITS_TO_PICK,
                    chosen_so_far: String::new(),
                    locked_indices: Vec::new(),
                    running_total,
                    bank_value: None,
                    bank_output: None,
                    banks_completed: None,
                    skip_count: None,
                },
                true,
            );
        }

        pub fn record_skip(
            &mut self,
            bank_index: usize,
            bank_digits: &str,
            remaining_picks: usize,
            running_total: u64,
            skip_count: usize,
        ) {
            if skip_count <= 2 || self.skip_frames_this_bank >= self.max_skip_frames_this_bank {
                return;
            }

            self.skip_frames_this_bank += 1;

            self.push_frame(
                Frame {
                    frame_type: FrameType::Skip,
                    bank_index,
                    bank_digits: bank_digits.to_string(),
                    window_start: self.window_start,
                    window_end: self.window_end,
                    cursor: self.window_end,
                    remaining_picks,
                    chosen_so_far: self.chosen_so_far.clone(),
                    locked_indices: self.locked_indices.clone(),
                    running_total,
                    bank_value: None,
                    bank_output: None,
                    banks_completed: None,
                    skip_count: Some(skip_count),
                },
                false,
            );
        }

        pub fn record_pick(
            &mut self,
            bank_index: usize,
            bank_digits: &str,
            pick_index: usize,
            digit: u32,
            remaining_after: usize,
            running_total: u64,
        ) {
            // Update internal state
            let digit_char = std::char::from_digit(digit, 10).unwrap();
            self.chosen_so_far.push(digit_char);
            self.locked_indices.push(pick_index);
            self.last_window_start = self.window_start;
            self.last_window_end = self.window_end;
            self.last_pick_index = pick_index;

            self.push_frame(
                Frame {
                    frame_type: FrameType::Pick,
                    bank_index,
                    bank_digits: bank_digits.to_string(),
                    window_start: self.window_start,
                    window_end: self.window_end,
                    cursor: pick_index,
                    remaining_picks: remaining_after,
                    chosen_so_far: self.chosen_so_far.clone(),
                    locked_indices: self.locked_indices.clone(),
                    running_total,
                    bank_value: None,
                    bank_output: None,
                    banks_completed: None,
                    skip_count: None,
                },
                true,
            );
        }

        pub fn record_scan_window(&mut self, bank_index: usize, bank_digits: &str, remaining_after: usize, running_total: u64) {
            if remaining_after > 0 {
                self.push_frame(
                    Frame {
                        frame_type: FrameType::ScanWindow,
                        bank_index,
                        bank_digits: bank_digits.to_string(),
                        window_start: self.window_start,
                        window_end: self.window_end,
                        cursor: self.window_end,
                        remaining_picks: remaining_after,
                        chosen_so_far: self.chosen_so_far.clone(),
                        locked_indices: self.locked_indices.clone(),
                        running_total,
                        bank_value: None,
                        bank_output: None,
                        banks_completed: None,
                        skip_count: None,
                    },
                    false,
                );
            }
        }

        pub fn complete_bank(&mut self, bank_index: usize, bank_digits: &str, bank_value: u64, running_total: u64) {
            let frame = Frame {
                frame_type: FrameType::BankComplete,
                bank_index,
                bank_digits: bank_digits.to_string(),
                window_start: self.last_window_start,
                window_end: self.last_window_end,
                cursor: self.last_pick_index,
                remaining_picks: 0,
                chosen_so_far: self.chosen_so_far.clone(),
                locked_indices: self.locked_indices.clone(),
                running_total,
                bank_value: Some(bank_value),
                bank_output: Some(self.chosen_so_far.clone()),
                banks_completed: Some(bank_index + 1),
                skip_count: None,
            };

            self.push_frame(frame.clone(), true);
            self.last_bank_complete = Some(frame);
        }

        pub fn finalize(&mut self, final_total: u64) {
            self.output.final_total = final_total;

            if let Some(mut last_bank_complete) = self.last_bank_complete.clone() {
                last_bank_complete.frame_type = FrameType::Final;
                last_bank_complete.running_total = final_total;
                last_bank_complete.banks_completed = Some(self.output.total_banks);
                self.push_frame(last_bank_complete, true);
            }

            if let Ok(json) = serde_json::to_string(&self.output) {
                println!("{}", json);
            }
        }

        fn push_frame(&mut self, frame: Frame, always: bool) {
            if always || self.output.frames.len() < self.frame_cap {
                self.output.frames.push(frame);
            }
        }
    }
}
