pub fn map(map: &Vec<Vec<char>>) {
    let width = map[0].len();
    print_horizontal_border(width);
    for row in map {
        let line: String = row.iter().collect();
        println!("|{}|", line);
    }
    print_horizontal_border(width);
}

fn print_horizontal_border(width: usize) {
    print!("+");
    for _ in 0..(width) {
        print!("-");
    }
    println!("+");
    }