use std::fs;

use matrix::{prelude::Compressed, Element, Matrix, Size};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let engine_schematic = EngineSchematic::from(&input);
    let sum = engine_schematic
        .symbols()
        .into_iter()
        .flat_map(|symbol| engine_schematic.get_adjacent_numbers(&symbol))
        .sum::<u32>();

    println!("Part one, sum is: {sum}");

    let gear_ratios = engine_schematic
        .symbols()
        .into_iter()
        .filter(|symbol| symbol.is_gear)
        .map(|symbol| engine_schematic.get_adjacent_numbers(&symbol))
        .filter(|nums| nums.len() == 2)
        .map(|nums| nums[0] * nums[1])
        .sum::<u32>();
    println!("Part two, gear ratios is: {gear_ratios}");
}

type Coordinates = (usize, usize);

#[derive(Debug)]
struct EngineSchematic(Compressed<Token>);

impl EngineSchematic {
    fn get_input_size(input: &str) -> Coordinates {
        let input = input.trim();
        let x = input.lines().next().unwrap().chars().count() + 1;
        let y = input.lines().count();
        (x, y)
    }

    fn new(size: (usize, usize)) -> Self {
        Self(Compressed::zero(size))
    }

    fn insert_line(&mut self, line_num: usize, line: Vec<Token>) {
        let mut idx = 0;
        for token in line.into_iter() {
            match token {
                Token::None => (),
                Token::Symbol { .. } => self.0.set((line_num, idx), token),
                Token::Number { len, .. } => {
                    for _ in 0..len {
                        self.0.set((line_num, idx), token);
                        idx += 1;
                    }
                    idx -= 1;
                }
            };
            idx += 1;
        }
    }

    fn from(input: &str) -> Self {
        let size = Self::get_input_size(input);
        let mut parser = Parser::default();
        let mut matrix = Self::new(size);
        input
            .lines()
            .map(|line| parser.parse_line(line))
            .enumerate()
            .for_each(|(num, line)| matrix.insert_line(num, line));
        matrix
    }

    fn symbols(&self) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        for x in 0..self.0.rows() {
            for y in 0..self.0.columns() {
                let token = self.0.get((x, y));
                if let Token::Symbol(is_gear) = token {
                    symbols.push(Symbol {
                        coordinates: (x, y),
                        is_gear,
                    })
                }
            }
        }
        symbols
    }

    fn get_adjacent_numbers(&self, symbol: &Symbol) -> Vec<u32> {
        let mut numbers = Vec::new();
        let mut ids = Vec::new();

        let Symbol {
            coordinates: (x, y),
            ..
        } = symbol;

        let x_start = x.checked_sub(1).unwrap_or_default();
        let x_end = if *x == self.0.columns() {
            self.0.columns()
        } else {
            x + 1
        };
        let y_start = y.checked_sub(1).unwrap_or_default();
        let y_end = if *x == self.0.columns() {
            self.0.rows()
        } else {
            y + 1
        };
        for x in x_start..=x_end {
            for y in y_start..=y_end {
                let token = self.0.get((x, y));
                if let Token::Number { id, value, .. } = token {
                    if !ids.contains(&id) {
                        numbers.push(value);
                        ids.push(id);
                    }
                }
            }
        }

        numbers
    }
}

#[derive(Debug, PartialEq)]
struct Symbol {
    coordinates: Coordinates,
    is_gear: bool,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    None,
    Symbol(bool),
    Number { id: usize, value: u32, len: usize },
}

impl Element for Token {
    fn zero() -> Self {
        Token::None
    }
}

#[derive(Debug, Default)]
struct Parser {
    current_id_number: usize,
}

impl Parser {
    fn parse_line(&mut self, line: &str) -> Vec<Token> {
        let mut tokens = Vec::new();

        let mut iter = line.chars().peekable();
        while iter.peek().is_some() {
            if iter.peek().is_some_and(|c| c.is_numeric()) {
                let mut chars = Vec::new();
                while iter.peek().is_some_and(|c| c.is_numeric()) {
                    chars.push(iter.next().unwrap());
                }
                let len = chars.len();
                let value = chars
                    .into_iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
                tokens.push(Token::Number {
                    id: self.current_id_number,
                    value,
                    len,
                });
                self.current_id_number += 1;
            } else {
                let c = iter.next().unwrap();
                if c == '.' {
                    tokens.push(Token::None)
                } else if c == '*' {
                    tokens.push(Token::Symbol(true))
                } else {
                    tokens.push(Token::Symbol(false))
                }
            }
        }

        tokens
    }
}

#[cfg(test)]
mod test {
    const INPUT: &str = r"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    mod part_one {
        use super::INPUT;
        use crate::*;

        #[test]
        fn get_m_size() {
            let m_size = EngineSchematic::get_input_size(INPUT);
            assert_eq!(m_size, (11, 10));
        }

        #[test]
        fn parse_line() {
            let mut parser = Parser::default();
            let line = "617*..63..";
            let output = vec![
                Token::Number {
                    id: 0,
                    value: 617,
                    len: 3,
                },
                Token::Symbol(true),
                Token::None,
                Token::None,
                Token::Number {
                    id: 1,
                    value: 63,
                    len: 2,
                },
                Token::None,
                Token::None,
            ];
            assert_eq!(parser.parse_line(line), output)
        }

        #[test]
        fn insert_line() {
            let size = (1, 10);
            let mut engine_schematic = EngineSchematic::new(size);
            let mut parser = Parser::default();
            let line = parser.parse_line("617*..63..");
            engine_schematic.insert_line(0, line);

            let mut matrix = Compressed::zero(size);
            let t1 = Token::Number {
                id: 0,
                value: 617,
                len: 3,
            };
            matrix.set((0, 0), t1);
            matrix.set((0, 1), t1);
            matrix.set((0, 2), t1);
            matrix.set((0, 3), Token::Symbol(true));
            let t2 = Token::Number {
                id: 1,
                value: 63,
                len: 2,
            };
            matrix.set((0, 6), t2);
            matrix.set((0, 7), t2);

            assert_eq!(matrix, engine_schematic.0);
        }

        #[test]
        fn symbols() {
            let mut matrix = EngineSchematic::new((2, 2));
            matrix.0.set((0, 0), Token::Symbol(false));
            matrix.0.set((1, 1), Token::Symbol(false));
            let symbols = matrix.symbols();
            assert_eq!(
                symbols,
                vec![
                    Symbol {
                        coordinates: (0, 0),
                        is_gear: false
                    },
                    Symbol {
                        coordinates: (1, 1),
                        is_gear: false
                    }
                ]
            );
        }

        #[test]
        fn get_adjacent_numbers() {
            let mut matrix = EngineSchematic::new((3, 3));
            matrix.0.set((1, 1), Token::Symbol(false));
            matrix.0.set(
                (0, 0),
                Token::Number {
                    id: 0,
                    value: 12,
                    len: 2,
                },
            );
            matrix.0.set(
                (1, 0),
                Token::Number {
                    id: 0,
                    value: 12,
                    len: 2,
                },
            );
            matrix.0.set(
                (2, 1),
                Token::Number {
                    id: 1,
                    value: 3,
                    len: 1,
                },
            );
            let mut adjacents = matrix.get_adjacent_numbers(&Symbol {
                coordinates: (1, 1),
                is_gear: false,
            });
            adjacents.sort();
            assert_eq!(adjacents, vec![3, 12])
        }

        #[test]
        fn sum() {
            let matrix = EngineSchematic::from(INPUT);
            let sum = matrix
                .symbols()
                .into_iter()
                .flat_map(|s| matrix.get_adjacent_numbers(&s))
                .sum::<u32>();
            assert_eq!(sum, 4361);
        }
    }

    mod part_two {
        use super::INPUT;
        use crate::EngineSchematic;

        #[test]
        fn get_gear() {
            let matrix = EngineSchematic::from(INPUT);
            let gears = matrix
                .symbols()
                .into_iter()
                .filter(|symbol| symbol.is_gear)
                .map(|symbol| matrix.get_adjacent_numbers(&symbol))
                .filter(|nums| nums.len() == 2)
                .map(|nums| nums[0] * nums[1])
                .sum::<u32>();
            assert_eq!(gears, 467835);
        }
    }
}
