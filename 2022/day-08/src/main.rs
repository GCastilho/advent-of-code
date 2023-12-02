use std::fs;
use matrix::{prelude::Compressed, Matrix};

const FILENAME: &str = "example.txt";
// const FILENAME: &str = "input.txt";

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

fn build_scenic_score_matrix(matrix: &Compressed<u8>) -> Compressed<usize> {
    let mut scenic_score_matrix = Compressed::zero((matrix.rows, matrix.columns));

    for (row, column, value) in matrix.iter() {
        println!("iter {}, {}, {}", row, column, value);
        #[derive(Debug)]
        enum Axis {
            Column,
            Row,
        }
        #[derive(Debug)]
        enum Direction {
            Forward,
            Backwards,
        }
        // o range dos q vai pro zero tá contanto o len errado, pq tá indo de fora pra dentro
        let ranges = [
            (0..column, Axis::Column, Direction::Backwards),
            (column+1..matrix.columns, Axis::Column, Direction::Forward),
            (0..row, Axis::Row, Direction::Backwards),
            (row+1..matrix.rows, Axis::Row, Direction::Forward),
        ];
        println!("ranges: {:?}", ranges);
        let scores = ranges.map(|(range, axis, direction)| {
            let len = range.len();
            range.enumerate().find(|(_, n)| {
                let position = match axis {
                    Axis::Column => (row, *n),
                    Axis::Row => (*n, column),
                };
                let stored = matrix.get(position);
                println!(
                    "n: {}; axis: {:?}; value: {}; stored: {}; result: {}",
                    n, axis, *value, stored, stored >= *value
                );
                stored >= *value
            })
            .map(|(idx, _)| {
                match direction {
                    Direction::Forward => idx + 1,
                    Direction::Backwards => len - idx,
                }
            })
            .unwrap_or(if len > 0 { len } else { 0 })
        });
        let score = scores
            .into_iter()
            .reduce(|acc, cur| acc * cur)
            .unwrap();
        println!("scores: {:?}\nscore: {}", scores, score);
        println!();
        scenic_score_matrix.set((row, column), score)
    }

    scenic_score_matrix
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

    let scenic_score_matrix = build_scenic_score_matrix(&matrix);

    println!("{:?}", scenic_score_matrix);

    let highest_scenic_score = scenic_score_matrix
        .iter()
        .map(|v|  v.2)
        .max()
        .unwrap();
    println!("highest scenic score: {}", highest_scenic_score);
}
