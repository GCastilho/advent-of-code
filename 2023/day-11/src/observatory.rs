use core::fmt;
use matrix::format::Compressed;
use matrix::prelude::Transpose;
use matrix::Matrix;
use matrix::{format::Conventional, Element};
use std::str::FromStr;
use strum::EnumString;

use crate::position::Coordinates;

#[derive(Debug, strum::Display, EnumString, PartialEq, PartialOrd, Clone, Copy)]
pub enum Item {
    #[strum(serialize = ".")]
    Void,
    #[strum(serialize = "#")]
    Galaxy,
}

impl Element for Item {
    fn zero() -> Self {
        Self::Void
    }
}

pub trait ObservationState {}

#[derive(Debug)]
pub struct Observation<State: ObservationState> {
    matrix: Compressed<Item>,
    state: std::marker::PhantomData<State>,
}

pub struct Raw {}
pub struct Expanded {}

impl ObservationState for Raw {}
impl ObservationState for Expanded {}

impl FromStr for Observation<Raw> {
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let image = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<Item>())
                    .collect()
            })
            .collect::<Result<Vec<Vec<_>>, strum::ParseError>>()?;

        let y = image.len();
        let x = image.first().map_or(0, |v| v.len());
        let matrix = Conventional::from_vec((y, x), image.into_iter().flatten().collect());

        Ok(Self {
            matrix: Compressed::from(matrix.transpose()),
            state: std::marker::PhantomData,
        })
    }
}

impl<State: ObservationState> fmt::Display for Observation<State> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.matrix.rows {
            for x in 0..self.matrix.columns {
                write!(f, "{}", self.matrix.get((y, x)))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Observation<Raw> {
    pub fn expand(&self) -> Observation<Expanded> {
        let scan_offsets = |acc: &mut usize, is_void_column| {
            if is_void_column {
                *acc += 1;
            }
            Some(*acc)
        };

        let rows_offset = (0..self.matrix.rows)
            .map(|y| {
                (0..self.matrix.columns).all(|x| match self.matrix.get((y, x)) {
                    Item::Void => true,
                    Item::Galaxy => false,
                })
            })
            .scan(0, scan_offsets)
            .collect::<Vec<_>>();
        let columns_offset = (0..self.matrix.columns)
            .map(|x| {
                (0..self.matrix.rows).all(|y| match self.matrix.get((y, x)) {
                    Item::Void => true,
                    Item::Galaxy => false,
                })
            })
            .scan(0, scan_offsets)
            .collect::<Vec<_>>();

        let y = self.matrix.rows + rows_offset.last().copied().unwrap_or_default();
        let x = self.matrix.columns + columns_offset.last().copied().unwrap_or_default();
        let mut expanded = Compressed::zero((y, x));

        for (y, y_offset) in rows_offset.iter().enumerate().take(self.matrix.rows) {
            for (x, x_offset) in columns_offset.iter().enumerate().take(self.matrix.columns) {
                let y1 = y + *y_offset;
                let x1 = x + *x_offset;
                expanded.set((y1, x1), self.matrix.get((y, x)));
            }
        }

        Observation {
            matrix: expanded,
            state: std::marker::PhantomData,
        }
    }
}

impl Observation<Expanded> {
    pub fn to_galaxies(&self) -> Vec<Coordinates> {
        (0..self.matrix.rows)
            .flat_map(|y| (0..self.matrix.columns).map(move |x| (y, x)))
            .filter(|position| !self.matrix.get(*position).is_zero())
            .map(Coordinates::new)
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse() {
        let input = fs::read_to_string("./examples/simple.txt").expect("read input");
        let observation = input
            .parse::<Observation<Raw>>()
            .expect("parse observation");
        assert_eq!(input, observation.to_string());
    }

    #[test]
    fn expand() {
        let input = fs::read_to_string("./examples/simple.txt").expect("read input");
        let expanded = fs::read_to_string("./examples/simple_expanded.txt").expect("read expanded");
        let observation = input
            .parse::<Observation<Raw>>()
            .expect("parse observation")
            .expand();
        assert_eq!(expanded, observation.to_string());
    }
}
