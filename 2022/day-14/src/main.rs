use std::fs;
use matrix::{prelude::Compressed, Matrix, Element, Size};

const INPUT: &str = "example.txt";
const SAND_HOLE: (usize, usize) = (0, 500);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Stuff {
    Void,
    Rock,
    Sand,
}

impl Element for Stuff {
    fn zero() -> Self {
        Self::Void
    }

    fn is_zero(&self) -> bool {
        *self == Self::Void
    }
}

fn build_matrix(input: &Vec<Vec<(usize, usize)>>) -> Compressed<Stuff> {
    let (xs, ys) = input.iter()
        .flatten()
        .map(|v| *v)
        .unzip::<_, _, Vec<_>, Vec<_>>();

    let rows = *ys.iter().max().unwrap() + 1;
    let columns = *xs.iter().max().unwrap() + 1;

    Compressed::zero((rows, columns))
}

fn main() {
    let input = fs::read_to_string(INPUT).unwrap();

    let input = input.lines()
        .map(|line| {
            line.split("->")
                .map(|tuple| {
                    let mut tuple = tuple.trim()
                        .split_terminator(",")
                        .map(|v| v.parse::<usize>().unwrap());
                    let row = tuple.next().unwrap();
                    let column = tuple.next().unwrap();
                    (row, column)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("input: {:?}", input);

    let mut matrix = build_matrix(&input);

    for (col, row) in input.iter().flatten() {
        println!("{} {}", row, col);
        matrix.set((*row, *col), Stuff::Rock);
    };

    println!("matrix: {:?}", matrix);

    let mut sand_count = 0;
    let mut sand = SAND_HOLE;
    // Game loop
    loop {
        #[derive(Debug)]
        enum Move {
            Rest,
            Down,
            Left,
            Right,
        }

        let (row, col) = sand;
        println!("sand: {:?}", sand);
        if row == matrix.rows -1 { break; }

        let movement = if matrix.get((row + 1, col)) == Stuff::Void {
            Move::Down
        } else if matrix.get((row + 1, col - 1)) == Stuff::Void {
            Move::Left
        } else if matrix.get((row + 1, col + 1)) == Stuff::Void {
            Move::Right
        } else {
            Move::Rest
        };
        println!("movement {:?}", movement);

        sand = match movement {
            Move::Down => (row + 1, col),
            Move::Left => (row + 1, col - 1),
            Move::Right => (row + 1, col + 1),
            Move::Rest => {
                if sand == SAND_HOLE { break; }
                matrix.set(sand, Stuff::Sand);
                sand_count += 1;
                SAND_HOLE
            },
        };
    }

    println!("sand count: {}", sand_count);
}
