use crate::infra::Problem;
use itertools::iproduct;
use nalgebra::{Point3, Point4};
use std::collections::{HashMap, HashSet};

pub struct Day17;

impl Problem<String, String, usize, usize> for Day17 {
    fn day() -> u8 {
        17
    }

    fn first(contents: String) -> usize {
        let mut data: HashSet<Point3<i8>> = parse(&contents)
            .map(|(x, y)| Point3::new(x as i8, y as i8, 0))
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
        let mut data: HashSet<Point4<i8>> = parse(&contents)
            .map(|(x, y)| Point4::new(x as i8, y as i8, 0, 0))
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
}

fn parse<'a>(contents: &'a str) -> impl Iterator<Item = (usize, usize)> + 'a {
    contents
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.bytes().enumerate().map(move |(x, b)| ((x, y), b)))
        .filter_map(|(p, b)| if b == b'#' { Some(p) } else { None })
}

fn ns3(p: Point3<i8>) -> impl Iterator<Item = Point3<i8>> {
    let x = &[0, 1, -1];
    iproduct!(x, x, x)
        .skip(1)
        .map(move |(&x, &y, &z)| Point3::new(p.x + x, p.y + y, p.z + z))
}

fn ns4(p: Point4<i8>) -> impl Iterator<Item = Point4<i8>> {
    let x = &[0i8, 1, -1];
    iproduct!(x, x, x, x)
        .skip(1)
        .map(move |(&x, &y, &z, &w)| Point4::new(p.x + x, p.y + y, p.z + z, p.w + w))
}
