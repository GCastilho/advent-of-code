use matrix::{prelude::Compressed, Matrix};

pub fn get_matrix(input: &str) -> Compressed<u8> {
    let rows = input.matches('\n').count();
    let columns = input.lines().next().unwrap().len();

    let mut matrix = Compressed::zero((rows, columns));

    for (r, line) in input.lines().enumerate() {
        let row = line.chars();
        for (c, char) in row.enumerate() {
            matrix.set((r, c), char as u8);
        }
    }

    matrix
}

pub fn find_char(matrix: &Compressed<u8>, char: char) -> (usize, usize) {
    matrix.iter()
    .find(|(_, _, &c)| c as char == char)
    .map(|(x, y, _)| (x, y))
    .unwrap()
}
