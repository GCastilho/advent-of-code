use std::{fs, str::FromStr};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut cards = input
        .lines()
        .map(|line| line.parse::<Card>().unwrap())
        .collect::<Vec<_>>();

    let mw_nums = cards
        .iter()
        .map(|card| card.get_winning_nums())
        .collect::<Vec<_>>();
    let points = get_points(&mw_nums);
    println!("Part one, points: {points}");

    process_cards(&mut cards);
    let card_count = cards.iter().map(|c| c.instances).sum::<u64>();
    println!("Part two, cards: {card_count}");
}

#[derive(Debug, PartialEq)]
struct Card {
    id: u8,
    w_nums: Vec<u8>,
    m_nums: Vec<u8>,
    instances: u64,
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
            instances: 1,
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

fn process_cards(cards: &mut [Card]) {
    for current in 0..cards.len() {
        let current_card = &cards[current];
        let w_cards = current_card.get_winning_nums().len();
        for _ in 0..current_card.instances {
            for card in cards.iter_mut().skip(current + 1).take(w_cards) {
                card.instances += 1;
            }
        }
    }
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
                instances: 1,
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

    mod part_two {
        use super::INPUT;
        use crate::{process_cards, Card};

        fn get_cards() -> Vec<Card> {
            INPUT
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .map(|line| line.parse::<Card>().unwrap())
                .collect()
        }

        #[derive(Debug, PartialEq)]
        struct CardTrimmed {
            id: u8,
            instances: u64,
        }

        #[test]
        fn to_process_cards() {
            let mut cards = get_cards();
            process_cards(&mut cards);
            let cards = cards
                .into_iter()
                .map(|card| CardTrimmed {
                    id: card.id,
                    instances: card.instances,
                })
                .collect::<Vec<_>>();
            assert_eq!(
                cards,
                vec![
                    CardTrimmed {
                        id: 1,
                        instances: 1
                    },
                    CardTrimmed {
                        id: 2,
                        instances: 2
                    },
                    CardTrimmed {
                        id: 3,
                        instances: 4
                    },
                    CardTrimmed {
                        id: 4,
                        instances: 8
                    },
                    CardTrimmed {
                        id: 5,
                        instances: 14
                    },
                    CardTrimmed {
                        id: 6,
                        instances: 1
                    },
                ]
            );
            let card_count = cards.iter().map(|c| c.instances).sum::<u64>();
            assert_eq!(card_count, 30)
        }
    }
}
