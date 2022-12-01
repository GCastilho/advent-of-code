use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap();

    let x = input.split("\n\n")
        .map(|v| {
            v.split("\n")
                .filter_map(|v| v.parse::<u32>().ok())
                .reduce(|acc, cur| acc + cur)
                .unwrap()
        })
        .max()
        .unwrap();

    println!("Items: {:?}", x);
}
