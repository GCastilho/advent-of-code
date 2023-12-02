use std::{fs, mem};
use serde_json::Value::{self, Array, Number};

const INPUT: &str = "example.txt";

/**
 * Não está considerando tamanhos diferentes
 * Está retornando 1 valor, qdo tem q retornar um array de boolean pra cada par
 */
fn process_packets(packets: &Vec<(Value, Value)>) -> bool {
    for packet in packets {
        match packet {
            (Number(left), Number(right)) => {
                let left = left.as_i64().unwrap();
                let right = right.as_i64().unwrap();
                if left == right { continue; }
                return left > right;
            },
            (Array(left), Array(right)) => {
                let zipped = left.iter()
                    .zip(right.iter())
                    .map(|v| (v.0.clone(), v.1.clone()))
                    .collect::<Vec<_>>();
                let ordered = process_packets(&zipped);
                if !ordered { return false; }
            },
            (Array(left), Number(right)) => {
                let right = vec![serde_json::json!(right)];
                let zipped = left.iter()
                    .zip(right.iter())
                    .map(|v| (v.0.clone(), v.1.clone()))
                    .collect::<Vec<_>>();
                let ordered = process_packets(&zipped);
                if !ordered { return false; }
            },
            (Number(left), Array(right)) => {
                let left = vec![serde_json::json!(left)];
                let zipped = left.iter()
                    .zip(right.iter())
                    .map(|v| (v.0.clone(), v.1.clone()))
                    .collect::<Vec<_>>();
                let ordered = process_packets(&zipped);
                if !ordered { return false; }
            },
            _ => panic!("unexpected packet {packets:?}"),
        };
    }
    false
}

fn main() {
    let input = fs::read_to_string(INPUT).unwrap();

    let input = input.split_terminator("\n\n")
        .map(|packets| {
            let mut packets = packets
                .lines()
                .filter(|line| !line.is_empty())
                .map(|packet| serde_json::from_str::<Value>(packet).unwrap())
                .collect::<Vec<_>>();
            let packet1 = mem::take(&mut packets[0]);
            let packet2 = mem::take(&mut packets[1]);
            (packet1, packet2)
        })
        .collect::<Vec<_>>();

    let orderd = process_packets(&input);

    println!("orderd: {orderd}");
}
