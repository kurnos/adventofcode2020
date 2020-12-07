use crate::infra::Problem;
use std::collections::{HashMap, HashSet};

pub struct Day7;

impl Problem<String, String, usize, u32> for Day7 {
    fn day() -> u8 {
        7
    }

    fn first(contents: String) -> usize {
        let x = parse_bags(&contents);

        let mut can_hold_gold = HashSet::new();
        can_hold_gold.insert("shiny gold");

        let mut changed = true;
        while changed {
            changed = false;
            for bag in &x {
                if bag.1.keys().any(|k| can_hold_gold.contains(k)) {
                    changed |= can_hold_gold.insert(bag.0);
                }
            }
        }
        can_hold_gold.len() - 1
    }

    fn second(contents: String) -> u32 {
        let x = parse_bags(&contents);

        let mut total_count = 0;
        let mut counts = HashMap::new();
        counts.insert("shiny gold", 1u32);

        while !counts.is_empty() {
            let mut new_counts: HashMap<&str, u32> = HashMap::new();
            for (b, c) in &counts {
                for (b2, c2) in x[b].iter() {
                    *new_counts.entry(b2).or_insert(0) += c * c2;
                }
            }
            total_count += new_counts.values().sum::<u32>();
            counts = new_counts;
        }
        total_count
    }
}

fn parse_bags(contents: &str) -> HashMap<&str, HashMap<&str, u32>> {
    contents
        .lines()
        .map(|line| {
            let mut x = line.splitn(2, " contain ");
            (
                x.next().unwrap().rsplitn(2, " ").nth(1).unwrap(),
                x.next()
                    .unwrap()
                    .split(", ")
                    .filter_map(|y| {
                        let mut d = y.splitn(2, ' ');
                        d.next()
                            .unwrap()
                            .parse::<u32>()
                            .ok()
                            .map(|n| (d.next().unwrap().rsplitn(2, " ").nth(1).unwrap(), n))
                    })
                    .collect(),
            )
        })
        .collect()
}
