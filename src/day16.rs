use crate::infra::Problem;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Day16;

type Constraint<'a> = (&'a str, [u16; 4]);

impl Problem<String, String, u64, u64, 16> for Day16 {
    fn first(contents: String) -> u64 {
        let (constraints, _, tickets) = parse(&contents);

        tickets
            .into_iter()
            .flat_map(|x| x.into_iter())
            .filter(|&n| !constraints.iter().any(|c| is_valid(c, n)))
            .map(|n| n as u64)
            .sum()
    }

    fn second(contents: String) -> u64 {
        let (constraints, my_ticket, tickets) = parse(&contents);

        let valid_tickets = tickets
            .into_iter()
            .filter(|ticket| {
                ticket
                    .iter()
                    .all(|&n| constraints.iter().any(|c| is_valid(c, n)))
            })
            .collect_vec();

        let mut choices = constraints
            .iter()
            .map(|c| (c, constraint_matches(&valid_tickets, c)))
            .collect_vec();
        choices.sort_by_key(|x| x.1.len());

        find_mapping(&choices, HashMap::new())
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

fn find_mapping<'a>(
    choices: &'a [(&Constraint<'a>, Vec<usize>)],
    so_far: HashMap<usize, Constraint<'a>>,
) -> Option<HashMap<usize, Constraint<'a>>> {
    if choices.is_empty() {
        return Some(so_far);
    }

    for &i in &choices[0].1 {
        if !so_far.contains_key(&i) {
            let mut new_so_far = so_far.clone();
            new_so_far.insert(i, *choices[0].0);
            let res = find_mapping(&choices[1..], new_so_far);
            if res.is_some() {
                return res;
            }
        }
    }
    None
}

fn constraint_matches(tickets: &[Vec<u16>], constraint: &Constraint) -> Vec<usize> {
    let mut res = vec![];
    for i in 0..tickets[0].len() {
        if tickets.iter().all(|t| is_valid(constraint, t[i])) {
            res.push(i)
        }
    }
    res
}

fn is_valid(c: &Constraint, n: u16) -> bool {
    n >= c.1[0] && n <= c.1[1] || n >= c.1[2] && n <= c.1[3]
}

fn parse(contents: &str) -> (Vec<Constraint>, Vec<u16>, Vec<Vec<u16>>) {
    let mut lines = contents.lines().peekable();
    let mut constraints = vec![];
    while !lines.peek().unwrap().is_empty() {
        let (s, x) = lines
            .next()
            .unwrap()
            .splitn(2, ": ")
            .collect_tuple()
            .unwrap();
        let (a0, x) = x.splitn(2, '-').collect_tuple().unwrap();
        let (b0, x) = x.splitn(2, " or ").collect_tuple().unwrap();
        let (a1, b1) = x.splitn(2, '-').collect_tuple().unwrap();
        constraints.push((
            s,
            [
                a0.parse().unwrap(),
                b0.parse().unwrap(),
                a1.parse().unwrap(),
                b1.parse().unwrap(),
            ],
        ));
    }
    let (_, _) = lines.next_tuple().unwrap();
    let your_ticket = lines
        .next()
        .unwrap()
        .split(',')
        .map(|t| t.parse().unwrap())
        .collect();
    let (_, _) = lines.next_tuple().unwrap();
    let nearby_tickets = lines
        .map(|line| line.split(',').map(|t| t.parse().unwrap()).collect())
        .collect();

    (constraints, your_ticket, nearby_tickets)
}
