use core::fmt;
use matrix::{
    format::{Compressed, Conventional},
    Element, Matrix, Size,
};
use std::{
    collections::VecDeque,
    fs,
    ops::{Index, IndexMut},
};
use strum::{EnumIter, EnumString, IntoEnumIterator};

fn main() {
    let sketch = Sketch::from("./input.txt");
    let steps_to_farthest_point = sketch.get_steps_to_farthest_point();
    println!(
        "Part one, it takes {steps_to_farthest_point} to get to the farthest point from the starting position"
    );

    println!(
        "Part two, there are {} tiles enclosed by the main loop",
        sketch.get_main_loop().get_tiles_enclosed_by_main_loop()
    );
}

#[derive(Debug, EnumIter)]
enum Side {
    Up,
    Right,
    Down,
    Left,
}

impl Side {
    fn get_pos(&self, (y, x): (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Side::Up => y.checked_sub(1).map(|y| (y, x)),
            Side::Right => Some((y, x + 1)),
            Side::Down => Some((y + 1, x)),
            Side::Left => x.checked_sub(1).map(|x| (y, x)),
        }
    }
}

#[derive(Debug, PartialEq, EnumString, Clone, Copy, strum::Display)]
enum Pipe {
    #[strum(serialize = ".")]
    Ground,
    #[strum(serialize = "S")]
    Start,
    #[strum(serialize = "|", to_string = "│")]
    Vertical,
    #[strum(serialize = "-", to_string = "─")]
    Horizontal,
    #[strum(serialize = "L", to_string = "└")]
    NorthEast,
    #[strum(serialize = "J", to_string = "┘")]
    NorthWest,
    #[strum(serialize = "7", to_string = "┐")]
    SouthWest,
    #[strum(serialize = "F", to_string = "┌")]
    SouthEast,
}

impl Pipe {
    fn get_sides(&self) -> Option<Vec<Side>> {
        match self {
            Pipe::Ground => None,
            Pipe::Start => panic!("Can\'t get sides from S Pipe"),
            Pipe::Vertical => Some(vec![Side::Up, Side::Down]),
            Pipe::Horizontal => Some(vec![Side::Left, Side::Right]),
            Pipe::NorthEast => Some(vec![Side::Up, Side::Right]),
            Pipe::NorthWest => Some(vec![Side::Up, Side::Left]),
            Pipe::SouthWest => Some(vec![Side::Down, Side::Left]),
            Pipe::SouthEast => Some(vec![Side::Down, Side::Right]),
        }
    }

    fn connects_from(&self, side: &Side) -> bool {
        match self {
            Pipe::Vertical => matches!(side, Side::Up | Side::Down),
            Pipe::Horizontal => matches!(side, Side::Left | Side::Right),
            Pipe::NorthEast => matches!(side, Side::Down | Side::Left),
            Pipe::NorthWest => matches!(side, Side::Down | Side::Right),
            Pipe::SouthWest => matches!(side, Side::Up | Side::Right),
            Pipe::SouthEast => matches!(side, Side::Up | Side::Left),
            _ => false,
        }
    }

    fn is_inversion(&self) -> bool {
        matches!(&self, Pipe::Vertical | Pipe::NorthWest | Pipe::NorthEast)
    }
}

impl Element for Pipe {
    fn zero() -> Self {
        Self::Ground
    }
}

#[derive(Debug)]
struct Sketch {
    matrix: Conventional<Pipe>,
    start: Option<(usize, usize)>,
}

impl Sketch {
    fn from(filename: &str) -> Self {
        let file = fs::read_to_string(filename).unwrap();
        let x = file.lines().next().unwrap().chars().count();
        let y = file.lines().count();
        let mut matrix = Conventional::zero((y, x));

        let mut start = None;
        for (y, line) in file.lines().enumerate() {
            for (x, word) in line.char_indices() {
                let pipe = word.to_string().parse().unwrap();
                if matches!(pipe, Pipe::Start) {
                    start = Some((y, x));
                }
                *matrix.index_mut((y, x)) = pipe;
            }
        }

        Self { matrix, start }
    }

    fn get_steps_to_farthest_point(&self) -> usize {
        println!("{self}");

        #[derive(Debug, PartialEq, Clone, Copy)]
        struct Distance(Option<u32>);

        impl Element for Distance {
            fn zero() -> Self {
                Self(None)
            }
        }

        impl fmt::Display for Distance {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let text = match self.0 {
                    None => ".".to_string(),
                    Some(n) => format!("{n}"),
                };
                write!(f, "{}", text)
            }
        }

        let start_pos = self.start.unwrap();

        let mut longest_distance = 0;
        let mut m_distances = Compressed::zero((self.matrix.rows, self.matrix.columns));
        m_distances.set(start_pos, Distance(Some(0)));

        let mut queue = VecDeque::new();

        // Preenche os que conectam com start
        for side in Side::iter() {
            let Some(pos) = side.get_pos(start_pos) else {
                continue;
            };
            let item = self.matrix.index(pos);
            if item.connects_from(&side) {
                queue.push_back(pos);
                m_distances.set(pos, Distance(Some(1)));
            }
        }

        while let Some(pos) = queue.pop_front() {
            let item = self.matrix.index(pos);
            let cur_distance = match m_distances.get(pos) {
                Distance(None) => panic!("Distância ATUAL deveria ser conhecida"),
                Distance(Some(d)) => d + 1,
            };
            // Assumindo que não tem dead ends e que sempre os pipes conectam
            let sides = item.get_sides();
            let sides = sides.expect("Não implementado dead end");
            for side in sides {
                let Some(pos) = side.get_pos(pos) else {
                    continue;
                };
                let should_replace = match m_distances.get(pos) {
                    Distance(None) => true,
                    Distance(Some(d)) => cur_distance < d,
                };
                if should_replace {
                    m_distances.set(pos, Distance(Some(cur_distance)));
                    longest_distance = cur_distance;
                    queue.push_back(pos);
                }
            }
        }

        longest_distance as usize
    }

    fn get_start_pipe(connected_sides: [Side; 2]) -> Pipe {
        match connected_sides {
            [Side::Up, Side::Down] => Pipe::Vertical,
            [Side::Up, Side::Right] => Pipe::NorthEast,
            [Side::Up, Side::Left] => Pipe::NorthWest,

            [Side::Right, Side::Down] => Pipe::SouthEast,
            [Side::Right, Side::Left] => Pipe::Horizontal,
            [Side::Right, Side::Up] => Pipe::NorthEast,

            [Side::Down, Side::Left] => Pipe::SouthWest,
            [Side::Down, Side::Up] => Pipe::Vertical,
            [Side::Down, Side::Right] => Pipe::SouthEast,

            [Side::Left, Side::Up] => Pipe::NorthWest,
            [Side::Left, Side::Right] => Pipe::Horizontal,
            [Side::Left, Side::Down] => Pipe::SouthWest,

            side => panic!("Não tem como conectar Start com essa combinação: {side:?}"),
        }
    }

    fn get_main_loop(&self) -> Sketch {
        let start_pos = self.start.unwrap();
        let mut main_loop_matrix = Conventional::zero((self.matrix.rows, self.matrix.columns));
        *main_loop_matrix.index_mut(start_pos) = Pipe::Start;

        let mut queue = VecDeque::new();
        let mut connected_sides = Vec::with_capacity(2);

        // Preenche os que conectam com start
        for side in Side::iter() {
            let Some(pos) = side.get_pos(start_pos) else {
                continue;
            };
            let item = self.matrix.index(pos);
            if item.connects_from(&side) {
                queue.push_back(pos);
                *main_loop_matrix.index_mut(pos) = *self.matrix.index(pos);
                connected_sides.push(side)
            }
        }

        let start_pipe = Self::get_start_pipe(
            connected_sides
                .try_into()
                .expect("Start pipe deveria ter EXATAMENTE 2 lados conectados"),
        );
        *main_loop_matrix.index_mut(start_pos) = start_pipe;

        while let Some(pos) = queue.pop_front() {
            let item = self.matrix.index(pos);
            // Assumindo que não tem dead ends e que sempre os pipes conectam
            let sides = item.get_sides();
            let sides = sides.expect("Não implementado dead end");
            for side in sides {
                let Some(pos) = side.get_pos(pos) else {
                    continue;
                };
                if matches!(main_loop_matrix.index(pos), Pipe::Ground) {
                    *main_loop_matrix.index_mut(pos) = *self.matrix.index(pos);
                    queue.push_back(pos);
                }
            }
        }

        Sketch {
            matrix: main_loop_matrix,
            start: self.start,
        }
    }

    fn get_tiles_enclosed_by_main_loop(&self) -> usize {
        let mut tiles_enclosed_by_main_loop = 0;

        for y in 0..self.matrix.rows() {
            for x in 0..self.matrix.columns() {
                if matches!(self.matrix.index((y, x)), Pipe::Ground) {
                    let inversions = (0..x)
                        .map(|column| self.matrix.index((y, column)).is_inversion())
                        .filter(|v| *v)
                        .count();

                    if inversions % 2 == 1 {
                        tiles_enclosed_by_main_loop += 1;
                    }
                }
            }
        }

        tiles_enclosed_by_main_loop
    }
}

impl fmt::Display for Sketch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.matrix.rows() {
            for x in 0..self.matrix.columns() {
                write!(f, "{}", *self.matrix.index((y, x)))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{Pipe, Sketch};
    use matrix::matrix;

    #[test]
    fn parse() {
        let sketch = Sketch::from("./examples/simple.txt");
        let expected = matrix![
            Pipe::Ground, Pipe::Ground,    Pipe::Ground,     Pipe::Ground,    Pipe::Ground;
            Pipe::Ground, Pipe::Start,     Pipe::Horizontal, Pipe::SouthWest, Pipe::Ground;
            Pipe::Ground, Pipe::Vertical,  Pipe::Ground,     Pipe::Vertical,  Pipe::Ground;
            Pipe::Ground, Pipe::NorthEast, Pipe::Horizontal, Pipe::NorthWest, Pipe::Ground;
            Pipe::Ground, Pipe::Ground,    Pipe::Ground,     Pipe::Ground,    Pipe::Ground;
        ];
        assert_eq!(&*sketch.matrix, &*expected);
    }

    #[test]
    fn steps_to_farthest_point_simple() {
        let sketch = Sketch::from("./examples/simple.txt");
        assert_eq!(sketch.get_steps_to_farthest_point(), 4);
    }

    #[test]
    fn steps_to_farthest_point_complex() {
        let sketch = Sketch::from("./examples/complex.txt");
        assert_eq!(sketch.get_steps_to_farthest_point(), 8)
    }

    #[test]
    fn tiles_enclosed_by_main_loop() {
        let sketch = Sketch::from("./examples/inside_loop_1.txt");
        assert_eq!(sketch.get_main_loop().get_tiles_enclosed_by_main_loop(), 4);
    }

    #[test]
    fn tiles_enclosed_by_main_loop_2() {
        let sketch = Sketch::from("./examples/inside_loop_2.txt");
        assert_eq!(sketch.get_main_loop().get_tiles_enclosed_by_main_loop(), 4);
    }

    #[test]
    fn tiles_enclosed_by_main_loop_3() {
        let sketch = Sketch::from("./examples/inside_loop_3.txt");
        assert_eq!(sketch.get_main_loop().get_tiles_enclosed_by_main_loop(), 8);
    }

    #[test]
    fn tiles_enclosed_by_main_loop_4() {
        let sketch = Sketch::from("./examples/inside_loop_4.txt");
        assert_eq!(sketch.get_main_loop().get_tiles_enclosed_by_main_loop(), 10);
    }
}
