use std::{collections::HashSet, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];

    for line in read(filename) {
        println!("{}", marker_start(line, 14));
    }
}

fn marker_start(signal: String, size: usize) -> usize {
    if let Some((index, _)) = signal
        .as_bytes()
        .windows(size)
        .enumerate()
        .map(|(i, w)| {
            let mut uinq = HashSet::new();
            (i, w.into_iter().all(move |v| uinq.insert(v)))
        })
        .filter(|(_, unique)| *unique)
        .next()
    {
        return index + size;
    }

    0
}

fn read(filename: &str) -> impl Iterator<Item = String> {
    let contents = fs::read_to_string(filename).expect(&format!(
        "Something went wrong reading the file: {}",
        filename
    ));

    contents
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| String::from(l))
        .collect::<Vec<String>>()
        .into_iter()
}
