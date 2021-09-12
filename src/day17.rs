use crate::infra::Problem;
use itertools::iproduct;
use std::collections::{HashMap, HashSet};

pub struct Day17;

impl Problem<String, String, usize, usize, 17> for Day17 {
    fn first(contents: String) -> usize {
        let mut data: HashSet<[i8; 3]> = parse(&contents)
            .map(|(x, y)| [x as i8, y as i8, 0])
            .collect();

        let mut weights = HashMap::new();
        for _ in 0..6 {
            for p in data.iter() {
                for n in ns3(*p) {
                    *weights.entry(*p).or_insert(0u8) |= 128;
                    *weights.entry(n).or_insert(0u8) += 1;
                }
            }
            for (p, m) in weights.iter() {
                let a = (m >> 7) != 0;
                let c = m & 127;
                if a && !(c == 2 || c == 3) {
                    data.remove(p);
                } else if !a && c == 3 {
                    data.insert(*p);
                }
            }
            weights.clear();
        }
        data.len()
    }

    fn second(contents: String) -> usize {
        let mut data: HashSet<[i8; 4]> = parse(&contents)
            .map(|(x, y)| [x as i8, y as i8, 0, 0])
            .collect();

        let mut weights = HashMap::new();
        for _ in 0..6 {
            for p in data.iter() {
                for n in ns4(*p) {
                    *weights.entry(*p).or_insert(0u8) |= 128;
                    *weights.entry(n).or_insert(0u8) += 1;
                }
            }
            for (p, m) in weights.iter() {
                let a = (m & 128) != 0;
                let c = m & 127;
                if a && !(c == 2 || c == 3) {
                    data.remove(p);
                } else if *m == 3 {
                    data.insert(*p);
                }
            }
            weights.clear();
        }
        data.len()
    }
}

fn parse(contents: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
    contents
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.bytes().enumerate().map(move |(x, b)| ((x, y), b)))
        .filter_map(|(p, b)| if b == b'#' { Some(p) } else { None })
}

fn ns3(p: [i8; 3]) -> impl Iterator<Item = [i8; 3]> {
    let x = &[0, 1, -1];
    iproduct!(x, x, x)
        .skip(1)
        .map(move |(&x, &y, &z)| [p[0] + x, p[1] + y, p[2] + z])
}

fn ns4(p: [i8; 4]) -> impl Iterator<Item = [i8; 4]> {
    let x = &[0i8, 1, -1];
    iproduct!(x, x, x, x)
        .skip(1)
        .map(move |(&x, &y, &z, &w)| [p[0] + x, p[1] + y, p[2] + z, p[3] + w])
}
