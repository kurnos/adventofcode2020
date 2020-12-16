use crate::infra::Problem;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Day16;

type Constraint<'a> = (&'a str, u16, u16, u16, u16);

impl Problem<String, String, u64, u64> for Day16 {
    fn day() -> u8 {
        16
    }

    fn first(contents: String) -> u64 {
        let (constraints, _, tickets) = parse(&contents, 20);

        let mut not_valid = 0;
        for ticket in tickets {
            for &n in &ticket {
                if constraints.iter().all(|c| !is_valid(c, n)) {
                    not_valid += n as u64;
                }
            }
        }
        not_valid
    }

    fn second(contents: String) -> u64 {
        const N: usize = 20;
        let (constraints, my_ticket, tickets) = parse(&contents, N);

        let valid_tickets = tickets
            .into_iter()
            .filter(|ticket| {
                for &n in ticket {
                    if constraints.iter().all(|c| !is_valid(c, n)) {
                        return false;
                    }
                }
                return true;
            })
            .collect_vec();

        let mut choices = constraints
            .iter()
            .map(|c| (c, fdaa(&valid_tickets, c)))
            .collect_vec();
        choices.sort_by_key(|x| x.1.len());

        asdf2(&choices, HashMap::new())
            .unwrap()
            .into_iter()
            .map(|(i, c)| {
                if c.0.starts_with("departure") {
                    my_ticket[i] as u64
                } else {
                    1
                }
            })
            .product::<u64>()
    }
}

fn asdf2<'a>(
    choices: &'a [(&Constraint<'a>, Vec<usize>)],
    so_far: HashMap<usize, Constraint<'a>>,
) -> Option<HashMap<usize, Constraint<'a>>> {
    if choices.len() == 0 {
        return Some(so_far);
    }

    for &i in &choices[0].1 {
        if !so_far.contains_key(&i) {
            let mut new_so_far = so_far.clone();
            new_so_far.insert(i, choices[0].0.clone());
            let res = asdf2(&choices[1..], new_so_far);
            if res.is_some() {
                return res;
            }
        }
    }
    return None;
}

fn fdaa(tickets: &[Vec<u16>], constraint: &Constraint) -> Vec<usize> {
    let mut res = vec![];
    for i in 0..tickets[0].len() {
        if tickets.iter().all(|t| is_valid(constraint, t[i])) {
            res.push(i)
        }
    }
    res
}

fn is_valid(c: &Constraint, n: u16) -> bool {
    n >= c.1 && n <= c.2 || n >= c.3 && n <= c.4
}

fn parse(contents: &str, n: usize) -> (Vec<Constraint>, Vec<u16>, Vec<Vec<u16>>) {
    let mut lines = contents.lines();
    let mut constraints = vec![];
    for _ in 0..n {
        let line = lines.next().unwrap();
        let mut x = line.splitn(2, ": ");
        let s = x.next().unwrap();
        let mut x = x.next().unwrap().splitn(2, "-");
        let a0 = x.next().unwrap().parse().unwrap();
        let mut x = x.next().unwrap().splitn(2, " or ");
        let b0 = x.next().unwrap().parse().unwrap();
        let mut x = x.next().unwrap().splitn(2, "-");
        let a1 = x.next().unwrap().parse().unwrap();
        let b1 = x.next().unwrap().parse().unwrap();
        constraints.push((s, a0, b0, a1, b1));
    }
    while lines.next() != Some("your ticket:") {}
    let my_ticket = lines
        .next()
        .unwrap()
        .split(',')
        .map(|d| d.parse().unwrap())
        .collect();

    while lines.next() != Some("nearby tickets:") {}
    let tickets = lines
        .map(|line| line.split(',').map(|d| d.parse().unwrap()).collect())
        .collect();
    (constraints, my_ticket, tickets)
}
