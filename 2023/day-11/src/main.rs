use observatory::{Observation, Raw};
use position::{Coordinates, Pair};
use std::fs;

mod observatory;
mod position;

fn main() {
    let observation = fs::read_to_string("./input.txt")
        .expect("read input")
        .parse::<Observation<Raw>>()
        .expect("parse observation");

    let galaxies = observation.expand(2).to_galaxies();
    let min_distances = calculate_min_distance_sum(galaxies);
    println!("Part one: The the sum of the lengths is {min_distances}");

    let galaxies = observation.expand(10).to_galaxies();
    let min_distances = calculate_min_distance_sum(galaxies);
    println!("Part two: The the sum of the lengths is {min_distances}");
}

fn calculate_min_distance_sum(galaxies: Vec<Coordinates>) -> usize {
    let mut pairs_seen = Vec::new();
    galaxies
        .iter()
        .flat_map(|galaxy| {
            let pairs = galaxies
                .iter()
                .filter(|point| *point != galaxy)
                .map(|point| Pair::new(*galaxy, *point))
                .filter(|pair| {
                    pairs_seen
                        .iter()
                        .position(|v| v == pair)
                        .map(|position| {
                            pairs_seen.swap_remove(position);
                            false
                        })
                        .unwrap_or(true)
                })
                .collect::<Vec<_>>();
            pairs.iter().for_each(|pair| pairs_seen.push(*pair));
            pairs
                .into_iter()
                .map(|pair| pair.into_coordinates())
                .map(|(a, b)| a.distance_to(&b))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part1() {
        let galaxies = fs::read_to_string("./examples/simple.txt")
            .expect("read input")
            .parse::<Observation<Raw>>()
            .expect("parse observation")
            .expand(2)
            .to_galaxies();
        let min_distance_sum = calculate_min_distance_sum(galaxies);
        assert_eq!(min_distance_sum, 374);
    }

    #[test]
    fn part2_10() {
        let galaxies = fs::read_to_string("./examples/simple.txt")
            .expect("read input")
            .parse::<Observation<Raw>>()
            .expect("parse observation")
            .expand(10)
            .to_galaxies();
        let min_distance_sum = calculate_min_distance_sum(galaxies);
        assert_eq!(min_distance_sum, 1030);
    }

    #[test]
    fn part2_100() {
        let galaxies = fs::read_to_string("./examples/simple.txt")
            .expect("read input")
            .parse::<Observation<Raw>>()
            .expect("parse observation")
            .expand(100)
            .to_galaxies();
        let min_distance_sum = calculate_min_distance_sum(galaxies);
        assert_eq!(min_distance_sum, 8410);
    }
}
