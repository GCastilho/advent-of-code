use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let races = Race::from_str(&input);

    let margin = get_part_one(&races);
    println!("Part one, total margin is: {margin}");
}

#[derive(Debug, PartialEq)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn from_str(s: &str) -> Vec<Self> {
        let mut iter = s.lines();
        let mut parse_line = || {
            iter.next()
                .unwrap()
                .split_whitespace()
                .skip(1)
                .map(|w| w.parse::<u32>().unwrap())
        };
        let times = parse_line();
        let distances = parse_line();

        times
            .zip(distances)
            .map(|(time, distance)| Self { time, distance })
            .collect_vec()
    }

    fn get_distance(&self, time_holding: u32) -> u32 {
        time_holding * self.time - time_holding * time_holding
    }

    fn get_all_distances(&self) -> Vec<u32> {
        (0..=self.time)
            .map(|time_holding| self.get_distance(time_holding))
            .collect_vec()
    }

    fn get_winning_times(&self) -> Vec<u32> {
        self.get_all_distances()
            .iter()
            .enumerate()
            .filter(|(_, &d)| d > self.distance)
            .map(|(i, _)| i as u32)
            .collect_vec()
    }
}

fn get_part_one(races: &[Race]) -> u32 {
    races
        .iter()
        .map(|race| race.get_winning_times().len())
        .reduce(|acc, cur| acc * cur)
        .map(|n| n as u32)
        .unwrap()
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::{get_part_one, Race};

    fn get_input() -> String {
        fs::read_to_string("./example.txt").unwrap()
    }

    #[test]
    fn parse() {
        let races = Race::from_str(&get_input());
        assert_eq!(
            races,
            vec![
                Race {
                    time: 7,
                    distance: 9
                },
                Race {
                    time: 15,
                    distance: 40
                },
                Race {
                    time: 30,
                    distance: 200
                }
            ]
        );
    }

    #[test]
    fn distance() {
        let race = Race {
            time: 7,
            distance: 9,
        };
        assert_eq!(race.get_distance(0), 0);
        assert_eq!(race.get_distance(1), 6);
        assert_eq!(race.get_distance(2), 10);
        assert_eq!(race.get_distance(3), 12);
        assert_eq!(race.get_distance(4), 12);
        assert_eq!(race.get_distance(5), 10);
        assert_eq!(race.get_distance(6), 6);
        assert_eq!(race.get_distance(7), 0);
    }

    #[test]
    fn all_disances() {
        let race = Race {
            time: 7,
            distance: 9,
        };
        assert_eq!(race.get_all_distances(), vec![0, 6, 10, 12, 12, 10, 6, 0])
    }

    #[test]
    fn winning_times() {
        let race = Race {
            time: 7,
            distance: 9,
        };
        assert_eq!(race.get_winning_times(), vec![2, 3, 4, 5])
    }

    #[test]
    fn part_one() {
        let races = Race::from_str(&get_input());
        let margins = get_part_one(&races);
        assert_eq!(margins, 288);
    }
}
