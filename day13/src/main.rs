use std::process::exit;
use std::{env, fs};

use nom::character::complete::{multispace1, newline};
use nom::combinator::iterator;
use nom::multi::many0;
use nom::sequence::{pair, terminated};
use nom::IResult;
use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::separated_list0, sequence::delimited,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let contents = read(filename);

    // part 1
    let mut iter = iterator(contents.as_bytes(), signal_pair);
    let result: usize = iter
        .enumerate()
        .filter_map(|(index, (first, second))| (first < second).then_some(index + 1))
        .sum();

    println!("{}", result);

    // part 2
    let Ok((_, mut signals)) = signals(contents.as_bytes()) else {
        println!("failed to parse input for part 2");
        exit(1);
    };

    let (m1, m2) = (Signal::from(vec![vec![2]]), Signal::from(vec![vec![6]]));
    signals.push(m1.clone());
    signals.push(m2.clone());
    signals.sort();

    let p1 = signals.iter().position(|item| item == &m1).unwrap();
    let p2 = signals.iter().position(|item| item == &m2).unwrap();

    println!("{}", (p1 + 1) * (p2 + 1))
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Signal {
    Num(u32),
    List(Vec<Signal>),
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Signal::Num(first), Signal::Num(second)) => first.cmp(second),
            (Signal::List(first), Signal::List(second)) => first.cmp(second),
            (num @ Signal::Num(_), Signal::List(list)) => {
                [num.clone()].as_slice().cmp(list.as_slice())
            }
            (Signal::List(list), num @ Signal::Num(_)) => list.as_slice().cmp(&[num.clone()]),
        }
    }
}

impl From<u32> for Signal {
    fn from(val: u32) -> Self {
        Signal::Num(val)
    }
}

impl<T> From<Vec<T>> for Signal
where
    Signal: From<T>,
{
    fn from(vec: Vec<T>) -> Self {
        Signal::List(vec.into_iter().map(Signal::from).collect())
    }
}

fn signal(input: &[u8]) -> IResult<&[u8], Signal> {
    alt((
        map(nom::character::complete::u32, Signal::Num),
        map(
            delimited(tag("["), separated_list0(tag(","), signal), tag("]")),
            Signal::List,
        ),
    ))(input)
}

fn signal_pair(input: &[u8]) -> IResult<&[u8], (Signal, Signal)> {
    pair(terminated(signal, newline), terminated(signal, multispace1))(input)
}

fn signals(input: &[u8]) -> IResult<&[u8], Vec<Signal>> {
    many0(terminated(signal, multispace1))(input)
}

fn read(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect(&format!(
        "Something went wrong reading the file: {}",
        filename
    ));

    contents
}
