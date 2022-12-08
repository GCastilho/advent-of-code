use std::fs;
use matrix::{prelude::Compressed, Matrix};

// const FILENAME: &str = "example.txt";
const FILENAME: &str = "input.txt";

fn is_tallest(matrix: &Compressed<u8>, coordinates: (usize, usize)) -> bool {
    let (row, column) = coordinates;
    let value = matrix.get(coordinates);

    let is_bigger_than_stored = |col: usize| {
        let stored = matrix.get((row, col));
        value > stored
    };

    let from_left = (0..column)
        .all(is_bigger_than_stored);

    let from_right = (column+1..matrix.columns)
        .all(is_bigger_than_stored);

    // O parâmetro agora é a row
    let is_bigger_than_stored = |row: usize| {
        let stored = matrix.get((row, column));
        value > stored
    };

    let from_top = (0..row)
        .all(is_bigger_than_stored);

    let from_bottom = (row+1..matrix.rows)
        .all(is_bigger_than_stored);

    from_left || from_right || from_top || from_bottom
}

fn main() {
    let input = fs::read_to_string(FILENAME).unwrap();

    let rows = input.matches('\n').count();
    let columns = input.lines().next().unwrap().len();

    let mut matrix = Compressed::zero((rows, columns));

    for (r, line) in input.lines().enumerate() {
        let row = line
            .chars()
            .map(|c| c.to_digit(10).unwrap());
        for (c, digit) in row.enumerate() {
            matrix.set((r, c), digit as u8);
        }
    }

    let mut visible = 0;

    for row in 0..rows {
        for column in 0..columns {
            let coordinates = (row, column);
            if is_tallest(&matrix, coordinates) {
                visible += 1;
            }
        }
    }

    println!("visible: {}", visible);
}
