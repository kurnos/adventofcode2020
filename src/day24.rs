use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::infra::Problem;

pub struct Day24;

impl Problem<String, String, usize, usize> for Day24 {
    fn day() -> u8 {
        24
    }

    fn first(contents: String) -> usize {
        let data = contents
            .lines()
            .map(|x| parse_line(x.as_bytes()))
            .collect_vec();

        let mut grid = HashMap::new();
        for pattern in data {
            let mut pos = (0, 0, 0);
            for n in pattern {
                pos = match n {
                    N::E => (pos.0 + 1, pos.1, pos.2 - 1),
                    N::SE => (pos.0, pos.1 + 1, pos.2 - 1),
                    N::SW => (pos.0 - 1, pos.1 + 1, pos.2),
                    N::W => (pos.0 - 1, pos.1, pos.2 + 1),
                    N::NW => (pos.0, pos.1 - 1, pos.2 + 1),
                    N::NE => (pos.0 + 1, pos.1 - 1, pos.2),
                }
            }
            *grid.entry(pos).or_insert(false) ^= true;
        }

        let board = grid
            .into_iter()
            .filter_map(|(p, b)| if b { Some(p) } else { None })
            .collect::<HashSet<_>>();
        board.len()
    }

    fn second(contents: String) -> usize {
        let data = contents
            .lines()
            .map(|x| parse_line(x.as_bytes()))
            .collect_vec();

        let mut grid = HashMap::new();
        for pattern in data {
            let mut pos = (0i8, 0i8, 0i8);
            for n in pattern {
                pos = match n {
                    N::E => (pos.0 + 1, pos.1, pos.2 - 1),
                    N::SE => (pos.0, pos.1 + 1, pos.2 - 1),
                    N::SW => (pos.0 - 1, pos.1 + 1, pos.2),
                    N::W => (pos.0 - 1, pos.1, pos.2 + 1),
                    N::NW => (pos.0, pos.1 - 1, pos.2 + 1),
                    N::NE => (pos.0 + 1, pos.1 - 1, pos.2),
                }
            }
            *grid.entry(pos).or_insert(false) ^= true;
        }

        let mut board = grid
            .into_iter()
            .filter_map(|(p, b)| if b { Some(p) } else { None })
            .collect::<HashSet<_>>();

        for _ in 0..100 {
            board = evolve(&board);
        }
        board.len()
    }
}

fn evolve(board: &HashSet<(i8, i8, i8)>) -> HashSet<(i8, i8, i8)> {
    let mut counts = HashMap::new();
    for &(x, y, z) in board {
        *counts.entry((x, y, z)).or_insert(0u8) |= 8;
        for &(dx, dy, dz) in [
            (1, 0, -1),
            (1, -1, 0),
            (0, 1, -1),
            (0, -1, 1),
            (-1, 1, 0),
            (-1, 0, 1),
        ]
        .iter()
        {
            *counts.entry((x + dx, y + dy, z + dz)).or_insert(0) += 1;
        }
    }

    counts
        .into_iter()
        .filter_map(|(p, c)| {
            let is_black = (c & 8) != 0;
            let count = c & 7;
            if is_black && (count == 0 || count > 2) {
                None
            } else if !is_black && (count == 2) {
                Some(p)
            } else if is_black {
                Some(p)
            } else {
                None
            }
        })
        .collect()
}

#[derive(Debug)]
enum N {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

fn parse_line(mut line: &[u8]) -> Vec<N> {
    let mut res = vec![];
    while !line.is_empty() {
        line = match line {
            [b'e', rest @ ..] => {
                res.push(N::E);
                rest
            }
            [b's', b'e', rest @ ..] => {
                res.push(N::SE);
                rest
            }
            [b's', b'w', rest @ ..] => {
                res.push(N::SW);
                rest
            }
            [b'n', b'w', rest @ ..] => {
                res.push(N::NW);
                rest
            }
            [b'n', b'e', rest @ ..] => {
                res.push(N::NE);
                rest
            }
            [b'w', rest @ ..] => {
                res.push(N::W);
                rest
            }
            s => panic!(String::from_utf8(s.to_vec())),
        }
    }
    res
}
