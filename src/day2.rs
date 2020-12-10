use crate::infra::Problem;

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

fn parse<'a>(line: &'a str) -> (usize, usize, char, &'a str) {
    let mut x = line.splitn(2, "-");
    let r1 = x.next().unwrap().parse().unwrap();
    let mut x = x.next().unwrap().splitn(2, " ");
    let r2 = x.next().unwrap().parse().unwrap();
    let mut x = x.next().unwrap().splitn(2, ": ");
    let r3 = x.next().unwrap().parse().unwrap();
    let r4 = x.next().unwrap();
    (r1, r2, r3, r4)
}
