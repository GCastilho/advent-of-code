use std::{collections::HashMap, fs};

fn init_priority() -> HashMap<char, usize> {
    let mut priority = HashMap::new();

    ('a'..='z')
        .enumerate()
        .for_each(|(i, v)| {
            priority.insert(v, i + 1);
        });

    ('A'..='Z')
        .enumerate()
        .for_each(|(i, v)| {
            priority.insert(v, i + 27);
        });

    assert_eq!(priority.len(), 26 * 2);
    priority
}

fn main() {
    let prority_map = init_priority();

    let input = fs::read_to_string("input.txt").unwrap();

    let priority_sum = input
        .split_terminator("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .map(|char_vec| {
            let middle = char_vec.len() / 2;
            let (first, second) = (&char_vec[..middle], &char_vec[middle..]);

            let duplicated = second.iter()
                .find(|&item| first.contains(item))
                .expect("Expected first half to contain an item from the second half");
            duplicated.clone()
        })
        .filter_map(|duplicated| prority_map.get(&duplicated))
        .sum::<usize>();

    println!("Priority sum: {}", priority_sum);

    let badge_priority_sum = input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .chunks(3)
        .filter_map(|group| {
            let first = group[0];
            let treplicated = first.chars().find(|item| {
                let second = group[1];
                let third = group[2];
                second.contains(*item) && third.contains(*item)
            }).expect("Expected a char to be on all 3 lines");
            prority_map.get(&treplicated)
        })
        .sum::<usize>();

    println!("Badge priority sum: {}", badge_priority_sum);
}
