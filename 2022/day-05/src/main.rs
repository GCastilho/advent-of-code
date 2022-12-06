use std::{fs, collections::HashMap};

fn get_table(column_line: usize, input: &str) -> HashMap<usize, Vec<char>> {
    let columns = input
        .lines()
        .nth(column_line)
        .unwrap()
        .chars()
        .enumerate()
        .map(|(u, c)| {
            if c.is_numeric() {
                Some(u)
            } else {
                None
            }
        })
        .filter_map(|v| v)
        .collect::<Vec<_>>();

    let table = input
        .lines()
        .collect::<Vec<_>>();
    let table = table
        .split_at(column_line).0
        .iter()
        .map(|&v| v)
        .collect::<Vec<_>>();

    let mut hash_table = HashMap::new();

    columns
        .iter()
        .enumerate()
        .for_each(|(idx, column)| {
            let table = table
                .iter()
                .map(|&line| {
                    line.chars().nth(*column).unwrap()
                })
                .filter(|v| v.is_alphabetic())
                .rev()
                .collect::<Vec<char>>();
            let key = idx + 1;
            hash_table.insert(key, table);
        });

    hash_table
}

#[derive(Debug)]
struct Procedure {
    qty: usize,
    from: usize,
    to: usize,
}

impl Procedure {
    fn parse(line: &str) -> Procedure {
        let procedure = line
            .split_whitespace()
            .filter(|&word| word.parse::<u32>().is_ok())
            .map(|char| char.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let qty = procedure[0];
        let from = procedure[1];
        let to = procedure[2];

        Procedure { qty, from, to }
    }
}

fn main() {
    // let filename = "example.txt";
    let filename = "input.txt";
    let input = fs::read_to_string(filename).unwrap();

    let column_line = input
        .lines()
        .enumerate()
        .find(|&(_, line)| line.starts_with(" 1"))
        .unwrap().0;

    let mut table = get_table(column_line, &input);

    input
        .lines()
        .skip(column_line + 1)
        .filter(|line| !line.is_empty())
        .map(|line| Procedure::parse(line))
        .for_each(|procedure| {
            for _ in 0..procedure.qty {
                let stack = table.get_mut(&procedure.from).unwrap();
                let item = stack.pop().unwrap();
                let stack = table.get_mut(&procedure.to).unwrap();
                stack.push(item);
            }
        });

    let output = (1..=table.len())
        .map(|key| table.get(&key).unwrap().last().unwrap())
        .collect::<String>();

    println!("output: {:?}", output);
}
