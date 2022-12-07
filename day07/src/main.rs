use std::{env, fs};

use anyhow::Result;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];

    let mut lines = read(filename);

    let mut current = Directory {
        directories: Vec::new(),
        name: "".to_string(),
        size: 0,
    };

    while let Some(line) = lines.next() {
        if line.starts_with("$ cd") {
            current.name = line.split_whitespace().rev().next().unwrap().to_string();
        }
        if line == "$ ls" {
            contents(&mut current, &mut lines)
        }
    }
}

fn contents(dir: &mut Directory, lines: &mut impl Iterator<Item = String>) {
    while let Some(line) = lines.next() {
        if line.starts_with("dir") {
            dir.directories.push(Directory {
                directories: Vec::new(),
                name: line.split_whitespace().rev().next().unwrap().to_string(),
                size: 0,
            })
        }
        if line.as_bytes()[0].is_ascii_digit() {
            dir.size += line
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap()
        }
    }
}

// struct File {
//     name: String,
//     size: usize,
// }

struct Directory {
    directories: Vec<Directory>,
    name: String,
    size: usize,
}

fn read(filename: &str) -> impl Iterator<Item = String> {
    let contents = fs::read_to_string(filename).expect(&format!(
        "Something went wrong reading the file: {}",
        filename
    ));

    contents
        .lines()
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect::<Vec<String>>()
        .into_iter()
}
