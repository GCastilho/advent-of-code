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

    println!("start-of-packet: {}", total_amount);

    let max_queue_size = 14;
    let mut queue = CircularQueue::with_capacity(max_queue_size);
    let mut total_amount = 0;

    for char in input.chars().take_while(|char| *char != '\n') {
        queue.push(char);
        total_amount += 1;

        let mut uniq = HashSet::new();
        for c in queue.iter() {
            uniq.insert(*c);
        }
        if uniq.len() == max_queue_size { break; }
    }

    println!("start-of-message: {}", total_amount);
}
