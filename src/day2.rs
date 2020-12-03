use crate::infra::Problem;
use crate::parse;

pub struct Day2;

impl Problem<String, String, usize, usize> for Day2 {
    fn day() -> u8 {
        2
    }
    fn first(contents: String) -> usize {
        contents
            .lines()
            .map(|line| {
                let (min, max, letter, password): (usize, usize, char, String) =
                    parse!(r"(.+)-(.+) (.): (.+)", line, 4);
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
                let (min, max, letter, password): (usize, usize, char, String) =
                    parse!(r"(.+)-(.+) (.): (.+)", line, 4);
                (password.chars().nth(min - 1) == Some(letter))
                    ^ (password.chars().nth(max - 1) == Some(letter))
            })
            .filter(|a| *a)
            .count()
    }
}
