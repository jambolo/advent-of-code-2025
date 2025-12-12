pub fn map(map: &[Vec<char>]) {
    let width = map[0].len();
    println!("+{}+", "-".repeat(width));
    for row in map {
        println!("|{}|", row.iter().collect::<String>());
    }
    println!("+{}+", "-".repeat(width));
}
