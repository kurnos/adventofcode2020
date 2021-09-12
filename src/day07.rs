use crate::infra::Problem;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day7;

impl Problem<String, String, usize, u32, 7> for Day7 {
    fn first(contents: String) -> usize {
        let mut containers_of: HashMap<&str, Vec<&str>> = HashMap::new();
        parse_bags(&contents, |container, _, contained| {
            containers_of
                .entry(contained)
                .or_insert_with(Vec::new)
                .push(container);
        });

        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back("shiny gold");
        while let Some(bag) = queue.pop_front() {
            if seen.insert(bag) {
                if let Some(containers) = containers_of.get(bag) {
                    containers.iter().for_each(|c| queue.push_back(c));
                }
            }
        }
        seen.len() - 1
    }

    fn second(contents: String) -> u32 {
        let mut bags = HashMap::new();

        parse_bags(&contents, |container, count, contained| {
            bags.entry(container)
                .or_insert_with(Vec::new)
                .push((contained, count));
        });

        let mut total_count = 0u32;
        let mut queue = vec![("shiny gold", 1u32)];

        while let Some((bag, count)) = queue.pop() {
            for &(contained, inner_count) in bags.get(&bag).unwrap_or(&vec![]) {
                total_count += count * inner_count;
                queue.push((contained, count * inner_count));
            }
        }
        total_count
    }
}

fn parse_bags<'a>(contents: &'a str, mut visitor: impl FnMut(&'a str, u32, &'a str)) {
    contents.lines().for_each(|line| {
        let mut x = line.splitn(2, " bags contain ");
        let container = x.next().unwrap();
        x.next()
            .unwrap()
            .split(", ")
            .filter_map(|y| {
                let mut d = y.splitn(2, ' ');
                Some((
                    d.next()?.parse::<u32>().ok()?,
                    d.next()?.rsplitn(2, ' ').nth(1)?,
                ))
            })
            .for_each(|(count, contained)| visitor(container, count, contained));
    });
}
