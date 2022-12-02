use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap();

    let mut x = input.split("\n\n")
        .map(|v| {
            v.split("\n")
                .filter_map(|v| v.parse::<u32>().ok())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    x.sort();

    let sum_biggest_tree = &x[x.len()-3..]
        .to_vec()
        .iter()
        .sum::<u32>();

    println!("Items: {:?};\nfirst_tree: {:?}", x, sum_biggest_tree);
}
