use std::{collections::HashSet, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let marker_size: usize = args[2].parse().unwrap();

    for line in read(filename) {
        if let Some(result) = marker_start(line, marker_size) {
            println!("{}", result);
        } else {
            println!("no result")
        }
    }
}

fn marker_start(signal: String, size: usize) -> Option<usize> {
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
        return Some(index + size);
    }

    None
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

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
    const MARKER_SIZE: usize = 4;
    const MESSAGE_SIZE: usize = 14;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn marker_starts_correctly(#[case] signal: impl Into<String>, #[case] expected: usize) {
        let result = marker_start(signal.into(), MARKER_SIZE);
        assert_eq!(expected, result.unwrap())
    }

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn message_starts_correctly(#[case] signal: impl Into<String>, #[case] expected: usize) {
        let result = marker_start(signal.into(), MESSAGE_SIZE);
        assert_eq!(expected, result.unwrap())
    }
}
