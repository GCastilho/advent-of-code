mod monkey;

use std::{fs, collections::VecDeque};
use monkey::Monkey;

const FILENAME: &str = "example.txt";
// const FILENAME: &str = "input.txt";
const ROUNDS: u8 = 1;

fn main() {
    let input = fs::read_to_string(FILENAME).unwrap();

    let mut monkeys = input
        .split_terminator("\n\n")
        .map(|monkey| Monkey::new(monkey))
        .collect::<Vec<_>>();

    let mut inspected = Vec::new();
    for _ in 0..monkeys.len() {
        inspected.push(0);
    }

    for _round in 0..ROUNDS {
        let mut changes: Vec<VecDeque<i32>> = Vec::new();
        monkeys.iter().for_each(|_| { changes.push(VecDeque::new()); });

        for (index, monkey) in monkeys.iter_mut().enumerate() {
            // println!("{:?}", monkey);

            for change in changes[index].iter() {
                monkey.items.push_back(*change);
            }

            inspected[index] += monkey.items.len();
            for item in monkey.items.iter() {
                let item = monkey.inspect(*item);
                // println!("{} item {}", monkey.name, item);
                let item = item / 3;
                // println!("{} item {}", monkey.name, item);
                let change_position = match monkey.test(item) {
                    true => monkey.dest.0,
                    false => monkey.dest.1,
                };
                // println!("{} change_position {}", monkey.name, change_position);
                let vec = changes.get_mut(change_position).unwrap();
                vec.push_back(item);
            }
            monkey.items = VecDeque:: new();

            // println!("{:?}", changes);
        }

        for (index, change) in changes.iter_mut().enumerate() {
            monkeys[index].items.append(change);
        }
    }

    println!("monkeys: {:?}", monkeys);

    inspected.sort();
    let two_most_active = inspected.split_at(inspected.len() - 2).1;
    let two_most_active = two_most_active[0] * two_most_active[1];

    println!("monkey business level: {}", two_most_active);
}
