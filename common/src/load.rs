use std::{
    env,
    fs::read_to_string,
    io,
    path::Path,
};

/// Loads the entire contents of the file specified as the first command-line argument into a string.
/// 
/// # Panics
/// Panics if the file cannot be read or the argument is missing.
/// 
/// # Example
/// ```no_run
/// let contents = common::load::string();
/// println!("{}", contents);
/// ```
pub fn string() -> String {
    let path = get_path();

    // Load data
    read_to_string(&path)
        .unwrap_or_else(|_| panic!("Could not read the file \"{}\"", path))
}

/// Loads lines of data from the file specified as the first command-line argument into a vector of strings.
/// 
/// # Panics
/// Panics if the file cannot be read or the argument is missing.
/// 
/// # Example
/// ```no_run
/// let lines = common::load::lines();
/// for line in lines {
///     println!("{}", line);
/// }
/// ```
pub fn lines() -> Vec<String> {
    let path = get_path();

    // Load data
    read_lines(&path)
        .unwrap_or_else(|_| panic!("Could not read the file \"{}\"", path))
}

/// Reads a file line by line into a vector of strings.
/// 
/// # Arguments
/// * `filename` - Path to the file to read.
/// 
/// # Returns
/// * `Ok(Vec<String>)` with each line as a string, or an error if the file cannot be read.
fn read_lines(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    read_to_string(filename)
        .map(|input|
            input.lines().map(str::to_string).collect()
        )
}

/// Loads a file specified as the first command-line argument and splits its contents by commas into a vector of strings.
/// 
/// # Panics
/// Panics if the file cannot be read or the argument is missing.
/// 
/// # Example
/// ```no_run
/// let values = common::load::comma_separated_values();
/// for value in values {
///     println!("{}", value);
/// }
/// ```
pub fn comma_separated_values() -> Vec<String> {
    let path = get_path();

    // Load data
    read_comma_separated_values(&path)
        .unwrap_or_else(|_| panic!("Could not read the file \"{}\"", path))
}

/// Reads an entire file and splits it by ',' into a vector of strings.
/// 
/// # Arguments
/// * `filename` - Path to the file to read.
/// 
/// # Returns
/// * `Ok(Vec<String>)` with each value as a string, or an error if the file cannot be read.
fn read_comma_separated_values(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    read_to_string(filename)
        .map(|input|
            input.split(',').map(|s| s.trim().to_string()).collect()
        )
}

/// Loads a file specified as the first command-line argument into a 2D array of characters.
/// Each line becomes a row, and each character becomes an element.
/// 
/// # Panics
/// Panics if the file cannot be read or the argument is missing.
/// 
/// # Example
/// ```no_run
/// let map = common::load::map();
/// for row in map {
///     println!("{:?}", row);
/// }
/// ```
pub fn map() -> Vec<Vec<char>> {
    let path = get_path();

    // Load data
    read_map(&path)
        .unwrap_or_else(|_| panic!("Could not read the file \"{}\"", path))
}

/// Reads an entire file into a 2D array of characters.
/// Each line becomes a row, and each character becomes an element.
/// 
/// # Arguments
/// * `filename` - Path to the file to read.
/// 
/// # Returns
/// * `Ok(Vec<Vec<char>>)` with each row as a vector of characters, or an error if the file cannot be read.
fn read_map(filename: impl AsRef<Path>) -> io::Result<Vec<Vec<char>>> {
    read_to_string(filename)
        .map(|input|
            input
                .lines()
                .map(|line| line.chars().collect())
                .collect()
        )
}

/// Loads a file specified as the first command-line argument into a 2D array of numbers.
/// Each digit in the file is parsed as an i32. Non-digit characters are ignored.
/// 
/// # Panics
/// Panics if the file cannot be read or the argument is missing.
/// 
/// # Example
/// ```no_run
/// let numbers = common::load::numbers_map();
/// for row in numbers {
///     println!("{:?}", row);
/// }
/// ```
pub fn numbers_map() -> Vec<Vec<i32>> {
    let path = get_path();

    // Load data
    read_numbers_map(&path)
        .unwrap_or_else(|_| panic!("Could not read the file \"{}\"", path))
}

/// Reads an entire file into a 2D array of numbers (i32).
/// Each digit in the file is parsed as an i32. Non-digit characters are ignored.
/// 
/// # Arguments
/// * `filename` - Path to the file to read.
/// 
/// # Returns
/// * `Ok(Vec<Vec<i32>>)` with each row as a vector of numbers, or an error if the file cannot be read.
fn read_numbers_map(filename: impl AsRef<Path>) -> io::Result<Vec<Vec<i32>>> {
    read_to_string(filename)
        .map(|input|
            input
                .lines()
                .map(|line|
                    line
                        .chars()
                        .filter_map(|c|
                            c
                                .to_digit(10)
                                .map(|u| u as i32)
                        )
                    .collect()
                )
                .collect()
        )
}

/// Gets the path from the command line arguments
fn get_path() -> String {
    env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("Please provide the input file path as the first argument."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines() {
        let input = "a\nb\nc";
        let file_path = std::env::temp_dir().join("test_read_lines.txt");
        std::fs::write(&file_path, input).unwrap();
        let lines = read_lines(&file_path).unwrap();
        assert_eq!(lines, vec!["a", "b", "c"]);
        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_read_comma_separated_values() {
        let input = "foo, bar, baz";
        let file_path = std::env::temp_dir().join("test_csv.txt");
        std::fs::write(&file_path, input).unwrap();
        let values = read_comma_separated_values(&file_path).unwrap();
        assert_eq!(values, vec!["foo", "bar", "baz"]);
        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_read_map() {
        let input = "ab\ncd";
        let file_path = std::env::temp_dir().join("test_map.txt");
        std::fs::write(&file_path, input).unwrap();
        let map = read_map(&file_path).unwrap();
        assert_eq!(map, vec![vec!['a', 'b'], vec!['c', 'd']]);
        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_read_numbers_map() {
        let input = "12\n34";
        let file_path = std::env::temp_dir().join("test_numbers_map.txt");
        std::fs::write(&file_path, input).unwrap();
        let map = read_numbers_map(&file_path).unwrap();
        assert_eq!(map, vec![vec![1, 2], vec![3, 4]]);
        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_read_lines_file_not_found() {
        let path = std::path::PathBuf::from("/nonexistent/file.txt");
        let result = read_lines(&path);
        assert!(result.is_ok() == false);
    }
}
