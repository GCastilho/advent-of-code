use std::{fs, collections::HashSet};

// const FILENAME: &str = "example.txt";
const FILENAME: &str = "input.txt";

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn is_same(p1: (i32, i32), p2: (i32, i32)) -> bool {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    x1 == x2 && y1 == y2
}

fn is_adjacent(p1: (i32, i32), p2: (i32, i32)) -> bool {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let dx = (x1 - x2).abs();
    let dy = (y1 - y2).abs();
    dx + dy == 1 || dx > 0 && dy > 0 && dx + dy == 2
}

fn main() {
    let input = fs::read_to_string(FILENAME).unwrap();

    let motions = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|row| row.split_whitespace().collect::<Vec<_>>())
        .map(|row| (row[0], row[1].parse::<u8>().unwrap()))
        .map(|(motion, steps)| {
            match motion {
                "U" => (Direction::Up, steps),
                "D" => (Direction::Down,steps),
                "L" => (Direction::Left, steps),
                "R" => (Direction::Right, steps),
                _ => panic!("unexpected motion: {}", motion),
            }
        })
        .collect::<Vec<_>>();

    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut places = vec![(0, 0)];

    for (direction, steps) in motions.iter() {
        for _ in 0..*steps {
            let old_head = head.clone();
            match direction {
                Direction::Up => head.0 += 1,
                Direction::Down => head.0 -= 1,
                Direction::Left => head.1 -= 1,
                Direction::Right => head.1 += 1,
            };
            if is_same(head, tail) || is_adjacent(head, tail){
                continue;
            }
            tail = old_head;
            places.push(tail);
        }
    }

    let mut uniques = HashSet::new();
    for item in places.iter() {
        uniques.insert(item);
    }

    println!("uniques: {}", uniques.len());
}
