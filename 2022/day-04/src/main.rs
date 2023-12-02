use std::fs;
use std::ops;

#[derive(Debug)]
struct Section {
    start: u32,
    end: u32,
}

impl Section {
    fn from_string(range: &str) -> Section {
        let mut arr = range
            .split("-")
            .filter_map(|n| n.parse::<u32>().ok());
        let start = arr.next().unwrap();
        let end = arr.next().unwrap();
        Section { start, end }
    }

    fn intersect(&self, other: &Section) -> bool {
        self.start <= other.start && self.end >= other.end ||
        other.start <= self.start && other.end >= self.end
    }

    fn as_range(&self) -> ops::RangeInclusive<u32> {
        self.start..=self.end
    }

    fn overlap(&self, other: &Section) -> bool {
        let mut self_range = self.as_range();
        let other_range = other.as_range();
        self_range.any(|v| other_range.contains(&v))
    }
}

fn main() {
    // let filename = "example.txt";
    let filename = "input.txt";
    let input = fs::read_to_string(filename).unwrap();

    let sections = input
        .split_terminator("\n")
        .map(|line| {
            let mut sections = line
                .split(",")
                .map(|section| Section::from_string(section));
            (sections.next().unwrap(), sections.next().unwrap())
        });

    let intersect = sections
        .clone()
        .map(|(s1, s2)| s1.intersect(&s2))
        .map(|x| x as u32)
        .sum::<u32>();
    println!("Sections that intersect: {}", intersect);

    let overlap = sections
        .map(|(s1, s2)| s1.overlap(&s2))
        .map(|x| x as u32)
        .sum::<u32>();
    println!("Sections that overlap: {}", overlap);
}
