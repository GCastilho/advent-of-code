use std::{fs, collections::HashSet};
use circular_queue::CircularQueue;

// const FILENAME: &str = "example.txt";
const FILENAME: &str = "input.txt";

fn main() {
    let input = fs::read_to_string(FILENAME).unwrap();

    let mut queue = CircularQueue::with_capacity(4);
    let mut total_amount = 0;

    for char in input.chars().take_while(|char| *char != '\n') {
        queue.push(char);
        total_amount += 1;

        let mut uniq = HashSet::new();
        queue.iter().for_each(|c| {
            uniq.insert(c);
        });
        if uniq.len() == 4 { break; }
    }

    println!("counter: {}", total_amount);
}
