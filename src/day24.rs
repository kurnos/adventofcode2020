use crate::infra::Problem;
use std::collections::{HashMap, HashSet};

pub struct Day24;

impl Problem<String, String, usize, usize, 24> for Day24 {
    fn first(contents: String) -> usize {
        make_board(contents).len()
    }

    fn second(contents: String) -> usize {
        let mut board = make_board(contents);
        let mut counts = HashMap::new();
        for _ in 0..100 {
            for &(x, y) in board.iter() {
                *counts.entry((x, y)).or_insert(0u8) |= 8;
                *counts.entry((x + 1, y)).or_insert(0) += 1;
                *counts.entry((x + 1, y - 1)).or_insert(0) += 1;
                *counts.entry((x, y + 1)).or_insert(0) += 1;
                *counts.entry((x, y - 1)).or_insert(0) += 1;
                *counts.entry((x - 1, y + 1)).or_insert(0) += 1;
                *counts.entry((x - 1, y)).or_insert(0) += 1;
            }

            counts.iter().for_each(|(p, c)| {
                let is_black = (c & 8) != 0;
                let count = c & 7;
                if is_black && (count == 0 || count > 2) {
                    board.remove(p);
                } else if *c == 2 {
                    board.insert(*p);
                }
            });
            counts.clear();
        }
        board.len()
    }
}

fn make_board(contents: String) -> HashSet<(i8, i8)> {
    let mut res = HashSet::new();
    for pos in contents.lines().map(parse_line) {
        if res.contains(&pos) {
            res.remove(&pos);
        } else {
            res.insert(pos);
        }
    }
    res
}

fn parse_line(line: &str) -> (i8, i8) {
    let mut pos = (0, 0);
    let mut prev = 0;
    for b in line.bytes() {
        match (prev, b) {
            (b's', b'e') => pos.1 += 1,
            (b's', b'w') => pos = (pos.0 - 1, pos.1 + 1),
            (b'n', b'e') => pos = (pos.0 + 1, pos.1 - 1),
            (b'n', b'w') => pos.1 -= 1,
            (_, b'e') => pos.0 += 1,
            (_, b'w') => pos.0 -= 1,
            _ => {}
        }
        prev = b;
    }
    pos
}
