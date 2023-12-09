use itertools::Itertools;
use std::{fs, str::FromStr};

fn main() {
    let sequences = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Sequence>>();
    let extrapolated_values = sequences.iter().map(|s| s.find_next()).sum::<i64>();
    println!("Part one, the sum of the extrapolated are {extrapolated_values}");
}

#[derive(Debug, PartialEq)]
struct Sequence(Vec<i64>);

impl Sequence {
    fn find_next(&self) -> i64 {
        let mut sequences = vec![self.0.clone()];

        while !sequences.last().unwrap().iter().all(|v| *v == 0) {
            let next_sequence = sequences
                .last()
                .unwrap()
                .iter()
                .tuple_windows::<(_, _)>()
                .map(|(left, right)| right - left)
                .collect_vec();
            sequences.push(next_sequence)
        }

        sequences
            .iter()
            .fold(0, |acc, sequence| acc + sequence.last().unwrap())
    }
}

impl FromStr for Sequence {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sequence = s
            .split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect();
        Ok(Self(sequence))
    }
}

#[cfg(test)]
mod test {
    use crate::Sequence;
    use std::fs;

    fn get_input() -> Vec<Sequence> {
        fs::read_to_string("./example.txt")
            .unwrap()
            .lines()
            .map(|line| line.parse().unwrap())
            .collect()
    }

    mod part_one {
        use super::get_input;
        use crate::Sequence;

        #[test]
        fn parse() {
            let sequences = get_input();
            assert_eq!(
                sequences,
                vec![
                    Sequence(vec![0, 3, 6, 9, 12, 15]),
                    Sequence(vec![1, 3, 6, 10, 15, 21]),
                    Sequence(vec![10, 13, 16, 21, 30, 45]),
                ]
            )
        }

        #[test]
        fn find_next() {
            let next_sum = get_input().iter().map(|s| s.find_next()).sum::<i64>();
            assert_eq!(next_sum, 114);
        }
    }
}
