use std::iter::FromIterator;
use std::{env, fs};

use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];

    let input_lines = read_input(filename);

    let compartments = input_lines
        .iter()
        .map(|l| into_compartments(l))
        .collect::<Vec<(HashSet<char>, HashSet<char>)>>();
    let duplicates = compartments
        .iter()
        .map(|(f, s)| f.intersection(s).next().unwrap().clone())
        .collect::<Vec<char>>();
    let score: u32 = duplicates.iter().map(|&c| score_item(c)).sum();

    println!("{}", score);

    let badges = input_lines
        .chunks(3)
        .map(|v| {
            let first: HashSet<char> = HashSet::from_iter(v[0].chars());
            let second: HashSet<char> = HashSet::from_iter(v[1].chars());
            let third: HashSet<char> = HashSet::from_iter(v[2].chars());
            first
                .intersection(&second)
                .map(|c| c.clone())
                .collect::<HashSet<char>>()
                .intersection(&third)
                .next()
                .unwrap()
                .clone()
        })
        .collect::<Vec<char>>();
    let score = badges.iter().map(|&c| score_item(c)).sum::<u32>();
    println!("{}", score)
}

fn score_item(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        return (item as u32) - ('a' as u32) + 1;
    }

    (item as u32) - ('A' as u32) + 27
}

fn into_compartments(items: &String) -> (HashSet<char>, HashSet<char>) {
    let (first, second) = items.split_at(items.len() / 2);

    (
        HashSet::from_iter(first.chars()),
        HashSet::from_iter(second.chars()),
    )
}

fn read_input(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect(&format!(
        "Something went wrong reading the file: {}",
        filename
    ));
    contents.split_whitespace().map(|l| l.to_string()).collect()
}
