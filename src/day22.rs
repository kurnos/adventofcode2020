use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub struct Day22;

impl crate::infra::Problem<String, String, u32, u32> for Day22 {
    fn day() -> u8 {
        22
    }

    fn first(contents: String) -> u32 {
        let data = parse(&contents);
        let (d1, d2) = data;

        score(normal(d1, d2))
    }

    fn second(contents: String) -> u32 {
        let data = parse(&contents);
        let (d1, d2) = data;

        score(run_recursive(d1, d2))
    }
}

#[derive(Debug, Clone)]
enum W {
    P1(VecDeque<u8>),
    P2(VecDeque<u8>),
}

fn run_recursive(mut d1: VecDeque<u8>, mut d2: VecDeque<u8>) -> W {
    let mut seen = HashSet::new();
    loop {
        if !seen.insert((d1.clone(), d2.clone())) {
            return W::P1(d1);
        } else {
            let (c1, c2) = (d1.pop_front().unwrap(), d2.pop_front().unwrap());
            if (c1 as usize) <= d1.len() && (c2 as usize) <= d2.len() {
                match run_recursive(
                    d1.iter().take(c1 as usize).cloned().collect(),
                    d2.iter().take(c2 as usize).cloned().collect(),
                ) {
                    W::P1(_) => {
                        d1.push_back(c1);
                        d1.push_back(c2);
                    }
                    W::P2(_) => {
                        d2.push_back(c2);
                        d2.push_back(c1);
                    }
                }
            } else if c1 < c2 {
                d2.push_back(c2);
                d2.push_back(c1);
            } else {
                d1.push_back(c1);
                d1.push_back(c2);
            }

            if d1.is_empty() {
                return W::P2(d2);
            } else if d2.is_empty() {
                return W::P1(d1);
            }
        }
    }
}

fn score(winner: W) -> u32 {
    match winner {
        W::P1(d) | W::P2(d) => d
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, n)| (i as u32 + 1) * n as u32)
            .sum(),
    }
}

fn normal(mut d1: VecDeque<u8>, mut d2: VecDeque<u8>) -> W {
    loop {
        let (c1, c2) = (d1.pop_front().unwrap(), d2.pop_front().unwrap());
        if c1 < c2 {
            d2.push_back(c2);
            d2.push_back(c1);
        } else {
            d1.push_back(c1);
            d1.push_back(c2);
        }

        if d1.is_empty() {
            return W::P2(d2);
        } else if d2.is_empty() {
            return W::P1(d1);
        }
    }
}

fn parse(s: &str) -> (VecDeque<u8>, VecDeque<u8>) {
    s.split("\n\n")
        .map(|x| x.lines().skip(1).map(|n| n.parse().unwrap()).collect())
        .collect_tuple()
        .unwrap()
}
