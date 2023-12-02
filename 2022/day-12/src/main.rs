use std::fs;

use day_12::*;

const INPUT: &str = "example.txt";

fn main() {
    let input = fs::read_to_string(INPUT).unwrap();

    let matrix = get_matrix(&input);
    let start = find_char(&matrix, 'S');
    let end = find_char(&matrix, 'E');

    println!("matrix {:?}\nS: {:?}, E {:?}", matrix, start, end);
}
