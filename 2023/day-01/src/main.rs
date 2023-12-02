use std::fs;

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

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let calibration_sum = file
        .lines()
        .map(get_calibration)
        .map(|v| v as u64)
        .sum::<u64>();
    println!("part one: {calibration_sum}");
}

#[cfg(test)]
mod test {
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
