use std::{fs, str::FromStr};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let almanac = input.parse::<Almanac>().unwrap();

    let locations = almanac.find_locations();
    let lowest_location = locations.into_iter().min().unwrap();
    println!("Part one, lowest location is: {lowest_location}");

    let lowest_location_range = almanac.find_lowest_location_using_range();
    println!("Part two, lowest location with range is: {lowest_location_range}");
}

#[derive(Debug, PartialEq)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Vec<(u64, u64, u64)>>,
}

impl Almanac {
    fn convert_categories(&self, destination_category_idx: usize, numbers: &[u64]) -> Vec<u64> {
        let destination = self.maps.get(destination_category_idx).unwrap();

        let mut found = Vec::new();

        for number in numbers {
            let initial_len = found.len();
            for (d_range_start, s_range_start, len) in destination {
                if number >= s_range_start && *number < s_range_start + len {
                    let new_number = number - s_range_start + d_range_start;
                    found.push(new_number)
                }
            }
            if initial_len == found.len() {
                found.push(*number);
            }
        }

        found
    }

    fn find_locations(&self) -> Vec<u64> {
        let mut locations = self.seeds.clone();

        for i in 0..self.maps.len() {
            locations = self.convert_categories(i, &locations)
        }

        locations
    }

    fn find_lowest_location_using_range(&self) -> u64 {
        self.seeds
            .iter()
            .tuples::<(_, _)>()
            .flat_map(|(&first, &len)| (first..first + len))
            .flat_map(|seed| {
                let mut seed = vec![seed];
                for i in 0..self.maps.len() {
                    seed = self.convert_categories(i, &seed);
                }
                seed
            })
            .min()
            .unwrap()
    }
}

impl FromStr for Almanac {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seeds = s
            .lines()
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|w| w.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let blocks = s.split_terminator("\n\n").skip(1);

        let mut maps = Vec::new();
        for block in blocks {
            let mut map = Vec::new();
            for line in block.lines().skip(1) {
                let (destination_range_start, source_range_start, range_len) = line
                    .split_whitespace()
                    .map(|w| w.parse::<u64>().unwrap())
                    .collect_tuple()
                    .unwrap();
                map.push((destination_range_start, source_range_start, range_len))
            }
            maps.push(map)
        }

        Ok(Self { seeds, maps })
    }
}

#[cfg(test)]
mod test {
    use crate::Almanac;
    use std::{fs, str::FromStr};

    fn get_input() -> String {
        fs::read_to_string("./example.txt").unwrap()
    }

    #[test]
    fn parse() {
        let almanac = Almanac::from_str(&get_input()).unwrap();
        assert_eq!(
            almanac,
            Almanac {
                seeds: vec![79, 14, 55, 13],
                maps: vec![
                    vec![(50, 98, 2), (52, 50, 48)],
                    vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)],
                    vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)],
                    vec![(88, 18, 7), (18, 25, 70)],
                    vec![(45, 77, 23), (81, 45, 19), (68, 64, 13)],
                    vec![(0, 69, 1), (1, 0, 69)],
                    vec![(60, 56, 37), (56, 93, 4)]
                ]
            }
        );
    }

    fn get_almanac() -> Almanac {
        get_input().parse::<Almanac>().unwrap()
    }

    #[test]
    fn t1() {
        let almanac = get_almanac();
        let seeds = [79, 14, 55, 13];
        let soil_numbers = almanac.convert_categories(0, &seeds);
        assert_eq!(soil_numbers, vec![81, 14, 57, 13])
    }

    #[test]
    fn t2() {
        let almanac = get_almanac();
        let locations = almanac.find_locations();
        assert_eq!(locations, vec![82, 43, 86, 35]);
    }

    #[test]
    fn t3() {
        let almanac = get_almanac();
        let lowest = almanac.find_lowest_location_using_range();
        assert_eq!(lowest, 46);
    }
}
