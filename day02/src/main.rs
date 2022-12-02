use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let strategy = read_strategy(filename);

    println!(
        "{}",
        strategy
            .iter()
            .map(|round| round_score(*round))
            .sum::<u64>()
    );
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Figure {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Figure {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Figure::Rock,
            "B" | "Y" => Figure::Paper,
            "C" | "Z" => Figure::Scissors,
            _ => Figure::Rock,
        }
    }
}

fn figure_score(fig: Figure) -> u64 {
    match fig {
        Figure::Rock => 1,
        Figure::Paper => 2,
        Figure::Scissors => 3,
    }
}

fn round_score(round: (Figure, Figure)) -> u64 {
    match round {
        (x, y) if x == y => 3 + figure_score(x),
        (Figure::Rock, fig @ Figure::Paper)
        | (Figure::Scissors, fig @ Figure::Rock)
        | (Figure::Paper, fig @ Figure::Scissors) => 6 + figure_score(fig),
        (_, fig) => 0 + figure_score(fig),
    }
}

fn read_strategy(filename: &str) -> Vec<(Figure, Figure)> {
    let contents = fs::read_to_string(filename).expect(&format!(
        "Something went wrong reading the file: {}",
        filename
    ));
    contents
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|row| {
            let row = row
                .split_whitespace()
                .map(|s| Figure::from(s))
                .collect::<Vec<Figure>>();
            (row[0], row[1])
        })
        .collect()
}
