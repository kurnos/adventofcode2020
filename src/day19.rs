use crate::infra::Problem;

use itertools::Itertools;

pub struct Day19;

impl Problem<String, String, usize, usize> for Day19 {
    fn day() -> u8 {
        19
    }

    fn first(contents: String) -> usize {
        let (a, b) = contents.split("\n\n").collect_tuple().unwrap();
        let rules = a
            .lines()
            .map(parse_rule)
            .sorted_by_key(|x| x.0)
            .map(|x| x.1)
            .collect_vec();
        b.lines()
            .into_iter()
            .filter(|d| matches(d, &rules, 0) == Some(""))
            .count()
    }

    fn second(contents: String) -> usize {
        let (a, b) = contents.split("\n\n").collect_tuple().unwrap();
        let rules = a
            .lines()
            .map(parse_rule)
            .sorted_by_key(|x| x.0)
            .map(|x| x.1)
            .collect_vec();
        assert_eq!(rules[0], R::Node(vec![8, 11]));
        b.lines()
            .into_iter()
            .filter(|s| {
                let mut ss = matches_one(s, &rules, &[42, 42]);
                let mut c42 = 1;
                while let Some(s) = ss {
                    let mut x = matches(s, &rules, 31);
                    let mut c31 = 1;
                    while let Some(s) = x {
                        if s == "" {
                            return true;
                        } else if c31 == c42 {
                            break;
                        }
                        x = matches(s, &rules, 31);
                        c31 += 1;
                    }
                    c42 += 1;
                    ss = matches(s, &rules, 42);
                }
                false
            })
            .count()
    }
}

fn matches<'a>(s: &'a str, rules: &'a [R], rule: u8) -> Option<&'a str> {
    match &rules[rule as usize] {
        R::Node(ns) => matches_one(s, rules, ns),
        R::Node2(ns1, ns2) => matches_one(s, rules, ns1).or_else(|| matches_one(s, rules, ns2)),
        R::Leaf(c) => s
            .chars()
            .next()
            .and_then(|x| if x == *c { Some(&s[1..]) } else { None }),
    }
}

fn matches_one<'a>(mut s: &'a str, rules: &'a [R], ns: &[u8]) -> Option<&'a str> {
    for n in ns {
        if let Some(r) = matches(s, rules, *n) {
            s = r;
        } else {
            return None;
        }
    }
    Some(s)
}

#[derive(Debug, PartialEq, Clone)]
enum R {
    Node(Vec<u8>),
    Node2(Vec<u8>, Vec<u8>),
    Leaf(char),
}

fn parse_rule(line: &str) -> (u8, R) {
    let (n, rest) = line.split(": ").collect_tuple().unwrap();
    (
        n.parse().unwrap(),
        if rest.contains('"') {
            R::Leaf(rest.chars().nth(1).unwrap())
        } else if rest.contains('|') {
            let (a, b) = rest.split(" | ").collect_tuple().unwrap();
            R::Node2(
                a.split_whitespace().map(|n| n.parse().unwrap()).collect(),
                b.split_whitespace().map(|n| n.parse().unwrap()).collect(),
            )
        } else {
            R::Node(
                rest.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            )
        },
    )
}
