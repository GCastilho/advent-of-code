use lcmx::lcmx;
use std::{collections::HashMap, fs, str::FromStr};
use strum_macros::EnumString;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let map = input.parse::<Map>().unwrap();

    let steps = map.walk_to_zzz();
    println!("Part one, required steps are {}", steps.len());

    let steps = map.walk_to_z_par();
    println!("Part two, required steps are {steps}");
}

#[derive(Debug, PartialEq, EnumString)]
enum Instruction {
    #[strum(serialize = "L")]
    Left,
    #[strum(serialize = "R")]
    Right,
}

#[derive(Debug, PartialEq, Clone)]
struct Node {
    left: String,
    right: String,
}

#[cfg(test)]
impl Node {
    fn new(left: &str, right: &str) -> Self {
        Self {
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}

impl FromStr for Node {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let left = words.next().unwrap()[1..4].to_string();
        let right = words.next().unwrap()[0..3].to_string();
        Ok(Self { left, right })
    }
}

#[derive(Debug, PartialEq)]
struct Map {
    instructions: Vec<Instruction>,
    nodes: HashMap<String, Node>,
}

impl Map {
    fn walk_to_zzz(&self) -> Vec<(String, Node)> {
        let mut steps = vec![];

        let mut current_node = self.nodes.get("AAA").unwrap();
        for instruction in self.instructions.iter().cycle() {
            let next_node = match instruction {
                Instruction::Left => &current_node.left,
                Instruction::Right => &current_node.right,
            };
            current_node = self.nodes.get(next_node).unwrap();
            steps.push((next_node.clone(), current_node.clone()));
            if next_node == "ZZZ" {
                break;
            }
        }

        steps
    }

    fn walk(&self, from: &str, instruction: &Instruction) -> &String {
        let current_node = self.nodes.get(from).unwrap();
        match instruction {
            Instruction::Left => &current_node.left,
            Instruction::Right => &current_node.right,
        }
    }

    fn walk_to_z_par(&self) -> u64 {
        let steps = self
            .nodes
            .keys()
            .filter(|key| key.ends_with('A'))
            .collect::<Vec<_>>()
            .iter()
            .map(|&node| {
                let mut steps = 0;

                let mut next_node = node;
                for instruction in self.instructions.iter().cycle() {
                    steps += 1;
                    next_node = self.walk(next_node, instruction);
                    if next_node.ends_with('Z') {
                        break;
                    }
                }

                steps
            })
            .collect::<Vec<_>>();
        lcmx(&steps).unwrap()
    }
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let instructions = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        let nodes = lines
            .skip(1)
            .map(|line| {
                let el = line[0..3].to_string();
                let node = line[6..].parse().unwrap();
                (el, node)
            })
            .collect();

        Ok(Self {
            instructions,
            nodes,
        })
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, fs};

    use crate::{Instruction, Map, Node};

    fn get_map() -> Map {
        fs::read_to_string("./example.txt")
            .unwrap()
            .parse::<Map>()
            .unwrap()
    }

    #[test]
    fn parse() {
        let map = get_map();
        assert_eq!(
            map,
            Map {
                instructions: vec![Instruction::Left, Instruction::Right],
                nodes: HashMap::from([
                    ("11A".into(), Node::new("11B", "XXX")),
                    ("11B".into(), Node::new("XXX", "11Z")),
                    ("11Z".into(), Node::new("11B", "XXX")),
                    ("22A".into(), Node::new("22B", "XXX")),
                    ("22B".into(), Node::new("22C", "22C")),
                    ("22C".into(), Node::new("22Z", "22Z")),
                    ("22Z".into(), Node::new("22B", "22B")),
                    ("XXX".into(), Node::new("XXX", "XXX")),
                ])
            }
        )
    }

    #[test]
    fn walk_par() {
        let map = get_map();
        let steps = map.walk_to_z_par();
        assert_eq!(steps, 6);
    }
}
