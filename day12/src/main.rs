// Advent of Code 2025, Day 12

use common::load;

#[derive(Debug)]
struct Package {
    area: usize,
}

impl Package {
    fn new(base_shape: [[char; 3]; 3]) -> Self {
        // Count the number of filled cells
        let area = base_shape.iter().flatten().filter(|&&c| c == '#').count();

        Package { area }
    }
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    counts: Vec<usize>,
}

fn main() {
    println!("Day 12, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    let lines = load::lines();
    let (packages, regions) = parse_input(&lines);

    // Count the number of regions that can fit all packages
    let mut rejected: i64 = 0;
    let mut accepted: i64 = 0;

    for region in &regions {
        let region_area = region.width * region.height;
        let total_packages: usize = region.counts.iter().sum();

        // See if the region's has enough space for the areas of all packages.
        // Reject those that don't.
        let mut packages_area = 0;
        for (i, &c) in region.counts.iter().enumerate() {
            packages_area += c * packages[i].area;
        }
        if packages_area > region_area {
            rejected += 1;
            continue;
        }

        // Packages are at most 3x3, so check if the region has enough 3x3 slots for all packages.
        // Accept all that do.
        let num_slots = region.width / 3 * (region.height / 3);
        if total_packages <= num_slots {
            accepted += 1;
        }
    }
    println!("Rejected: {}", rejected);
    println!("Accepted: {}", accepted);
    println!("Undetermined: {}", regions.len() as i64 - accepted - rejected);
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
