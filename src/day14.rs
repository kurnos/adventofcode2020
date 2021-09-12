use crate::infra::Problem;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Day14;

impl Problem<String, String, u64, u64, 14> for Day14 {
    fn first(contents: String) -> u64 {
        let data = contents
            .split("mask = ")
            .skip(1)
            .map(|x| {
                let mut y = x.lines();
                let m = y.next().unwrap();
                let one_m = m.bytes().fold(0, |a, b| a << 1 | ((b == b'1') as u64));
                let zero_m = m.bytes().fold(0, |a, b| a << 1 | ((b != b'0') as u64));
                let xs: Vec<(u32, u64)> = y
                    .map(|x| {
                        let (a, b) = x["mem[".len()..].splitn(2, "] = ").collect_tuple().unwrap();
                        (a.parse().unwrap(), b.parse().unwrap())
                    })
                    .collect_vec();
                (one_m, zero_m, xs)
            })
            .collect_vec();

        let mut mem: HashMap<u32, u64> = HashMap::new();
        for cmd in data {
            for (adr, val) in cmd.2 {
                mem.insert(adr, val & cmd.1 | cmd.0);
            }
        }
        mem.values().sum::<u64>()
    }

    fn second(contents: String) -> u64 {
        let data = contents
            .split("mask = ")
            .skip(1)
            .map(|x| {
                let mut y = x.lines();
                let m = y.next().unwrap();
                let one_m = m.bytes().fold(0, |a, b| a << 1 | ((b == b'1') as u64));
                let floating = m
                    .bytes()
                    .rev()
                    .enumerate()
                    .filter(|b| b.1 == b'X')
                    .map(|x| x.0)
                    .collect_vec();
                let xs: Vec<(u64, u64)> = y
                    .map(|x| {
                        let (a, b) = x["mem[".len()..].splitn(2, "] = ").collect_tuple().unwrap();
                        (a.parse().unwrap(), b.parse().unwrap())
                    })
                    .collect_vec();
                (one_m, floating, xs)
            })
            .collect_vec();

        let mut mem: HashMap<u64, u64> = HashMap::new();
        for (one_m, floats, cmds) in data {
            let mut masks = vec![(0u64, 0u64)];
            for a in &floats {
                let mut new_masks = vec![];
                for m in masks {
                    new_masks.push((m.0 | 1 << a, m.1));
                    new_masks.push((m.0, m.1 | 1 << a));
                }
                masks = new_masks;
            }
            for m in masks {
                for &(addr, value) in &cmds {
                    mem.insert((addr | one_m | m.0) & !m.1, value);
                }
            }
        }

        mem.values().sum::<u64>()
    }
}
