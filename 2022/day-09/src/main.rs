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

fn is_same(p1: &(i32, i32), p2: &(i32, i32)) -> bool {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    x1 == x2 && y1 == y2
}

fn is_adjacent(p1: &(i32, i32), p2: &(i32, i32)) -> bool {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let dx = (x1 - x2).abs();
    let dy = (y1 - y2).abs();
    dx + dy == 1 || dx > 0 && dy > 0 && dx + dy == 2
}

fn should_move(p1: &(i32, i32), p2: &(i32, i32)) -> bool {
    !is_same(p1, p2) && !is_adjacent(p1, p2)
}

fn move_knot(head: &(i32, i32), tail: &mut (i32, i32)) {
    let (hx, hy) = *head;
    let (tx, ty) = *tail;

    let x_diff = hx - tx;
    let y_diff = hy - ty;

    if (hx - tx).abs() > 1 || (hy - ty).abs() > 1 {
        if x_diff > 0 {
            tail.0 += 1;
        } else if x_diff < 0 {
            tail.0 -= 1;
        }
        if y_diff > 0 {
            tail.1 += 1;
        } else if y_diff < 0 {
            tail.1 -= 1;
        }
    }
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

    let mut knots = vec![];
    let nodes = 10;
    for _ in 0..nodes {
        knots.push((0, 0));
    }

    let mut uniques = HashSet::new();
    uniques.insert((0, 0));

    for (direction, steps) in motions.iter() {
        for _ in 0..*steps {
            let mut head = knots.first_mut().unwrap();
            match direction {
                Direction::Up => head.0 += 1,
                Direction::Down => head.0 -= 1,
                Direction::Left => head.1 -= 1,
                Direction::Right => head.1 += 1,
            };

            let mut current_head = head.clone();

            let iter = knots.iter_mut().skip(1);
            for knot in iter {
                if should_move(&current_head, knot) {
                    move_knot(&current_head, knot);
                }
                current_head = knot.clone();
            }

            // println!("{:?}", knots);

            let tail = knots.last().unwrap();
            uniques.insert(*tail);
        }
    }

    println!("uniques: {}", uniques.len());
}
