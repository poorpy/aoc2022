use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];

    let ranges = read_ranges(filename);
    let overlap_count: usize = ranges
        .clone()
        .into_iter()
        .map(|(first, second)| is_full_overlap(first, second))
        .filter(|&b| b)
        .count();

    println!("{}", overlap_count);

    let overlap_count: usize = ranges
        .into_iter()
        .map(|(first, second)| is_overlap(first, second))
        .filter(|&b| b)
        .count();

    println!("{}", overlap_count);
}

type Pair = (u64, u64);

fn is_full_overlap(first: Pair, second: Pair) -> bool {
    first.0 >= second.0 && first.1 <= second.1 || second.0 >= first.0 && second.1 <= first.1
}

fn is_overlap(first: Pair, second: Pair) -> bool {
    first.0 <= second.1 && second.0 <= first.1
}

fn read_ranges(filename: &str) -> Vec<(Pair, Pair)> {
    let contents = fs::read_to_string(filename).expect(&format!(
        "Something went wrong reading the file: {}",
        filename
    ));
    fn to_pair(s: &str) -> (u64, u64) {
        let pair = s.split("-").collect::<Vec<&str>>();
        (pair[0].parse().unwrap(), pair[1].parse().unwrap())
    }
    contents
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut pairs = l.split(",").map(|s| to_pair(s));
            (pairs.next().unwrap(), pairs.next().unwrap())
        })
        .collect()
}
