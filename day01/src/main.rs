use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let calories = read_calories(filename);

    println!("max calories: {}", calories.iter().max().unwrap())
}

fn read_calories(filename: &str) -> Vec<u64> {
    let contents = fs::read_to_string(filename).expect(&format!(
        "Something went wrong reading the file: {}",
        filename
    ));
    contents
        .split("\n\n")
        .map(|x| {
            x.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .sum()
        })
        .collect()
}
