// Advent of Code 2025, Day 9

use common::load;
//use common::print;

fn main() {
    println!("Day 9, part {}", if cfg!(feature = "part2") { "2" } else { "1" });

    // Load the locations of the corners.
    let lines = load::lines();
    let corners: Vec<(usize, usize)> = lines.iter().map(|line| {
        let mut parts = line.split(',');
        let x: usize = parts.next().unwrap().parse().unwrap();
        let y: usize = parts.next().unwrap().parse().unwrap();
        (x, y)
    }).collect();

//    let mut map = create_map(&corners);
//    if cfg!(feature = "part2") {
//        for (v0, v1) in consecutive_pairs(&corners) {
//            map[v0.1][v0.0] = '#';
//            draw_side(&mut map, &v0, &v1);
//        }
//
//    } else {
//        for c in corners.iter() {
//            let (x, y) = c;
//            map[*y][*x] = '#';
//        }
//    }
//
//    print::map(&map);

    let mut rectangles: Vec<((usize, usize), i64)> = Vec::new();

    // Create a list of all possible rectangles and their areas.
    for i in 0..corners.len() - 1 {
        let c0 = corners[i];
        let (x0, y0) = c0;
        for j in i + 1..corners.len() {
            let c1 = corners[j];
            let (x1, y1) = c1;
            let width = (x1 as i64 - x0 as i64).abs() + 1;
            let height = (y1 as i64 - y0 as i64).abs() + 1;
            let area = width * height;
            if cfg!(feature = "part2") {
                if fully_contained(&corners, &(c0, c1)) {
                    rectangles.push(((i, j), area));
                }
            } else {
                rectangles.push(((i, j), area));
            }
        }
    }

    // Sort rectangles by area.
    rectangles.sort_by_key(|&(_, area)| area);
//    println!("Rectangles found: {:?}", rectangles);

//    let largest_rect = rectangles.last().unwrap();
//    let rect_c0 = corners[largest_rect.0 .0];
//    let rect_c1 = corners[largest_rect.0 .1];
//    draw_rect(&mut map, &(rect_c0, rect_c1));
//    print::map(&map);

    // Output the area of the largest rectangle.
    let largest_area = rectangles.last().unwrap().1;
    println!("Largest area: {}", largest_area);
}

//fn draw_rect(map: &mut Vec<Vec<char>>, rect: &((usize, usize), (usize, usize))) {
//    let c0 = rect.0;
//    let c1 = rect.1;
//    let x0 = c0.0.min(c1.0);
//    let x1 = c0.0.max(c1.0);
//    let y0 = c0.1.min(c1.1);
//    let y1 = c0.1.max(c1.1);
//    for y in y0..=y1 {
//        for x in x0..=x1 {
//            map[y][x] = 'O';
//        }
//    }
//}
//
//fn create_map(corners: &Vec<(usize, usize)>) -> Vec<Vec<char>> {
//    let (max_x, max_y) = corners.iter()
//        .fold((0, 0), |(mx, my), &(x, y)| (
//            mx.max(x),
//            my.max(y)
//        ));
//    vec![vec!['.'; max_x + 1]; max_y + 1]
//}
//
//fn draw_side(map: &mut Vec<Vec<char>>, v0: &(usize, usize), v1: &(usize, usize)) {
//    let (x0, y0) = v0;
//    let (x1, y1) = v1;
//    let dx: isize = if x1 > x0 { 1 } else if x1 < x0 { -1 } else { 0 };
//    let dy: isize = if y1 > y0 { 1 } else if y1 < y0 { -1 } else { 0 };
//    let mut x = *x0 as isize + dx;
//    let mut y = *y0 as isize + dy;
//    while (x as usize != *x1) || (y as usize != *y1) {
//        map[y as usize][x as usize] = 'X';
//        if x as usize != *x1 {
//            x += dx;
//        }
//        if y as usize != *y1 {
//            y += dy;
//        }
//    }
//}

fn crosses(rect: &((usize, usize), (usize, usize)), v0: &(usize, usize), v1: &(usize, usize)) -> bool {
    let x0 = v0.0.min(v1.0);    // 7
    let x1 = v0.0.max(v1.0);    // 11
    let y0 = v0.1.min(v1.1);    // 1
    let y1 = v0.1.max(v1.1);    //1

    let c0 = rect.0;   // 9, 5
    let c1 = rect.1;   // 2, 3
    let rx0 = c0.0.min(c1.0);   // 2
    let rx1 = c0.0.max(c1.0);   // 9
    let ry0 = c0.1.min(c1.1);   // 3
    let ry1 = c0.1.max(c1.1);   

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

fn fully_contained(corners: &Vec<(usize, usize)>, rect: &((usize, usize), (usize, usize))) -> bool {
//    println!("Checking containment for rect: {:?}", rect);
    for (v0, v1) in consecutive_pairs(corners) {
        if crosses(rect, v0, v1) {
//            println!("Crossed by: {:?}, {:?}", v0, v1);
            return false;
        }
    }
    true
}

// Re
fn consecutive_pairs<T>(slice: &[T]) -> impl Iterator<Item = (&T, &T)> {
    slice.iter()
        .zip(slice.iter()
            .cycle()
            .skip(1))
        .take(slice.len())
}