use crate::infra::Problem;
use std::collections::HashMap;

pub struct Day11;

type Pos = (usize, usize);
type Map = HashMap<Pos, S>;

impl Problem<String, String, usize, usize> for Day11 {
    fn day() -> u8 {
        11
    }

    fn first(contents: String) -> usize {
        asdf(parse(&contents), 4, |map, pos| {
            let mut res = 0;
            for &dx in &[-1isize, 0, 1] {
                for &dy in &[-1isize, 0, 1] {
                    if (dx, dy) != (0, 0) {
                        let mut p = pos.clone();
                        p.0 = ((p.0 as isize) + dx) as usize;
                        p.1 = ((p.1 as isize) + dy) as usize;
                        if let Some(S::Occupied) = map.get(&p) {
                            res += 1;
                        }
                    }
                }
            }
            res
        })
    }

    fn second(contents: String) -> usize {
        asdf(parse(&contents), 5, |map, pos| {
            let mut res = 0;
            for &dx in &[-1isize, 0, 1] {
                for &dy in &[-1isize, 0, 1] {
                    if (dx, dy) != (0, 0) {
                        let mut p = pos.clone();
                        loop {
                            p.0 = ((p.0 as isize) + dx) as usize;
                            p.1 = ((p.1 as isize) + dy) as usize;
                            match map.get(&p) {
                                None => {
                                    break;
                                }
                                Some(S::Occupied) => {
                                    res += 1;
                                    break;
                                }
                                Some(S::Empty) => {
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            res
        })
    }
}

fn asdf(mut map: Map, limit: u8, f: impl Fn(&Map, Pos) -> u8) -> usize {
    let mp = (
        map.keys().map(|x| x.0).max().unwrap(),
        map.keys().map(|x| x.1).max().unwrap(),
    );

    let mut changed = true;
    while changed {
        let mut next_map = Map::new();
        changed = false;
        for i in 1..=mp.0 {
            for j in 1..=mp.1 {
                let pos = (i, j);
                next_map.insert(
                    pos,
                    match (map.get(&pos).unwrap(), f(&map, pos)) {
                        (S::Empty, 0) => {
                            changed = true;
                            S::Occupied
                        }
                        (S::Occupied, n) if n >= limit => {
                            changed = true;
                            S::Empty
                        }
                        (t, _) => *t,
                    },
                );
            }
        }
        map = next_map;
    }
    map.values()
        .filter(|x| if let S::Occupied = x { true } else { false })
        .count()
}

#[derive(Debug, Copy, Clone)]
enum S {
    Floor,
    Empty,
    Occupied,
}

fn parse(contents: &str) -> Map {
    let res = contents
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().map(move |(i, c)| {
                (
                    (row + 1, i + 1),
                    match c {
                        '.' => S::Floor,
                        'L' => S::Empty,
                        _ => panic!(),
                    },
                )
            })
        })
        .collect();
    res
}
