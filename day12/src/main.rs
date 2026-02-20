// Advent of Code 2025, Day 12

use common::load;

#[cfg(feature = "instrumented")]
use instrumentation::Instrumentation;

#[derive(Debug)]
struct Package {
    #[cfg(feature = "instrumented")]
    pattern: [[char; 3]; 3],
    area: usize,
}

impl Package {
    fn new(base_shape: [[char; 3]; 3]) -> Self {
        // Count the number of filled cells
        let area = base_shape.iter().flatten().filter(|&&c| c == '#').count();

        Package {
            #[cfg(feature = "instrumented")]
            pattern: base_shape,
            area,
        }
    }
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    counts: Vec<usize>,
}

fn main() {
    #[cfg(not(feature = "instrumented"))]
    println!("Day 12, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    let lines = load::lines();
    let (packages, regions) = parse_input(&lines);

    #[cfg(feature = "instrumented")]
    let mut inst = Instrumentation::new(&packages, regions.len());

    // Count the number of regions that can fit all packages
    let mut rejected: i64 = 0;
    let mut accepted: i64 = 0;

    #[cfg(feature = "instrumented")]
    inst.reset_region_index();

    for region in &regions {
        let region_area = region.width * region.height;
        let total_packages: usize = region.counts.iter().sum();

        // See if the region's has enough space for the areas of all packages.
        // Reject those that don't.
        let mut packages_area = 0;
        for (i, &c) in region.counts.iter().enumerate() {
            packages_area += c * packages[i].area;
        }

        #[cfg(feature = "instrumented")]
        inst.region_start(region.width, region.height, &region.counts, packages_area, total_packages);

        if packages_area > region_area {
            rejected += 1;
            #[cfg(feature = "instrumented")]
            inst.verdict(
                "rejected",
                packages_area,
                region_area,
                0,
                total_packages,
                accepted as usize,
                rejected as usize,
            );
            #[cfg(feature = "instrumented")]
            inst.next_region_index();
            continue;
        }

        #[cfg(feature = "instrumented")]
        inst.area_check(packages_area, region_area);

        // Packages are at most 3x3, so check if the region has enough 3x3 slots for all packages.
        // Accept all that do.
        let num_slots = region.width / 3 * (region.height / 3);

        #[cfg(feature = "instrumented")]
        inst.slot_check(num_slots, total_packages);

        if total_packages <= num_slots {
            accepted += 1;
            #[cfg(feature = "instrumented")]
            inst.verdict(
                "accepted",
                packages_area,
                region_area,
                num_slots,
                total_packages,
                accepted as usize,
                rejected as usize,
            );
        } else {
            #[cfg(feature = "instrumented")]
            inst.verdict(
                "undetermined",
                packages_area,
                region_area,
                num_slots,
                total_packages,
                accepted as usize,
                rejected as usize,
            );
        }
        #[cfg(feature = "instrumented")]
        inst.next_region_index();
    }

    #[cfg(not(feature = "instrumented"))]
    {
        println!("Rejected: {}", rejected);
        println!("Accepted: {}", accepted);
        println!("Undetermined: {}", regions.len() as i64 - accepted - rejected);
    }

    #[cfg(feature = "instrumented")]
    inst.finalize_and_print(accepted as usize);
}

fn parse_input(lines: &[String]) -> (Vec<Package>, Vec<Region>) {
    let mut packages = Vec::new();
    let mut i = 0;

    // Parse packages
    while i < lines.len() {
        // Skip blank lines between packages
        while i < lines.len() && lines[i].trim().is_empty() {
            i += 1;
        }
        if i >= lines.len() || lines[i].contains('x') {
            break;
        }
        let id = parse_package_header(&lines[i]);
        i += 1; // Move past header
        let shape = parse_package_shape(&lines[i..i + 3]);
        i += 3; // Move past shape
        assert!(packages.len() == id);
        packages.push(Package::new(shape));
    }

    // Parse regions
    let regions = lines[i..].iter().map(|line| parse_region(line)).collect();

    (packages, regions)
}

fn parse_region(line: &str) -> Region {
    let (dims_str, counts_str) = line.split_once(':').expect("Invalid region format");
    let (width, height) = parse_region_dimensions(dims_str);
    let counts = parse_region_counts(counts_str);
    Region { width, height, counts }
}

fn parse_region_counts(counts_str: &str) -> Vec<usize> {
    counts_str
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid package count"))
        .collect()
}

fn parse_region_dimensions(dims_str: &str) -> (usize, usize) {
    let (width_str, height_str) = dims_str.trim().split_once('x').expect("Invalid region dimensions");
    let width: usize = width_str.parse().expect("Invalid width");
    let height: usize = height_str.parse().expect("Invalid height");
    (width, height)
}

fn parse_package_shape(lines: &[String]) -> [[char; 3]; 3] {
    let mut grid = [['.'; 3]; 3];
    for (i, row) in lines.iter().enumerate() {
        let chars: Vec<char> = row.chars().collect();
        grid[i].copy_from_slice(&chars[..3]);
    }
    grid
}

fn parse_package_header(line: &str) -> usize {
    line.trim_end_matches(':').parse().expect("Invalid package id")
}

#[cfg(feature = "instrumented")]
mod instrumentation {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Shape {
        id: usize,
        pattern: Vec<String>,
        area: usize,
    }

    #[derive(Serialize)]
    #[serde(tag = "frameType")]
    pub enum Frame {
        #[serde(rename = "intro")]
        Intro { message: String },
        #[serde(rename = "regionStart")]
        RegionStart {
            #[serde(rename = "regionIndex")]
            region_index: usize,
            #[serde(rename = "regionWidth")]
            region_width: usize,
            #[serde(rename = "regionHeight")]
            region_height: usize,
            #[serde(rename = "regionCounts")]
            region_counts: Vec<usize>,
            #[serde(rename = "regionArea")]
            region_area: usize,
            #[serde(rename = "presentArea")]
            present_area: usize,
            #[serde(rename = "totalPresents")]
            total_presents: usize,
        },
        #[serde(rename = "areaCheck")]
        AreaCheck {
            #[serde(rename = "regionIndex")]
            region_index: usize,
            #[serde(rename = "presentArea")]
            present_area: usize,
            #[serde(rename = "regionArea")]
            region_area: usize,
        },
        #[serde(rename = "slotCheck")]
        SlotCheck {
            #[serde(rename = "regionIndex")]
            region_index: usize,
            #[serde(rename = "numSlots")]
            num_slots: usize,
            #[serde(rename = "totalPresents")]
            total_presents: usize,
        },
        #[serde(rename = "verdict")]
        Verdict {
            #[serde(rename = "regionIndex")]
            region_index: usize,
            verdict: String,
            #[serde(rename = "acceptedCount")]
            accepted_count: usize,
            #[serde(rename = "rejectedCount")]
            rejected_count: usize,
            #[serde(rename = "undeterminedCount")]
            undetermined_count: usize,
        },
        #[serde(rename = "batchUpdate")]
        BatchUpdate {
            #[serde(rename = "regionIndex")]
            region_index: usize,
            #[serde(rename = "acceptedCount")]
            accepted_count: usize,
            #[serde(rename = "rejectedCount")]
            rejected_count: usize,
            #[serde(rename = "undeterminedCount")]
            undetermined_count: usize,
            message: String,
        },
        #[serde(rename = "summary")]
        Summary {
            #[serde(rename = "acceptedCount")]
            accepted_count: usize,
            #[serde(rename = "rejectedCount")]
            rejected_count: usize,
            #[serde(rename = "undeterminedCount")]
            undetermined_count: usize,
            message: String,
        },
    }

    #[derive(Serialize)]
    pub struct Output {
        #[serde(rename = "puzzleDay")]
        puzzle_day: usize,
        #[serde(rename = "puzzleName")]
        puzzle_name: String,
        part: usize,
        shapes: Vec<Shape>,
        #[serde(rename = "totalRegions")]
        total_regions: usize,
        frames: Vec<Frame>,
        #[serde(rename = "finalAnswer")]
        final_answer: usize,
    }

    pub struct Instrumentation {
        output: Output,
        last_batch_index: usize,
        accepted: usize,
        rejected: usize,
        undetermined: usize,
        region_index: usize,
    }

    impl Instrumentation {
        pub fn new(packages: &[super::Package], total_regions: usize) -> Self {
            let shapes: Vec<Shape> = packages
                .iter()
                .enumerate()
                .map(|(id, pkg)| Shape {
                    id,
                    pattern: pkg.pattern.iter().map(|row| row.iter().collect()).collect(),
                    area: pkg.area,
                })
                .collect();

            let mut output = Output {
                puzzle_day: 12,
                puzzle_name: "Christmas Tree Farm".to_string(),
                part: 1,
                shapes,
                total_regions,
                frames: Vec::new(),
                final_answer: 0,
            };

            output.frames.push(Frame::Intro {
                message: format!("Analyzing {} regions...", total_regions),
            });

            Self {
                output,
                last_batch_index: 0,
                accepted: 0,
                rejected: 0,
                undetermined: 0,
                region_index: 0,
            }
        }

        pub fn reset_region_index(&mut self) {
            self.region_index = 0;
        }

        pub fn next_region_index(&mut self) {
            self.region_index += 1;
        }

        fn should_show_detail(&self) -> bool {
            let total = self.output.total_regions;
            self.region_index < 20 || self.region_index >= total.saturating_sub(20) || self.region_index % 10 == 0
        }

        pub fn region_start(&mut self, width: usize, height: usize, counts: &[usize], present_area: usize, total_presents: usize) {
            if !self.should_show_detail() {
                return;
            }

            // Emit batch update if we skipped regions
            if self.region_index > self.last_batch_index + 1 && self.last_batch_index >= 20 {
                self.output.frames.push(Frame::BatchUpdate {
                    region_index: self.region_index - 1,
                    accepted_count: self.accepted,
                    rejected_count: self.rejected,
                    undetermined_count: self.undetermined,
                    message: format!("Processed regions {}-{}...", self.last_batch_index + 1, self.region_index - 1),
                });
            }

            self.output.frames.push(Frame::RegionStart {
                region_index: self.region_index,
                region_width: width,
                region_height: height,
                region_counts: counts.to_vec(),
                region_area: width * height,
                present_area,
                total_presents,
            });
        }

        pub fn area_check(&mut self, present_area: usize, region_area: usize) {
            if !self.should_show_detail() {
                return;
            }
            self.output.frames.push(Frame::AreaCheck {
                region_index: self.region_index,
                present_area,
                region_area,
            });
        }

        pub fn slot_check(&mut self, num_slots: usize, total_presents: usize) {
            if !self.should_show_detail() {
                return;
            }
            self.output.frames.push(Frame::SlotCheck {
                region_index: self.region_index,
                num_slots,
                total_presents,
            });
        }

        pub fn verdict(
            &mut self,
            verdict: &str,
            _present_area: usize,
            _region_area: usize,
            _num_slots: usize,
            _total_presents: usize,
            accepted: usize,
            rejected: usize,
        ) {
            let total = self.output.total_regions;
            let undetermined = (self.region_index + 1).saturating_sub(accepted + rejected);

            self.accepted = accepted;
            self.rejected = rejected;
            self.undetermined = undetermined;

            if self.should_show_detail() {
                self.output.frames.push(Frame::Verdict {
                    region_index: self.region_index,
                    verdict: verdict.to_string(),
                    accepted_count: accepted,
                    rejected_count: rejected,
                    undetermined_count: undetermined,
                });
                self.last_batch_index = self.region_index;
            }

            // Emit final batch update before last 20 regions
            if self.region_index == total.saturating_sub(21) && self.region_index > 20 {
                self.output.frames.push(Frame::BatchUpdate {
                    region_index: self.region_index,
                    accepted_count: accepted,
                    rejected_count: rejected,
                    undetermined_count: undetermined,
                    message: format!("Processed regions {}-{}...", self.last_batch_index + 1, self.region_index),
                });
                self.last_batch_index = self.region_index;
            }
        }

        pub fn finalize_and_print(&mut self, accepted: usize) {
            self.output.frames.push(Frame::Summary {
                accepted_count: self.accepted,
                rejected_count: self.rejected,
                undetermined_count: self.undetermined,
                message: "Analysis complete!".to_string(),
            });
            self.output.final_answer = accepted;

            println!("{}", serde_json::to_string(&self.output).unwrap());
        }
    }
}
