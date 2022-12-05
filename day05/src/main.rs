use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];

    let (stack, instructions) = read_input(filename);

    let stack = to_stacks(stack);
    let instructions = to_instructions(instructions);

    let stack = exec(stack, instructions);

    println!("{}", read_top(stack));
}

fn read_top(stacks: Vec<Vec<char>>) -> String {
    let mut result: Vec<char> = Vec::new();

    for stack in stacks {
        if stack.len() > 0 {
            result.push(stack[stack.len() - 1])
        }
    }

    result.iter().collect()
}

fn exec(mut stacks: Vec<Vec<char>>, moves: Vec<Move>) -> Vec<Vec<char>> {
    for m in moves {
        let mut to_push: Vec<char> = Vec::new();
        for _ in 0..m.count {
            // part 1
            // let to_push = stacks[m.from].pop().unwrap();
            // stacks[m.to].push(to_push);

            // part 2
            to_push.push(stacks[m.from].pop().unwrap());
        }
        // part 2
        stacks[m.to].extend(to_push.into_iter().rev())
    }

    stacks
}

#[derive(Clone, Copy, Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        let vec = s
            .split_whitespace()
            .filter(|sub| !sub.is_empty())
            .filter_map(|sub| sub.parse::<usize>().ok())
            .collect::<Vec<usize>>();
        Self {
            count: vec[0],
            from: vec[1] - 1,
            to: vec[2] - 1,
        }
    }
}

fn to_instructions(s: String) -> Vec<Move> {
    s.split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| Move::from(l))
        .collect()
}

fn to_stacks(s: String) -> Vec<Vec<char>> {
    let count = s
        .split("\n")
        .filter(|l| !l.is_empty() && l.contains("1"))
        .next()
        .unwrap()
        .rsplit(" ")
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<usize>().unwrap())
        .next()
        .unwrap();

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); count];

    for row in s.split("\n").filter(|l| !l.is_empty()) {
        for (index, chunk) in row.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            if chunk.contains(&'[') {
                stacks[index].push(chunk[1]);
            }
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    stacks
}

fn read_input(filename: &str) -> (String, String) {
    let contents = fs::read_to_string(filename).expect(&format!(
        "Something went wrong reading the file: {}",
        filename
    ));
    let mut iter = contents
        .split("\n\n")
        .filter(|l| !l.is_empty())
        .map(|s| s.to_string());
    (iter.next().unwrap(), iter.next().unwrap())
}
