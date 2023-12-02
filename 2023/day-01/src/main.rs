use lazy_static::lazy_static;
use std::{collections::HashMap, fs};

fn get_calibration(line: &str) -> u8 {
    let first = line
        .chars()
        .find(|c| c.is_numeric())
        .and_then(|c| c.to_digit(10))
        .unwrap_or_default();
    let last = line
        .chars()
        .rev()
        .find(|c| c.is_numeric())
        .and_then(|c| c.to_digit(10))
        .unwrap_or_default();
    (first * 10 + last).try_into().unwrap()
}

lazy_static! {
    static ref WORDS: HashMap<&'static str, &'static str> = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "th3ee"),
        ("four", "fo4r"),
        ("five", "fi5e"),
        ("six", "s6x"),
        ("seven", "se7en"),
        ("eight", "ei8ht"),
        ("nine", "ni9e")
    ]);
}

fn get_fancy_calibration(line: &str) -> u8 {
    let mut line = String::from(line);
    for (k, v) in WORDS.iter() {
        line = line.replace(k, v);
    }

    let first = line
        .chars()
        .find(|c| c.is_numeric())
        .and_then(|c| c.to_digit(10))
        .unwrap_or_default();
    let last = line
        .chars()
        .rev()
        .find(|c| c.is_numeric())
        .and_then(|c| c.to_digit(10))
        .unwrap_or_default();
    (first * 10 + last).try_into().unwrap()
}

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let calibration_sum = file
        .lines()
        .map(get_calibration)
        .map(|v| v as u64)
        .sum::<u64>();
    println!("part one: {calibration_sum}");
    let calibration_sum = file
        .lines()
        .map(get_fancy_calibration)
        .map(|v| v as u64)
        .sum::<u64>();
    println!("part two: {calibration_sum}");
}

#[cfg(test)]
mod test {
    mod part_one {
        use crate::get_calibration;

        #[test]
        fn first() {
            let value = get_calibration("1abc2");
            assert_eq!(value, 12)
        }

        #[test]
        fn second() {
            let value = get_calibration("pqr3stu8vwx");
            assert_eq!(value, 38)
        }

        #[test]
        fn third() {
            let value = get_calibration("a1b2c3d4e5f");
            assert_eq!(value, 15)
        }

        #[test]
        fn fourth() {
            let value = get_calibration("treb7uchet");
            assert_eq!(value, 77)
        }
    }

    mod part_two {
        use crate::get_fancy_calibration;

        #[test]
        fn t1() {
            let value = get_fancy_calibration("two1nine");
            assert_eq!(value, 29)
        }

        #[test]
        fn t2() {
            let value = get_fancy_calibration("eightwothree");
            assert_eq!(value, 83)
        }

        #[test]
        fn t3() {
            let value = get_fancy_calibration("abcone2threexyz");
            assert_eq!(value, 13)
        }

        #[test]
        fn t4() {
            let value = get_fancy_calibration("xtwone3four");
            assert_eq!(value, 24)
        }

        #[test]
        fn t5() {
            let value = get_fancy_calibration("4nineeightseven2");
            assert_eq!(value, 42)
        }

        #[test]
        fn t6() {
            let value = get_fancy_calibration("zoneight234");
            assert_eq!(value, 14)
        }

        #[test]
        fn t7() {
            let value = get_fancy_calibration("7pqrstsixteen");
            assert_eq!(value, 76)
        }
    }
}
