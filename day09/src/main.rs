// Advent of Code 2025, Day 9

use common::load;

#[cfg(feature = "instrumented")]
use instrumentation::Instrumentation;

type Edge = ((usize, usize), (usize, usize));
type Rect = ((usize, usize), (usize, usize));

fn main() {
    #[cfg(not(feature = "instrumented"))]
    println!("Day 9, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    #[cfg(feature = "instrumented")]
    let mut inst = Instrumentation::new();

    // Load the locations of the corners.
    let corners: Vec<(usize, usize)> = load::lines()
        .iter()
        .map(|line| {
            let (x, y) = line
                .split_once(',')
                .expect("Failed to split line on ',' for corner coordinates");
            (
                x.parse().expect("Failed to parse x coordinate as usize"),
                y.parse().expect("Failed to parse y coordinate as usize"),
            )
        })
        .collect();

    // List of edges of the region with normalized vertex order.
    let edges: Vec<_> = consecutive_pairs(&corners)
        .map(|(v0, v1)| if v0.0 < v1.0 || v0.1 < v1.1 { (*v0, *v1) } else { (*v1, *v0) })
        .collect();

    // List of horizontal edges with normalized vertex order.
    let horizontal_edges: Vec<_> = edges.iter().filter(|(v0, v1)| v0.1 == v1.1).copied().collect();

    #[cfg(feature = "instrumented")]
    inst.emit_initial(&corners, &edges);

    // Create a list of all possible rectangles and their areas.
    let mut rectangles = Vec::new();
    for i in 0..corners.len() - 1 {
        let (x0, y0) = corners[i];
        for j in i + 1..corners.len() {
            let (x1, y1) = corners[j];
            let area = ((x1 as i64 - x0 as i64).abs() + 1) * ((y1 as i64 - y0 as i64).abs() + 1);
            if cfg!(feature = "part2") {
                let is_valid = fully_contained(&edges, &horizontal_edges, &(corners[i], corners[j]));
                #[cfg(feature = "instrumented")]
                inst.record_candidate(&corners, &edges, i, j, area, is_valid);
                if is_valid {
                    rectangles.push(((i, j), area));
                }
            } else {
                rectangles.push(((i, j), area));
            }
        }
    }

    // Sort rectangles by area.
    rectangles.sort_by_key(|&(_, area)| area);

    let largest = rectangles.last().expect("No rectangles found");

    #[cfg(feature = "instrumented")]
    {
        inst.emit_final(&corners, &edges, largest.0.0, largest.0.1, largest.1);
        inst.print();
    }

    // Output the area of the largest rectangle.
    #[cfg(not(feature = "instrumented"))]
    println!("Largest area: {}", largest.1);
}

fn crosses(rect: &Rect, v0: &(usize, usize), v1: &(usize, usize)) -> bool {
    let (x0, y0) = *v0;
    let (x1, y1) = *v1;
    let ((cx0, cy0), (cx1, cy1)) = *rect;
    let (rx0, rx1) = (cx0.min(cx1), cx0.max(cx1));
    let (ry0, ry1) = (cy0.min(cy1), cy0.max(cy1));

    if x0 <= rx0 && rx0 <= x1 && ry0 < y0 && y0 < ry1 {
        return true;
    }
    if x0 <= rx1 && rx1 <= x1 && ry0 < y0 && y0 < ry1 {
        return true;
    }
    if y0 <= ry0 && ry0 <= y1 && rx0 < x0 && x0 < rx1 {
        return true;
    }
    if y0 <= ry1 && ry1 <= y1 && rx0 < x0 && x0 < rx1 {
        return true;
    }
    false
}

fn fully_contained(edges: &[Edge], horizontal_edges: &[Edge], rect: &Rect) -> bool {
    // Check that no region edges cross any of the rect edges.
    if edges.iter().any(|(v0, v1)| crosses(rect, v0, v1)) {
        return false;
    }

    // A single row or column is always contained.

    // Check that an interior point of the rectangle is inside the region.
    let test_x = rect.0.0.min(rect.1.0) as f64 + 0.5;
    let test_y = rect.0.1.min(rect.1.1) as f64 + 0.5;

    // Cast a vertical ray downward and count crossings with horizontal edges
    let crossings = horizontal_edges
        .iter()
        .filter(|(v0, v1)| {
            let edge_y = v0.1 as f64;
            let v0_x = v0.0 as f64;
            let v1_x = v1.0 as f64;
            // Check if the ray crosses this horizontal edge
            edge_y > test_y && v0_x < test_x && test_x < v1_x
        })
        .count();

    crossings % 2 == 1
}

fn consecutive_pairs<T>(slice: &[T]) -> impl Iterator<Item = (&T, &T)> {
    slice.iter().zip(slice.iter().cycle().skip(1)).take(slice.len())
}

#[cfg(feature = "instrumented")]
mod instrumentation {
    use serde::Serialize;

    #[derive(Serialize, Clone)]
    pub struct Corner {
        x: usize,
        y: usize,
        index: usize,
    }

    #[derive(Serialize, Clone)]
    pub struct Edge {
        from: Corner,
        to: Corner,
    }

    #[derive(Serialize, Clone)]
    pub struct Rectangle {
        corner1: Corner,
        corner2: Corner,
        area: i64,
        #[serde(rename = "isValid")]
        is_valid: bool,
        #[serde(rename = "isNewBest")]
        is_new_best: bool,
    }

    #[derive(Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Frame {
        frame_type: String,
        corners: Vec<Corner>,
        edges: Vec<Edge>,
        #[serde(skip_serializing_if = "Option::is_none")]
        candidate: Option<Rectangle>,
        #[serde(skip_serializing_if = "Option::is_none")]
        best_rectangle: Option<Rectangle>,
        pairs_tested: usize,
        total_pairs: usize,
        valid_count: usize,
        best_area: i64,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Output {
        puzzle_day: u8,
        puzzle_name: String,
        part: u8,
        frames: Vec<Frame>,
        final_answer: i64,
    }

    pub struct Instrumentation {
        frames: Vec<Frame>,
        pairs_tested: usize,
        total_pairs: usize,
        valid_count: usize,
        best_area: i64,
        best_i: usize,
        best_j: usize,
        final_answer: i64,
    }

    impl Instrumentation {
        pub fn new() -> Self {
            Self {
                frames: Vec::new(),
                pairs_tested: 0,
                total_pairs: 0,
                valid_count: 0,
                best_area: 0,
                best_i: 0,
                best_j: 0,
                final_answer: 0,
            }
        }

        fn make_corners(corners: &[(usize, usize)]) -> Vec<Corner> {
            corners
                .iter()
                .enumerate()
                .map(|(index, &(x, y))| Corner { x, y, index })
                .collect()
        }

        fn make_edges(corners: &[(usize, usize)], edges: &[super::Edge]) -> Vec<Edge> {
            edges
                .iter()
                .map(|(v0, v1)| {
                    let from_idx = corners.iter().position(|c| c == v0).unwrap_or(0);
                    let to_idx = corners.iter().position(|c| c == v1).unwrap_or(0);
                    Edge {
                        from: Corner {
                            x: v0.0,
                            y: v0.1,
                            index: from_idx,
                        },
                        to: Corner {
                            x: v1.0,
                            y: v1.1,
                            index: to_idx,
                        },
                    }
                })
                .collect()
        }

        fn make_best_rectangle(&self, corners: &[(usize, usize)]) -> Option<Rectangle> {
            if self.best_area == 0 {
                None
            } else {
                Some(Rectangle {
                    corner1: Corner {
                        x: corners[self.best_i].0,
                        y: corners[self.best_i].1,
                        index: self.best_i,
                    },
                    corner2: Corner {
                        x: corners[self.best_j].0,
                        y: corners[self.best_j].1,
                        index: self.best_j,
                    },
                    area: self.best_area,
                    is_valid: true,
                    is_new_best: false,
                })
            }
        }

        pub fn emit_initial(&mut self, corners: &[(usize, usize)], edges: &[super::Edge]) {
            let n = corners.len();
            self.total_pairs = n * (n - 1) / 2;

            self.frames.push(Frame {
                frame_type: "initial".to_string(),
                corners: Self::make_corners(corners),
                edges: Self::make_edges(corners, edges),
                candidate: None,
                best_rectangle: None,
                pairs_tested: 0,
                total_pairs: self.total_pairs,
                valid_count: 0,
                best_area: 0,
            });
        }

        pub fn record_candidate(
            &mut self,
            corners: &[(usize, usize)],
            edges: &[super::Edge],
            i: usize,
            j: usize,
            area: i64,
            is_valid: bool,
        ) {
            self.pairs_tested += 1;
            if is_valid {
                self.valid_count += 1;
            }

            let is_new_best = is_valid && area > self.best_area;
            if is_new_best {
                self.best_area = area;
                self.best_i = i;
                self.best_j = j;
            }

            let should_emit = self.should_emit_frame(is_valid, is_new_best);
            if !should_emit {
                return;
            }

            let frame_type = if is_new_best { "newBest" } else { "candidate" };

            let candidate = Rectangle {
                corner1: Corner {
                    x: corners[i].0,
                    y: corners[i].1,
                    index: i,
                },
                corner2: Corner {
                    x: corners[j].0,
                    y: corners[j].1,
                    index: j,
                },
                area,
                is_valid,
                is_new_best,
            };

            self.frames.push(Frame {
                frame_type: frame_type.to_string(),
                corners: Self::make_corners(corners),
                edges: Self::make_edges(corners, edges),
                candidate: Some(candidate),
                best_rectangle: self.make_best_rectangle(corners),
                pairs_tested: self.pairs_tested,
                total_pairs: self.total_pairs,
                valid_count: self.valid_count,
                best_area: self.best_area,
            });
        }

        fn should_emit_frame(&self, is_valid: bool, is_new_best: bool) -> bool {
            // Always emit new best discoveries
            if is_new_best {
                return true;
            }

            let remaining = self.total_pairs - self.pairs_tested;

            // Initial phase: first 50 candidates
            if self.pairs_tested <= 50 {
                return true;
            }

            // Final phase: last 100 candidates
            if remaining < 100 {
                return self.pairs_tested.is_multiple_of(10);
            }

            // Middle phase: sample valid and invalid differently
            if is_valid {
                self.valid_count.is_multiple_of(50)
            } else {
                self.pairs_tested.is_multiple_of(500)
            }
        }

        pub fn emit_final(
            &mut self,
            corners: &[(usize, usize)],
            edges: &[super::Edge],
            best_i: usize,
            best_j: usize,
            best_area: i64,
        ) {
            self.final_answer = best_area;

            let best_rectangle = Rectangle {
                corner1: Corner {
                    x: corners[best_i].0,
                    y: corners[best_i].1,
                    index: best_i,
                },
                corner2: Corner {
                    x: corners[best_j].0,
                    y: corners[best_j].1,
                    index: best_j,
                },
                area: best_area,
                is_valid: true,
                is_new_best: false,
            };

            self.frames.push(Frame {
                frame_type: "final".to_string(),
                corners: Self::make_corners(corners),
                edges: Self::make_edges(corners, edges),
                candidate: None,
                best_rectangle: Some(best_rectangle),
                pairs_tested: self.pairs_tested,
                total_pairs: self.total_pairs,
                valid_count: self.valid_count,
                best_area: self.best_area,
            });
        }

        pub fn print(&self) {
            let output = Output {
                puzzle_day: 9,
                puzzle_name: "Movie Theater".to_string(),
                part: 2,
                frames: self.frames.clone(),
                final_answer: self.final_answer,
            };
            println!("{}", serde_json::to_string(&output).unwrap());
        }
    }
}
