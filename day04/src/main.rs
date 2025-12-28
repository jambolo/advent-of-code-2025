// Advent of Code 2025, Day 4

use common::load;

#[cfg(feature = "instrumented")]
use instrumentation::Instrumentation;

fn main() {
    #[cfg(not(feature = "instrumented"))]
    println!(
        "Day 4, part {}",
        if cfg!(feature = "part2") { "2" } else { "1" }
    );

    let map = load::map();

    if cfg!(feature = "part2") {
        part2(&map);
    } else {
        part1(&map);
    }
}

fn part1(map: &[Vec<char>]) {
    let count = map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, cell)| **cell == '@' && count_neighbors(map, *x, y) < 4)
                .count()
        })
        .sum::<usize>();
    println!("Cells with less than 4 neighbors: {}", count);
}

fn count_neighbors(map: &[Vec<char>], x: usize, y: usize) -> usize {
    neighboring_cells(map, x, y).filter(|&c| c == '@').count()
}

fn neighboring_cells(map: &[Vec<char>], x: usize, y: usize) -> impl Iterator<Item = char> + '_ {
    let height = map.len();
    let width = map[0].len();
    let x0 = x.saturating_sub(1);
    let y0 = y.saturating_sub(1);
    let x1 = (x + 1).min(width - 1);
    let y1 = (y + 1).min(height - 1);
    (y0..=y1).flat_map(move |ny| {
        (x0..=x1)
            .filter(move |&nx| nx != x || ny != y)
            .map(move |nx| map[ny][nx])
    })
}

fn part2(map: &[Vec<char>]) {
    #[cfg(feature = "instrumented")]
    let mut inst = Instrumentation::new(map);

    let mut new_map = map.to_vec();
    let height = new_map.len();
    let width = new_map[0].len();

    #[cfg(feature = "instrumented")]
    inst.emit_initial(&new_map);

    // Let's try the naive approach
    let mut removed = 0;
    loop {
        #[cfg(feature = "instrumented")]
        inst.begin_pass();

        let previous_removed = removed;
        for y in 0..height {
            for x in 0..width {
                if new_map[y][x] == '@' && count_neighbors(&new_map, x, y) < 4 {
                    new_map[y][x] = '.';
                    removed += 1;
                    #[cfg(feature = "instrumented")]
                    inst.record_removal(x, y);
                }
            }
        }
        if removed == previous_removed {
            #[cfg(feature = "instrumented")]
            inst.emit_final(&new_map);
            break;
        }

        #[cfg(feature = "instrumented")]
        inst.end_pass(&new_map);
    }

    #[cfg(not(feature = "instrumented"))]
    println!("Total cells removed: {}", removed);

    #[cfg(feature = "instrumented")]
    inst.finalize_and_print();
}

#[cfg(feature = "instrumented")]
mod instrumentation {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Position {
        x: usize,
        y: usize,
    }

    #[derive(Serialize)]
    pub struct Frame {
        frame_type: String,
        pass_number: usize,
        grid: Vec<String>,
        removed_this_pass: Vec<Position>,
        removed_count: usize,
        total_removed: usize,
    }

    #[derive(Serialize)]
    pub struct Output {
        day: u8,
        part: u8,
        width: usize,
        height: usize,
        initial_rolls: usize,
        final_removed: usize,
        frames: Vec<Frame>,
    }

    pub struct Instrumentation {
        output: Output,
        pass_number: usize,
        total_removed: usize,
        removed_this_pass: Vec<Position>,
    }

    impl Instrumentation {
        pub fn new(map: &[Vec<char>]) -> Self {
            let height = map.len();
            let width = if height > 0 { map[0].len() } else { 0 };
            let initial_rolls = map.iter().flat_map(|row| row.iter()).filter(|&&c| c == '@').count();
            Self {
                output: Output {
                    day: 4,
                    part: 2,
                    width,
                    height,
                    initial_rolls,
                    final_removed: 0,
                    frames: Vec::new(),
                },
                pass_number: 0,
                total_removed: 0,
                removed_this_pass: Vec::new(),
            }
        }

        fn grid_to_strings(map: &[Vec<char>]) -> Vec<String> {
            map.iter().map(|row| row.iter().collect()).collect()
        }

        pub fn emit_initial(&mut self, map: &[Vec<char>]) {
            self.output.frames.push(Frame {
                frame_type: "initial".to_string(),
                pass_number: 0,
                grid: Self::grid_to_strings(map),
                removed_this_pass: Vec::new(),
                removed_count: 0,
                total_removed: 0,
            });
        }

        pub fn begin_pass(&mut self) {
            self.pass_number += 1;
            self.removed_this_pass.clear();
        }

        pub fn record_removal(&mut self, x: usize, y: usize) {
            self.total_removed += 1;
            self.removed_this_pass.push(Position { x, y });
        }

        pub fn end_pass(&mut self, map: &[Vec<char>]) {
            let removed_count = self.removed_this_pass.len();
            self.output.frames.push(Frame {
                frame_type: "pass_complete".to_string(),
                pass_number: self.pass_number,
                grid: Self::grid_to_strings(map),
                removed_this_pass: std::mem::take(&mut self.removed_this_pass),
                removed_count,
                total_removed: self.total_removed,
            });
        }

        pub fn emit_final(&mut self, map: &[Vec<char>]) {
            self.output.final_removed = self.total_removed;
            self.output.frames.push(Frame {
                frame_type: "final".to_string(),
                pass_number: self.pass_number,
                grid: Self::grid_to_strings(map),
                removed_this_pass: Vec::new(),
                removed_count: 0,
                total_removed: self.total_removed,
            });
        }

        pub fn finalize_and_print(&self) {
            let json = serde_json::to_string(&self.output).expect("Failed to serialize");
            println!("{}", json);
        }
    }
}
