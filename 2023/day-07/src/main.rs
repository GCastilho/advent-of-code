use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap, fs, str::FromStr};
use strum_macros::EnumString;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut hands = input
        .lines()
        .map(|line| line.parse::<Hand>().unwrap())
        .collect_vec();
    hands.sort();
    let total_winnings = get_total_winnings(&hands);
    println!("Part one, total winnings are {total_winnings}");
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug, PartialEq, PartialOrd, EnumString, Eq, Hash, Clone, Copy)]
enum Card {
    J,
    #[strum(serialize = "2")]
    Two = 2,
    #[strum(serialize = "3")]
    Three,
    #[strum(serialize = "4")]
    Four,
    #[strum(serialize = "5")]
    Five,
    #[strum(serialize = "6")]
    Six,
    #[strum(serialize = "7")]
    Seven,
    #[strum(serialize = "8")]
    Eight,
    #[strum(serialize = "9")]
    Nine,
    T,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    points: u16,
    cards: [Card; 5],
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut items = HashMap::new();
        self.cards.iter().for_each(|card| {
            items.entry(card).and_modify(|v| *v += 1).or_insert(1);
        });
        if let Some(&joker_count) = items.get(&Card::J) {
            let biggest_card_count = items.iter().fold(None, |acc, (&card, &count)| match card {
                Card::J => acc,
                _ => match acc {
                    None => Some((*card, count)),
                    Some((_, acc_count)) => {
                        if !matches!(card, Card::J) && count > acc_count {
                            Some((*card, count))
                        } else {
                            acc
                        }
                    }
                },
            });
            if let Some((card, _)) = biggest_card_count {
                *items.get_mut(&card).unwrap() += joker_count;
                items.remove(&Card::J);
            }
        }
        if items.len() == 1 {
            HandType::FiveOfKind
        } else if items.len() == 2 && items.iter().any(|(_, c)| *c == 4) {
            HandType::FourOfKind
        } else if items.len() == 2 && items.iter().any(|(_, c)| *c == 3) {
            HandType::FullHouse
        } else if items.len() == 3 && items.iter().any(|(_, c)| *c == 3) {
            HandType::ThreeOfKind
        } else if items.len() == 3 && items.iter().filter(|(_, c)| **c == 2).count() == 2 {
            HandType::TwoPair
        } else if items.len() == 4 && items.iter().filter(|(_, c)| **c == 1).count() == 3 {
            HandType::OnePair
        } else if items.len() == 5 {
            HandType::HighCard
        } else {
            panic!("HandType not found {items:?} {:?}", self.cards);
        }
    }
}

impl FromStr for Hand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let cards = words
            .next()
            .unwrap()
            .chars()
            .map(|c| c.to_string().parse::<Card>().unwrap())
            .collect_vec()
            .try_into()
            .unwrap();
        let points = words.next().unwrap().parse::<u16>().unwrap();
        Ok(Self { points, cards })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_type()
            .partial_cmp(&other.get_type())
            .and_then(|types| {
                if matches!(types, Ordering::Equal) {
                    self.cards.partial_cmp(&other.cards)
                } else {
                    Some(types)
                }
            })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn get_total_winnings(hands: &[Hand]) -> u64 {
    hands.iter().enumerate().fold(0, |acc, (i, card)| {
        acc + (i as u64 + 1) * card.points as u64
    })
}

#[cfg(test)]
mod test {
    use crate::Hand;
    use itertools::Itertools;
    use std::fs;

    fn get_input() -> String {
        fs::read_to_string("./example.txt").unwrap()
    }

    fn get_cards() -> Vec<Hand> {
        get_input()
            .lines()
            .map(|line| line.parse::<Hand>().unwrap())
            .collect_vec()
    }

    mod part_two {
        use crate::{get_total_winnings, test::get_cards, Card, Hand, HandType};

        #[test]
        fn enum_compare() {
            assert!(HandType::FiveOfKind > HandType::FourOfKind);
            assert!(HandType::FourOfKind > HandType::FullHouse);
            assert!(HandType::FullHouse > HandType::ThreeOfKind);
            assert!(HandType::ThreeOfKind > HandType::TwoPair);
            assert!(HandType::TwoPair > HandType::OnePair);
            assert!(HandType::OnePair > HandType::HighCard);
        }

        #[test]
        fn parse() {
            let hand = "32T3K 765".parse::<Hand>().unwrap();
            assert_eq!(
                hand,
                Hand {
                    points: 765,
                    cards: [Card::Three, Card::Two, Card::T, Card::Three, Card::K]
                }
            )
        }

        fn get_hand_type(hand_word: &str) -> Hand {
            let line = format!("{hand_word} 123");
            line.parse().unwrap()
        }

        #[test]
        fn type_five() {
            let card = get_hand_type("AAAAA");
            assert_eq!(card.get_type(), HandType::FiveOfKind);
        }

        #[test]
        fn type_four() {
            let card = get_hand_type("AA8AA");
            assert_eq!(card.get_type(), HandType::FourOfKind);
        }

        #[test]
        fn type_house() {
            let card = get_hand_type("23332");
            assert_eq!(card.get_type(), HandType::FullHouse);
        }

        #[test]
        fn type_three() {
            let card = get_hand_type("TTT98");
            assert_eq!(card.get_type(), HandType::ThreeOfKind);
        }

        #[test]
        fn type_two() {
            let card = get_hand_type("23432");
            assert_eq!(card.get_type(), HandType::TwoPair);
        }

        #[test]
        fn type_one() {
            let card = get_hand_type("A23A4");
            assert_eq!(card.get_type(), HandType::OnePair);
        }

        #[test]
        fn type_high() {
            let card = get_hand_type("23456");
            assert_eq!(card.get_type(), HandType::HighCard);
        }

        #[test]
        fn ordering() {
            let mut cards = get_cards();
            cards.sort();
            assert_eq!(
                cards,
                vec![
                    Hand {
                        cards: [Card::Three, Card::Two, Card::T, Card::Three, Card::K],
                        points: 765
                    },
                    Hand {
                        cards: [Card::K, Card::K, Card::Six, Card::Seven, Card::Seven],
                        points: 28
                    },
                    Hand {
                        cards: [Card::T, Card::Five, Card::Five, Card::J, Card::Five],
                        points: 684
                    },
                    Hand {
                        cards: [Card::Q, Card::Q, Card::Q, Card::J, Card::A],
                        points: 483
                    },
                    Hand {
                        cards: [Card::K, Card::T, Card::J, Card::J, Card::T],
                        points: 220
                    },
                ]
            )
        }

        #[test]
        fn total_winnings() {
            let mut cards = get_cards();
            cards.sort();
            assert_eq!(get_total_winnings(&cards), 5905);
        }
    }
}
