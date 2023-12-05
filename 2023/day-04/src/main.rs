use std::{fs, str::FromStr};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mw_nums = input
        .lines()
        .map(|line| line.parse::<Card>().unwrap())
        .map(|card| card.get_winning_nums())
        .collect::<Vec<_>>();
    let points = get_points(&mw_nums);
    println!("Part one, points: {points}");
}

#[derive(Debug, PartialEq)]
struct Card {
    id: u8,
    w_nums: Vec<u8>,
    m_nums: Vec<u8>,
}

impl Card {
    fn get_winning_nums(&self) -> Vec<u8> {
        self.w_nums
            .iter()
            .filter(|&w_num| self.m_nums.contains(w_num))
            .copied()
            .collect::<Vec<_>>()
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(':');
        let id = iter
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u8>()
            .unwrap();

        let mut nums = iter.next().unwrap().split('|');

        let mut parse_nums = || {
            nums.next()
                .unwrap()
                .split_whitespace()
                .map(|w| w.parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        };

        Ok(Card {
            id,
            w_nums: parse_nums(),
            m_nums: parse_nums(),
        })
    }
}

fn get_points(mw_nums: &[Vec<u8>]) -> u64 {
    mw_nums
        .iter()
        .map(|mw_nums| mw_nums.len())
        .map(|mw_nums| {
            mw_nums
                .checked_sub(1)
                .map_or(0, |exp| 2_u64.pow(exp as u32))
        })
        .sum::<u64>()
}

#[cfg(test)]
mod test {
    const INPUT: &str = r"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    ";

    mod part_one {
        use crate::{get_points, Card};

        use super::INPUT;

        #[test]
        fn parse() {
            let input = "Card 13: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
            let card = input.parse::<Card>().unwrap();
            let expected = Card {
                id: 13,
                w_nums: vec![41, 48, 83, 86, 17],
                m_nums: vec![83, 86, 6, 31, 17, 9, 48, 53],
            };
            assert_eq!(card, expected)
        }

        #[test]
        fn t1() {
            let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
            let card = input.parse::<Card>().unwrap();
            assert_eq!(card.get_winning_nums(), vec![48, 83, 86, 17]);
        }

        #[test]
        fn t2() {
            let input = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
            let card = input.parse::<Card>().unwrap();
            assert_eq!(card.get_winning_nums(), vec![32, 61]);
        }

        #[test]
        fn t3() {
            let input = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
            let card = input.parse::<Card>().unwrap();
            assert_eq!(card.get_winning_nums(), vec![1, 21]);
        }

        #[test]
        fn t4() {
            let input = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
            let card = input.parse::<Card>().unwrap();
            assert_eq!(card.get_winning_nums(), vec![84]);
        }

        #[test]
        fn t5() {
            let input = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
            let card = input.parse::<Card>().unwrap();
            assert_eq!(card.get_winning_nums(), vec![]);
        }

        #[test]
        fn t6() {
            let input = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
            let card = input.parse::<Card>().unwrap();
            assert_eq!(card.get_winning_nums(), vec![]);
        }

        #[test]
        fn points() {
            let mw_nums = INPUT
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .map(|line| line.parse::<Card>().unwrap())
                .map(|card| card.get_winning_nums())
                .collect::<Vec<_>>();
            let points = get_points(&mw_nums);
            assert_eq!(points, 13);
        }
    }
}
