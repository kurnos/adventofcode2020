use crate::infra::{FromFile, InputProvider};
use crate::parse;

pub fn main() {
    first();
    second();
}

fn first() {
    let data = FromFile("day3.txt")
        .get_input()
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut res = 0u32;
    for i in 0..data.len() {
        if data[i][(i * 3) % data[i].len()] == '#' {
            res += 1;
        }
    }
    dbg!(res);
}

fn second() {
    let contents = FromFile("day3.txt").get_input();
    let data = contents
        .lines()
        .map(|x| x.as_bytes())
        .collect::<Vec<_>>();
    let mut prod = 1usize;
    for (down, right) in &[(1usize, 1usize), (1, 3), (1, 5), (1, 7), (2, 1)] {
        let mut i = 1usize;
        let mut res = 0;
        while i * down < data.len() {
            if data[i * down][(i * right) % data[i * down].len()] == b'#' {
                res += 1;
            }
            i += 1;
        }
        prod *= res;
    }

    dbg!(prod);
}
