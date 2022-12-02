use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let strategy = read_strategy(filename);
    let with_outcome = read_strategy_with_outcome(filename);

    println!(
        "{}",
        strategy
            .iter()
            .map(|round| round_score(*round))
            .sum::<u64>()
    );

    println!(
        "{}",
        with_outcome
            .iter()
            .map(|round| outcome_into_figures(*round))
            .map(|round| round_score(round))
            .sum::<u64>()
    )
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

#[derive(Copy, Clone, Debug, PartialEq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl From<&str> for Outcome {
    fn from(s: &str) -> Self {
        match s {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => Outcome::Lose,
        }
    }
}

fn outcome_into_figures(round: (Figure, Outcome)) -> (Figure, Figure) {
    match round {
        (x, Outcome::Draw) => (x, x),
        (x, Outcome::Win) => match x {
            Figure::Rock => (x, Figure::Paper),
            Figure::Paper => (x, Figure::Scissors),
            Figure::Scissors => (x, Figure::Rock),
        },
        (x, Outcome::Lose) => match x {
            Figure::Rock => (x, Figure::Scissors),
            Figure::Paper => (x, Figure::Rock),
            Figure::Scissors => (x, Figure::Paper),
        },
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
fn read_strategy_with_outcome(filename: &str) -> Vec<(Figure, Outcome)> {
    let contents = fs::read_to_string(filename).expect(&format!(
        "Something went wrong reading the file: {}",
        filename
    ));
    contents
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|row| {
            let row = row.split_whitespace().collect::<Vec<&str>>();
            (Figure::from(row[0]), Outcome::from(row[1]))
        })
        .collect()
}
