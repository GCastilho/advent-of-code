use std::fs;

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
        })
        .map(|(s1, s2)| s1.intersect(&s2))
        .map(|x| x as u32)
        .sum::<u32>();
    println!("Sections: {}", sections);
}
