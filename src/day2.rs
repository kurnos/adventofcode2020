use crate::infra::Problem;
use text_io::scan;

pub struct Day2;

impl Problem<String, String, usize, usize> for Day2 {
    fn day() -> u8 {
        2
    }
    fn first(contents: String) -> usize {
        contents
            .lines()
            .map(|line| {
                let (min, max, letter, password) = parse(line);
                let count = password.matches(letter).count();
                min <= count && count <= max
            })
            .filter(|a| *a)
            .count()
    }
    fn second(contents: String) -> usize {
        contents
            .lines()
            .map(|line| {
                let (min, max, letter, password) = parse(line);
                (password.chars().nth(min - 1) == Some(letter))
                    ^ (password.chars().nth(max - 1) == Some(letter))
            })
            .filter(|a| *a)
            .count()
    }
}

fn parse(line: &str) -> (usize, usize, char, String) {
    let (min, max, letter, password): (usize, usize, char, String);
    scan!(line.bytes() => "{}-{} {}: {}", min, max, letter, password);
    (min, max, letter, password)
}
