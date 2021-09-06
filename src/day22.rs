use itertools::Itertools;
use std::collections::VecDeque;

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

        let mut pool = Vec::new();
        score(run_recursive(&mut pool, d1, d2))
    }
}

type DequePool = Vec<VecDeque<u8>>;

fn get_copy(pool: &mut DequePool, src: &VecDeque<u8>, len: u8) -> VecDeque<u8> {
    if let Some(mut result) = pool.pop() {
        result.clear();
        result.extend(src.iter().take(len as usize).cloned());
        result
    } else {
        src.iter().take(len as usize).cloned().collect()
    }
}

fn return_deque(pool: &mut DequePool, deque: VecDeque<u8>) {
    pool.push(deque);
}

#[derive(Debug, Clone)]
enum W {
    P1(VecDeque<u8>),
    P2(VecDeque<u8>),
}

fn run_recursive(pool: &mut DequePool, mut d1: VecDeque<u8>, mut d2: VecDeque<u8>) -> W {
    for i in 0.. {
        if i > 1000 {
            return_deque(pool, d2);
            return W::P1(d1);
        } else {
            let (c1, c2) = (d1.pop_front().unwrap(), d2.pop_front().unwrap());
            if (c1 as usize) <= d1.len() && (c2 as usize) <= d2.len() {
                let r1 = get_copy(pool, &d1, c1);
                let r2 = get_copy(pool, &d2, c2);
                match run_recursive(pool, r1, r2) {
                    W::P1(d) => {
                        return_deque(pool, d);
                        d1.push_back(c1);
                        d1.push_back(c2);
                    }
                    W::P2(d) => {
                        return_deque(pool, d);
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
                return_deque(pool, d1);
                return W::P2(d2);
            } else if d2.is_empty() {
                return_deque(pool, d2);
                return W::P1(d1);
            }
        }
    }
    panic!()
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
